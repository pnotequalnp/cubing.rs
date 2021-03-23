use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cubing::core::definitions as def;
use cubing::core::transition as trans;
use once_cell::sync::Lazy;

type Array = def::Array<8, 3>;
type PCoord = def::PermutationCoord<8>;
type OCoord = def::OrientationCoord<8, 3>;

#[rustfmt::skip]
const GENS: [Array; 3] = [
    def::Array::new([(3, 0), (0, 0), (1, 0), (2, 0), (4, 0), (5, 0), (6, 0), (7, 0)]),
    def::Array::new([(4, 2), (1, 0), (2, 0), (0, 1), (7, 1), (5, 0), (6, 0), (3, 2)]),
    def::Array::new([(1, 1), (5, 2), (2, 0), (3, 0), (0, 2), (4, 1), (6, 0), (7, 0)]),
];

static P_TABLE: Lazy<trans::Table<PCoord, { PCoord::BOUND }, 3>> =
    Lazy::new(|| trans::Table::new(&GENS, PCoord::all(), |coord, array| coord.permute(array)));

static O_TABLE: Lazy<trans::Table<OCoord, { OCoord::BOUND }, 3>> =
    Lazy::new(|| trans::Table::new(&GENS, OCoord::all(), |coord, array| coord.permute(array)));

fn array8x3(c: &mut Criterion) {
    Lazy::force(&P_TABLE);
    let mut group = c.benchmark_group("8x3");
    group.sample_size(500);

    let position = PCoord::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("with table/permutation/gen {:?}", ix), |b| {
            b.iter(|| P_TABLE.lookup(black_box(position), black_box(ix)))
        });
    }

    let position = OCoord::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("with table/orientation/gen {:?}", ix), |b| {
            b.iter(|| O_TABLE.lookup(black_box(position), black_box(ix)))
        });
    }

    let mut position = Array::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("without table/mutable/gen {:?}", ix), |b| {
            b.iter(|| position.permute_inplace(&mut GENS[ix].clone()))
        });
    }

    let position = Array::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("without table/immutable/gen {:?}", ix), |b| {
            b.iter(|| position.permute(&GENS[ix]))
        });
    }

    group.finish();
}

criterion_group!(benches, array8x3);
criterion_main!(benches);
