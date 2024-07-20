#![feature(portable_simd)]
mod clipper;
mod tone;
mod shared {
  pub mod float_ext;
  pub mod non_inverting_op_amp;
  pub mod one_pole_filter;
}
mod op_amp;
mod smooth_parameters;
use {
  clipper::Clipper, op_amp::OpAmp, shared::one_pole_filter::OnePoleFilter,
  smooth_parameters::SmoothParameters, tone::Tone,
};

pub struct SD1 {
  one_pole_filter: OnePoleFilter,
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
  smooth_parameters: SmoothParameters,
}

impl SD1 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      one_pole_filter: OnePoleFilter::new(sample_rate, 88.419412828831),
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(),
      tone: Tone::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn apply_s_taper_curve(&self, input: f32) -> f32 {
    let inv_input = 1. - input;
    let squared_input = input * input;
    let squared_inv_input = inv_input * inv_input;
    (1. - squared_inv_input * squared_inv_input) * 0.5 + squared_input * squared_input * 0.5
  }

  pub fn initialize_params(&mut self, drive: f32, tone: f32, level: f32) {
    self.smooth_parameters.initialize(drive, tone, level);
  }

  pub fn process(&mut self, input: f32, drive: f32, tone: f32, level: f32) -> f32 {
    let (drive, tone, level) = self.smooth_parameters.process(drive, tone, level);
    let highpass_output = input - self.one_pole_filter.process(input);
    let op_amp_output = self.op_amp.process(highpass_output, drive);
    let clip_output = self.clipper.process(op_amp_output) + input;
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * level
  }
}
