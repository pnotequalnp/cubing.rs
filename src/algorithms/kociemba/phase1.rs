use crate::core::definitions as def;
use crate::core::pruning;
use crate::core::search::{Depth, Search};
use crate::core::transition as trans;
use crate::rubiks::*;
use alloc::vec::Vec;
use core::cmp::max;
use core::iter::FromIterator;

type Corners = def::OrientationCoord<CORNERS, TWISTS>;
type Edges = def::OrientationCoord<EDGES, FLIPS>;
type Slice = def::CombinationCoord<EDGES, BELT_EDGES>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Cube {
    corners: Corners,
    edges: Edges,
    slice: Slice,
}

impl Cube {
    pub fn new(corners: Corners, edges: Edges, slice: Slice) -> Self {
        Self {
            corners,
            edges,
            slice,
        }
    }

    pub fn create_table() -> Table {
        Table::new()
    }

    pub fn create_pruning_table(move_table: &Table) -> PruningTable {
        PruningTable::new(move_table)
    }
}

impl Search for Cube {
    type Iter = alloc::vec::IntoIter<(Self, Self::Edge)>;
    type Edge = usize;
    type HeuristicData = PruningTable;
    type TransitionData = Table;

    fn heuristic(self, table: &Self::HeuristicData) -> Depth {
        table.lookup(self)
    }

    fn transition(self, table: &Self::TransitionData) -> Self::Iter {
        (0..MOVE_COUNT)
            .map(|ix| (table.lookup(self, ix), ix))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl From<&Cube3x3> for Cube {
    fn from(cube: &Cube3x3) -> Self {
        let corners = cube.corners.o_coordinate();
        let edges = cube.edges.o_coordinate();
        let slice = cube.edges.c_coordinate();

        Self::new(corners, edges, slice)
    }
}

impl From<&FaceTurn> for Cube {
    fn from(turn: &FaceTurn) -> Self {
        let ix = match turn {
            FaceTurn::U => 0,
            FaceTurn::U2 => 1,
            FaceTurn::U3 => 2,
            FaceTurn::R => 3,
            FaceTurn::R2 => 4,
            FaceTurn::R3 => 5,
            FaceTurn::F => 6,
            FaceTurn::F2 => 7,
            FaceTurn::F3 => 8,
            FaceTurn::L => 9,
            FaceTurn::L2 => 10,
            FaceTurn::L3 => 11,
            FaceTurn::D => 12,
            FaceTurn::D2 => 13,
            FaceTurn::D3 => 14,
            FaceTurn::B => 15,
            FaceTurn::B2 => 16,
            FaceTurn::B3 => 17,
        };

        let corners = CORNER_MOVES[ix].o_coordinate();
        let edges = EDGE_MOVES[ix].o_coordinate();
        let slice = EDGE_MOVES[ix].c_coordinate();

        Cube {
            corners,
            edges,
            slice,
        }
    }
}

impl FromIterator<FaceTurn> for Cube {
    fn from_iter<T: IntoIterator<Item = FaceTurn>>(iter: T) -> Self {
        let (corners, edges) = iter
            .into_iter()
            .map(usize::from)
            .map(|ix| (&CORNER_MOVES[ix], &EDGE_MOVES[ix]))
            .fold(
                (def::Array::default(), def::Array::default()),
                |(w, x), (y, z)| (w.permute(&y), x.permute(&z)),
            );

        Self::new(
            corners.o_coordinate(),
            edges.o_coordinate(),
            edges.c_coordinate(),
        )
    }
}

impl<'a> FromIterator<&'a FaceTurn> for Cube {
    fn from_iter<T: IntoIterator<Item = &'a FaceTurn>>(iter: T) -> Self {
        let (corners, edges) = iter
            .into_iter()
            .map(|x| usize::from(*x))
            .map(|ix| (&CORNER_MOVES[ix], &EDGE_MOVES[ix]))
            .fold(
                (def::Array::default(), def::Array::default()),
                |(w, x), (y, z)| (w.permute(&y), x.permute(&z)),
            );

        Self::new(
            corners.o_coordinate(),
            edges.o_coordinate(),
            edges.c_coordinate(),
        )
    }
}

pub struct Table(
    trans::Table<Corners, { Corners::BOUND }, MOVE_COUNT>,
    trans::Table<Edges, { Edges::BOUND }, MOVE_COUNT>,
    trans::Table<Slice, { Slice::BOUND }, MOVE_COUNT>,
);

impl Table {
    pub fn new() -> Self {
        Self(
            trans::Table::new(&CORNER_MOVES, Corners::all(), Corners::permute),
            trans::Table::new(&EDGE_MOVES, Edges::all(), Edges::permute),
            trans::Table::new(&EDGE_MOVES, Slice::all(), Slice::permute),
        )
    }

    pub fn lookup(
        &self,
        Cube {
            corners,
            edges,
            slice,
        }: Cube,
        index: usize,
    ) -> Cube {
        let Self(c_table, e_table, s_table) = self;

        let corners = c_table.lookup(corners, index);
        let edges = e_table.lookup(edges, index);
        let slice = s_table.lookup(slice, index);

        Cube {
            corners,
            edges,
            slice,
        }
    }
}

pub struct PruningTable(
    pruning::Table<Corners, { Corners::BOUND }>,
    pruning::Table<Edges, { Edges::BOUND }>,
    pruning::Table<Slice, { Slice::BOUND }>,
);

impl PruningTable {
    pub fn new(Table(c_table, e_table, s_table): &Table) -> Self {
        Self(
            pruning::Table::new(&GENERATORS, |coord, gen| c_table.lookup(coord, *gen)),
            pruning::Table::new(&GENERATORS, |coord, gen| e_table.lookup(coord, *gen)),
            pruning::Table::new(&GENERATORS, |coord, gen| s_table.lookup(coord, *gen)),
        )
    }

    pub fn lookup(
        &self,
        Cube {
            corners,
            edges,
            slice,
        }: Cube,
    ) -> Depth {
        let PruningTable(c_table, e_table, s_table) = self;

        let corners = c_table.lookup(corners);
        let edges = e_table.lookup(edges);
        let slice = s_table.lookup(slice);

        max(max(corners, edges), slice)
    }
}

#[cfg(test)]
mod tests {
    use core::convert::TryFrom;

    use super::*;

    #[test]
    pub fn array_cancellation() {
        for ix in (0..18).step_by(3) {
            let x1 = &CORNER_MOVES[ix];
            let x2 = &CORNER_MOVES[ix + 1];
            let x3 = &CORNER_MOVES[ix + 2];
            assert_eq!(
                def::Array::default(),
                x1.permute(x3),
                "{:?}: {:?} {:?}",
                FaceTurn::from(ix),
                x1,
                x3
            );
            assert_eq!(def::Array::default(), x3.permute(x1));
            assert_eq!(def::Array::default(), x2.permute(x2));
            assert_eq!(
                def::Array::default(),
                x1.permute(x1).permute(x1).permute(x1)
            );
            assert_eq!(
                def::Array::default(),
                x3.permute(x3).permute(x3).permute(x3)
            );
            assert_eq!(def::Array::default(), x2.permute(x1).permute(x1));
            assert_eq!(def::Array::default(), x2.permute(x3).permute(x3));

            let x1 = &EDGE_MOVES[ix];
            let x2 = &EDGE_MOVES[ix + 1];
            let x3 = &EDGE_MOVES[ix + 2];
            assert_eq!(
                def::Array::default(),
                x1.permute(x3),
                "{:?}: {:?} {:?}",
                FaceTurn::from(ix),
                x1,
                x3
            );
            assert_eq!(def::Array::default(), x3.permute(x1));
            assert_eq!(def::Array::default(), x2.permute(x2));
            assert_eq!(
                def::Array::default(),
                x1.permute(x1).permute(x1).permute(x1)
            );
            assert_eq!(
                def::Array::default(),
                x3.permute(x3).permute(x3).permute(x3)
            );
            assert_eq!(def::Array::default(), x2.permute(x1).permute(x1));
            assert_eq!(def::Array::default(), x2.permute(x3).permute(x3));
        }
    }

    #[test]
    pub fn zero_orientations() {
        for ix in (1..18).step_by(3) {
            let co = CORNER_MOVES[ix].o_coordinate();
            assert_eq!(
                def::OrientationCoord::default(),
                co,
                "{:?}",
                FaceTurn::from(ix)
            );

            let eo = EDGE_MOVES[ix].o_coordinate();
            assert_eq!(
                def::OrientationCoord::default(),
                eo,
                "{:?}: {:?}",
                FaceTurn::from(ix),
                EDGE_MOVES[ix]
            );
        }

        for ix in 0..18 {
            let array = &CORNER_MOVES[ix];
            assert_eq!(
                def::OrientationCoord::default(),
                array.permute(array).o_coordinate()
            );

            let array = &EDGE_MOVES[ix];
            assert_eq!(
                def::OrientationCoord::default(),
                array.permute(array).o_coordinate()
            );
        }
    }

    #[test]
    pub fn zero_combinations() {
        assert_eq!(
            def::CombinationCoord::try_from(0).unwrap(),
            def::Array::<EDGES, FLIPS>::new([
                (8, 0),
                (9, 0),
                (10, 0),
                (11, 0),
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0)
            ])
            .c_coordinate::<BELT_EDGES>()
        );
    }

    #[test]
    pub fn max_combinations() {
        for ix in (1..18).step_by(3) {
            let cm = EDGE_MOVES[ix].c_coordinate::<BELT_EDGES>();
            assert_eq!(
                def::CombinationCoord::default(),
                cm,
                "{:?}: {:?}",
                FaceTurn::from(ix),
                EDGE_MOVES[ix]
            );
        }

        for ix in [0, 2, 12, 14].iter() {
            let array = &EDGE_MOVES[*ix];
            assert_eq!(
                def::CombinationCoord::default(),
                array.c_coordinate::<BELT_EDGES>(),
            );
        }

        for ix in [3, 5, 6, 8, 9, 11, 15, 17].iter() {
            let array = &EDGE_MOVES[*ix];
            assert_ne!(
                def::CombinationCoord::default(),
                array.c_coordinate::<BELT_EDGES>(),
                "{}: {:?}",
                FaceTurn::from(*ix),
                array.c_coordinate::<BELT_EDGES>()
            );
        }
    }

    #[test]
    pub fn max_orientations() {
        let array = def::Array::<8, 3>::create([
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 1),
        ])
        .unwrap();
        let coord = def::OrientationCoord::try_from(2186).unwrap();
        assert_eq!(coord, array.o_coordinate());
        assert_eq!(array, coord.array());

        let array = def::Array::<12, 2>::create([
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 1),
            (11, 1),
        ])
        .unwrap();
        let coord = def::OrientationCoord::try_from(2047).unwrap();
        assert_eq!(coord, array.o_coordinate());
        assert_eq!(array, coord.array());
    }
}
