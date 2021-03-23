use criterion::{criterion_group, criterion_main, Criterion};

pub fn random_state_generation(c: &mut Criterion) {
    c.bench_function("Cube3x3: random state generation", |b| {
        b.iter(|| cubing::rubiks::Cube3x3::random_state())
    });
}

criterion_group!(benches, random_state_generation,);
criterion_main!(benches);
