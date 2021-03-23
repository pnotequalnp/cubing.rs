use crate::core::definitions as def;
use crate::core::pruning;
use crate::core::search::{Depth, Search};
use crate::core::transition as trans;
use crate::notation::HTM;
use crate::rubiks::*;
use alloc::vec::Vec;
use core::cmp::max;
use core::convert::TryFrom;
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

    pub fn gen_to_htm(gen: usize) -> HTM {
        HTM::try_from(gen).unwrap()
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

impl From<&HTM> for Cube {
    fn from(turn: &HTM) -> Self {
        use HTM::*;

        let ix = match turn {
            U1 => 0,
            U2 => 1,
            U3 => 2,
            R1 => 3,
            R2 => 4,
            R3 => 5,
            F1 => 6,
            F2 => 7,
            F3 => 8,
            L1 => 9,
            L2 => 10,
            L3 => 11,
            D1 => 12,
            D2 => 13,
            D3 => 14,
            B1 => 15,
            B2 => 16,
            B3 => 17,
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

impl FromIterator<HTM> for Cube {
    fn from_iter<T: IntoIterator<Item = HTM>>(iter: T) -> Self {
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

impl<'a> FromIterator<&'a HTM> for Cube {
    fn from_iter<T: IntoIterator<Item = &'a HTM>>(iter: T) -> Self {
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
