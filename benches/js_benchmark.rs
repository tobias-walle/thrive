use criterion::{criterion_group, criterion_main, Criterion};
use thrive::js;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("js hello world", |b| {
        b.iter(|| js::exec(r#"console.log("Hello World")"#))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
