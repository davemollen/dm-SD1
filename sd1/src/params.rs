mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub drive: LinearSmooth,
  pub tone: LinearSmooth,
  pub level: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      drive: LinearSmooth::new(sample_rate, 20.),
      tone: LinearSmooth::new(sample_rate, 20.),
      level: LinearSmooth::new(sample_rate, 20.),
      is_initialized: false,
    }
  }

  pub fn set(&mut self, drive: f32, tone: f32, level: f32) {
    let drive = Self::apply_log_curve(drive);
    let tone = Self::apply_s_taper_curve(tone);
    let level = Self::apply_log_curve(level);

    if self.is_initialized {
      self.drive.set_target(drive);
      self.tone.set_target(tone);
      self.level.set_target(level);
    } else {
      self.drive.reset(drive);
      self.tone.reset(tone);
      self.level.reset(level);
      self.is_initialized = true;
    }
  }

  fn apply_s_taper_curve(input: f32) -> f32 {
    let inv_input = 1. - input;
    let squared_input = input * input;
    let squared_inv_input = inv_input * inv_input;
    (1. - squared_inv_input * squared_inv_input) * 0.5 + squared_input * squared_input * 0.5
  }

  fn apply_log_curve(input: f32) -> f32 {
    input * input * input
  }
}
