mod moves;

use crate::core::definitions as def;
use crate::core::pruning;
use crate::core::search::{Depth, Search};
use crate::core::transition as trans;
use crate::rubiks::{Cube3x3, FaceTurn};
use crate::util::count;
use alloc::vec::Vec;
use core::cmp::max;
use core::convert::TryFrom;
use core::iter::FromIterator;
use moves::*;

type Corners = def::PermutationCoord<CORNERS>;
type Edges = def::PermutationCoord<EDGES>;
type Slice = def::PermutationCoord<SLICE_EDGES>;

const CORNERS: usize = 8;
const EDGES: usize = 8;
const SLICE_EDGES: usize = 4;
const MOVE_COUNT: usize = 10;
const GENERATORS: [usize; MOVE_COUNT] = count::<MOVE_COUNT>();

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

    pub fn face_turn(value: usize) -> FaceTurn {
        match value {
            0 => FaceTurn::U,
            1 => FaceTurn::U2,
            2 => FaceTurn::U3,
            3 => FaceTurn::D,
            4 => FaceTurn::D2,
            5 => FaceTurn::D3,
            6 => FaceTurn::R2,
            7 => FaceTurn::F2,
            8 => FaceTurn::L2,
            9 => FaceTurn::B2,
            _ => panic!("Not a phase 2 generator"),
        }
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

impl TryFrom<&Cube3x3> for Cube {
    type Error = def::CreationError;

    fn try_from(cube: &Cube3x3) -> Result<Self, Self::Error> {
        let corners = cube.corners.truncate::<8>()?;
        let edges = cube.edges.truncate::<8>()?;
        let slice = cube.edges.drop::<4>()?;

        Ok(Self::new(corners.p_coordinate(), edges.p_coordinate(), slice.p_coordinate()))
    }
}

impl TryFrom<FaceTurn> for Cube {
    type Error = ();

    fn try_from(turn: FaceTurn) -> Result<Self, Self::Error> {
        let ix = match turn {
            FaceTurn::U => Ok(0),
            FaceTurn::U2 => Ok(1),
            FaceTurn::U3 => Ok(2),
            FaceTurn::D => Ok(3),
            FaceTurn::D2 => Ok(4),
            FaceTurn::D3 => Ok(5),
            FaceTurn::R2 => Ok(6),
            FaceTurn::F2 => Ok(7),
            FaceTurn::L2 => Ok(8),
            FaceTurn::B2 => Ok(9),
            _ => Err(()),
        }?;

        let corners = CORNER_MOVES[ix].p_coordinate();
        let edges = EDGE_MOVES[ix].p_coordinate();
        let slice = SLICE_MOVES[ix].p_coordinate();

        Ok(Cube {
            corners,
            edges,
            slice,
        })
    }
}

impl FromIterator<FaceTurn> for Cube {
    fn from_iter<T: IntoIterator<Item = FaceTurn>>(iter: T) -> Self {
        let (corners, edges, slice) = iter
            .into_iter()
            .map(usize::try_from)
            .map(|x| x.expect("Invalid move for Kociemba phase 2."))
            .map(|ix| (&CORNER_MOVES[ix], &EDGE_MOVES[ix], &SLICE_MOVES[ix]))
            .fold(
                (
                    def::Array::default(),
                    def::Array::default(),
                    def::Array::default(),
                ),
                |(u, v, w), (x, y, z)| (u.permute(&x), v.permute(&y), w.permute(&z)),
            );

        Self::new(
            corners.p_coordinate(),
            edges.p_coordinate(),
            slice.p_coordinate(),
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
            trans::Table::new(&SLICE_MOVES, Slice::all(), Slice::permute),
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
