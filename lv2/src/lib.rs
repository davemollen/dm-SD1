extern crate lv2;
extern crate sd1;
use lv2::prelude::*;
use sd1::SD1;

#[derive(PortCollection)]
struct Ports {
  drive: InputPort<Control>,
  tone: InputPort<Control>,
  level: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-SD1")]
struct DmSD1 {
  sd1: SD1,
  is_active: bool,
}

impl Plugin for DmSD1 {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      sd1: SD1::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let drive = *ports.drive;
    let tone = self.sd1.apply_s_taper_curve(*ports.tone);
    let level = *ports.level * 0.5;

    if !self.is_active {
      self.sd1.initialize_params(drive, tone, level);
      self.is_active = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.sd1.process(*input, drive, tone, level);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmSD1);
