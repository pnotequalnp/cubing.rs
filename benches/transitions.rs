use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cubing::core::definitions as def;
use cubing::core::transition as trans;
use once_cell::sync::Lazy;
use std::convert::TryFrom;

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

#[derive(Clone, Copy, Debug, Default)]
struct CompoundCoord(PCoord, OCoord);

impl CompoundCoord {
    const BOUND: usize = PCoord::BOUND * OCoord::BOUND;

    fn all() -> impl Iterator<Item = Self> {
        (0..Self::BOUND).map(|x| Self::try_from(x).unwrap())
    }
}

impl From<CompoundCoord> for usize {
    fn from(CompoundCoord(p_coord, o_coord): CompoundCoord) -> Self {
        usize::from(p_coord) * OCoord::BOUND + usize::from(o_coord)
    }
}

impl TryFrom<usize> for CompoundCoord {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let o = value % OCoord::BOUND;
        let o_coord = OCoord::try_from(o)?;
        let p = value / OCoord::BOUND;
        let p_coord = PCoord::try_from(p)?;
        Ok(Self(p_coord, o_coord))
    }
}

static COMPOUND_TABLE: Lazy<trans::Table<CompoundCoord, { CompoundCoord::BOUND }, 3>> =
    Lazy::new(|| {
        trans::Table::new(
            &[0, 1, 2],
            CompoundCoord::all(),
            |CompoundCoord(p_coord, o_coord), ix| {
                CompoundCoord(P_TABLE.lookup(p_coord, *ix), O_TABLE.lookup(o_coord, *ix))
            },
        )
    });

fn array8x3_transitions(c: &mut Criterion) {
    Lazy::force(&P_TABLE);
    Lazy::force(&O_TABLE);
    Lazy::force(&COMPOUND_TABLE);
    let mut group = c.benchmark_group("transitions/8x3");
    group.sample_size(500);
    group.warm_up_time(std::time::Duration::from_secs(1));

    let position = PCoord::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("with_table/permutation/gen_{:?}", ix), |b| {
            b.iter(|| P_TABLE.lookup(black_box(position), black_box(ix)))
        });
    }

    let position = OCoord::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("with_table/orientation/gen_{:?}", ix), |b| {
            b.iter(|| O_TABLE.lookup(black_box(position), black_box(ix)))
        });
    }

    let position = CompoundCoord::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("with_table/compound/gen_{:?}", ix), |b| {
            b.iter(|| COMPOUND_TABLE.lookup(black_box(position), black_box(ix)))
        });
    }

    let mut position = Array::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("without_table/mutable/gen_{:?}", ix), |b| {
            b.iter(|| position.permute_inplace(&mut GENS[ix].clone()))
        });
    }

    let position = Array::default();
    for ix in 0..GENS.len() {
        group.bench_function(format!("without_table/immutable/gen_{:?}", ix), |b| {
            b.iter(|| position.permute(&GENS[ix]))
        });
    }

    group.finish();
}

fn array8x3_table_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("transitions/table_generation/8x3");

    group.bench_function("permutation", |b| {
        b.iter(|| {
            trans::Table::<PCoord, { PCoord::BOUND }, 3>::new(
                &GENS,
                PCoord::all(),
                |coord, array| coord.permute(array),
            )
        })
    });

    group.bench_function("orientation", |b| {
        b.iter(|| {
            trans::Table::<OCoord, { OCoord::BOUND }, 3>::new(
                &GENS,
                OCoord::all(),
                |coord, array| coord.permute(array),
            )
        })
    });

    // This one takes a lot longer
    group.sample_size(10);
    group.bench_function("compound", |b| {
        b.iter(|| {
            trans::Table::<CompoundCoord, { CompoundCoord::BOUND }, 3>::new(
                &[0, 1, 2],
                CompoundCoord::all(),
                |CompoundCoord(p_coord, o_coord), ix| {
                    CompoundCoord(P_TABLE.lookup(p_coord, *ix), O_TABLE.lookup(o_coord, *ix))
                },
            )
        })
    });

    group.finish();
}

criterion_group!(benches, array8x3_transitions, array8x3_table_generation);
criterion_main!(benches);
