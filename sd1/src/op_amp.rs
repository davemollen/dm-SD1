use crate::shared::{bilinear_transform::BilinearTransform, biquad_filter::BiquadFilter};

const R1: f32 = 4700.;
const R2: f32 = 1000000.;
const R3: f32 = 33000.;
const C1: f32 = 4.7e-8;

pub struct OpAmp {
  op_amp: BiquadFilter,
  bilinear_transform: BilinearTransform,
}

impl OpAmp {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      op_amp: BiquadFilter::new(),
      bilinear_transform: BilinearTransform::new(sample_rate),
    }
  }

  pub fn reset(&mut self) {
    self.op_amp.reset();
  }

  pub fn process(&mut self, input: f32, drive: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients(drive);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.op_amp.process(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(&self, drive: f32) -> ([f32; 3], [f32; 3]) {
    let r2 = drive * R2 + R3;
    let a1 = R1 * C1;
    let b1 = r2 * C1 + a1;

    ([0., b1, 1.], [0., a1, 1.])
  }
}
