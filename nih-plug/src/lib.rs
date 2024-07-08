use nih_plug::prelude::*;
use sd1::SD1;
use std::sync::Arc;
mod sd1_parameters;
use sd1_parameters::SD1Parameters;
mod editor;

struct DmSD1 {
  params: Arc<SD1Parameters>,
  sd1: SD1,
}

impl DmSD1 {
  pub fn get_params(&self) -> (f32, f32, f32) {
    let drive = self.params.drive.value();
    let tone = self.params.tone.value();
    let level = self.params.level.value() * 0.5;

    (drive, self.sd1.apply_s_taper_curve(tone), level)
  }
}

impl Default for DmSD1 {
  fn default() -> Self {
    let params = Arc::new(SD1Parameters::default());
    Self {
      params: params.clone(),
      sd1: SD1::new(44100.),
    }
  }
}

impl Plugin for DmSD1 {
  const NAME: &'static str = "dm-SD1";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-SD1";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.sd1 = SD1::new(buffer_config.sample_rate);
    let (drive, tone, level) = self.get_params();
    self.sd1.initialize_params(drive, tone, level);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let (drive, tone, level) = self.get_params();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let sample = channel_samples.iter_mut().next().unwrap();
      *sample = self.sd1.process(*sample, drive, tone, level);
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmSD1 {
  const CLAP_ID: &'static str = "dm-SD1";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("An overdrive plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::Utility,
    ClapFeature::Distortion,
  ];
}

impl Vst3Plugin for DmSD1 {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-SD1..........";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Mono,
    Vst3SubCategory::Distortion,
  ];
}

nih_export_clap!(DmSD1);
nih_export_vst3!(DmSD1);
