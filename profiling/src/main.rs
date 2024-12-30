mod utils;
use sd1::{Params, SD1};
use utils::generate_signal;

fn main() {
  let mut sd1 = SD1::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0.5);

  loop {
    let input = generate_signal();
    sd1.process(input, &mut params);
  }
}
