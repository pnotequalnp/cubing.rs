use criterion::{criterion_group, criterion_main, Criterion};
use cubing::algorithms::kociemba;
use cubing::core::search;
use once_cell::sync::Lazy;

static TABLES: Lazy<kociemba::Tables> = Lazy::new(|| kociemba::generate_tables());

pub fn phase_1_move_table(c: &mut Criterion) {
    c.bench_function("Kociemba: phase 1 move table", |b| {
        b.iter(|| kociemba::phase1::Table::new())
    });
}

pub fn phase_2_move_table(c: &mut Criterion) {
    c.bench_function("Kociemba: phase 2 move table", |b| {
        b.iter(|| kociemba::phase2::Table::new())
    });
}

pub fn phase_1_pruning_table(c: &mut Criterion) {
    Lazy::force(&TABLES);
    c.bench_function("Kociemba: phase 1 pruning table", |b| {
        b.iter(|| kociemba::phase1::PruningTable::new(&TABLES.0))
    });
}

pub fn phase_2_pruning_table(c: &mut Criterion) {
    Lazy::force(&TABLES);
    c.bench_function("Kociemba: phase 2 pruning table", |b| {
        b.iter(|| kociemba::phase2::PruningTable::new(&TABLES.2))
    });
}

pub fn super_flip_phase_1(c: &mut Criterion) {
    Lazy::force(&TABLES);
    let super_flip = kociemba::Phase1::from(&cubing::rubiks::positions::SUPER_FLIP);

    c.bench_function("Kociemba: super flip phase 1", |b| {
        b.iter(|| search::ida_iter(super_flip, &TABLES.1, &TABLES.0, None))
    });
}

pub fn super_flip_full(c: &mut Criterion) {
    Lazy::force(&TABLES);
    let mut group = c.benchmark_group("Kociemba: super flip");
    let super_flip = cubing::rubiks::positions::SUPER_FLIP;

    for max_length in [None, Some(23), Some(22)].iter() {
        group.bench_with_input(
            max_length
                .map(|x| x.to_string())
                .unwrap_or("None".to_string()),
            max_length,
            |b, &max_length| {
                b.iter(|| super_flip.kociemba(&TABLES, max_length));
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    phase_1_move_table,
    phase_2_move_table,
    phase_1_pruning_table,
    phase_2_pruning_table,
    super_flip_phase_1,
    super_flip_full,
);
criterion_main!(benches);
