#![feature(iter_advance_by)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cubing::algorithms::kociemba;
use cubing::core::search;
use once_cell::sync::Lazy;

static TABLES: Lazy<kociemba::Tables> = Lazy::new(|| kociemba::generate_tables());

pub fn table_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("kociemba/table_generation");

    group.bench_function("transition/phase_1", |b| {
        b.iter(|| kociemba::phase1::Table::new())
    });

    group.bench_function("transition/phase_2", |b| {
        b.iter(|| kociemba::phase2::Table::new())
    });

    Lazy::force(&TABLES);

    group.bench_function("pruning/phase_1", |b| {
        b.iter(|| kociemba::phase1::PruningTable::new(&TABLES.0))
    });

    group.bench_function("pruning/phase 2", |b| {
        b.iter(|| kociemba::phase2::PruningTable::new(&TABLES.2))
    });
}

pub fn transitions(c: &mut Criterion) {
    Lazy::force(&TABLES);
    let mut group = c.benchmark_group("kociemba/transitions");
    group.sample_size(500);
    group.warm_up_time(std::time::Duration::from_secs(1));

    let position = kociemba::Phase1::default();
    let table = &TABLES.0;
    for ix in 0..18 {
        group.bench_function(format!("phase_1/gen_{:?}", ix), |b| {
            b.iter(|| table.lookup(black_box(position), black_box(ix)))
        });
    }

    group.finish();
}

pub fn superflip(c: &mut Criterion) {
    Lazy::force(&TABLES);
    let mut group = c.benchmark_group("kociemba/superflip");
    group.sample_size(50);
    let superflip = cubing::rubiks::positions::SUPER_FLIP;

    let position = kociemba::Phase1::from(&superflip);
    group.bench_function("phase_1/1_solution", |b| {
        b.iter(|| search::ida_iter(black_box(position), &TABLES.1, &TABLES.0, None).next())
    });

    group.bench_function("phase_1/10_solutions", |b| {
        b.iter(|| search::ida_iter(black_box(position), &TABLES.1, &TABLES.0, None).advance_by(10))
    });

    group.bench_function("phase_1/100_solutions", |b| {
        b.iter(|| search::ida_iter(black_box(position), &TABLES.1, &TABLES.0, None).advance_by(100))
    });

    group.bench_function("phase_1/1000_solutions", |b| {
        b.iter(|| {
            search::ida_iter(black_box(position), &TABLES.1, &TABLES.0, None).advance_by(1000)
        })
    });

    let position = superflip;
    for max_length in [None, Some(23), Some(22)].iter() {
        let max = max_length
            .map(|x| x.to_string())
            .unwrap_or("no".to_string());
        group.bench_with_input(format!("full/{}_max", max), max_length, |b, &max_length| {
            b.iter(|| position.kociemba(&TABLES, max_length));
        });
    }

    group.finish();
}

criterion_group!(benches, table_generation, transitions, superflip,);
criterion_main!(benches);
