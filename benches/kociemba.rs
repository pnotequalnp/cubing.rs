use criterion::{criterion_group, criterion_main, Criterion};
use once_cell::sync::Lazy;
use cubing::kociemba::*;
use cubing::search;

static MOVE_TABLE_1: Lazy<phase1::Table> = Lazy::new(|| phase1::Table::new());
static MOVE_TABLE_2: Lazy<phase2::Table> = Lazy::new(|| phase2::Table::new());
static PRUNING_TABLE_1: Lazy<phase1::PruningTable> =
    Lazy::new(|| phase1::PruningTable::new(&MOVE_TABLE_1));
static PRUNING_TABLE_2: Lazy<phase2::PruningTable> =
    Lazy::new(|| phase2::PruningTable::new(&MOVE_TABLE_2));

pub fn phase_1_move_table(c: &mut Criterion) {
    c.bench_function("Kociemba: phase 1 move table", |b| {
        b.iter(|| Phase1::create_table())
    });
}

pub fn phase_2_move_table(c: &mut Criterion) {
    c.bench_function("Kociemba: phase 2 move table", |b| {
        b.iter(|| Phase2::create_table())
    });
}

pub fn phase_1_pruning_table(c: &mut Criterion) {
    c.bench_function("Kociemba: phase 1 pruning table", |b| {
        b.iter(|| Phase1::create_pruning_table(&MOVE_TABLE_1))
    });
}

pub fn phase_2_pruning_table(c: &mut Criterion) {
    c.bench_function("Kociemba: phase 2 pruning table", |b| {
        b.iter(|| Phase2::create_pruning_table(&MOVE_TABLE_2))
    });
}

pub fn super_flip_phase_1(c: &mut Criterion) {
    let super_flip = Phase1::from(&cubing::rubiks::positions::SUPER_FLIP);

    c.bench_function("Kociemba: super flip phase 1", |b| {
        b.iter(|| search::ida_iter(super_flip, &PRUNING_TABLE_1, &MOVE_TABLE_1, None))
    });
}

pub fn super_flip_full(c: &mut Criterion) {
    let mut group = c.benchmark_group("Kociemba: super flip");
    let super_flip = cubing::rubiks::positions::SUPER_FLIP;

    for max_length in [None, Some(23), Some(22)].iter() {
        group.bench_with_input(
            max_length
                .map(|x| x.to_string())
                .unwrap_or("None".to_string()),
            max_length,
            |b, &max_length| {
                b.iter(|| {
                    solve(
                        &super_flip,
                        &MOVE_TABLE_1,
                        &MOVE_TABLE_2,
                        &PRUNING_TABLE_1,
                        &PRUNING_TABLE_2,
                        max_length,
                    )
                });
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
