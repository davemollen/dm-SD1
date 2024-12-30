#![feature(portable_simd)]
mod clipper;
mod params;
mod tone;
mod shared {
  pub mod float_ext;
  pub mod non_inverting_op_amp;
  pub mod one_pole_filter;
}
mod op_amp;
pub use params::Params;
use {
  clipper::Clipper, op_amp::OpAmp, params::Smoother, shared::one_pole_filter::OnePoleFilter,
  tone::Tone,
};

pub struct SD1 {
  one_pole_filter: OnePoleFilter,
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
}

impl SD1 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      one_pole_filter: OnePoleFilter::new(sample_rate, 88.419412828831),
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(),
      tone: Tone::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let drive = params.drive.next();
    let tone = params.tone.next();
    let level = params.level.next();

    let highpass_output = input - self.one_pole_filter.process(input);
    let op_amp_output = self.op_amp.process(highpass_output, drive);
    let clip_output = self.clipper.process(op_amp_output) + input;
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * level
  }
}
