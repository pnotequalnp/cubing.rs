use criterion::{criterion_group, criterion_main, Criterion};

pub fn random_state_generation(c: &mut Criterion) {
    c.bench_function("cube3x3/random_state_generation", |b| {
        b.iter(|| cubing::puzzle::Cube3x3::random_state())
    });
}

criterion_group!(benches, random_state_generation,);
criterion_main!(benches);
