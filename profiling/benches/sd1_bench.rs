#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use sd1::{Params, SD1};
use utils::generate_signal_stream;

fn sd1_bench(c: &mut Criterion) {
  let mut sd1 = SD1::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0.5);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("sd1", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        sd1.process(*signal, &mut params);
      }
    })
  });
}

criterion_group!(benches, sd1_bench);
criterion_main!(benches);
