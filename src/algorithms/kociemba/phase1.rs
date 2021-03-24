use crate::core::definitions as def;
use crate::core::pruning;
use crate::core::search::{Depth, Search};
use crate::core::transition as trans;
use crate::metric::{htm, Htm};
use crate::puzzle::*;
use std::cmp::max;
use std::convert::TryFrom;
use std::iter::FromIterator;

type Corners = def::OrientationCoord<8, 3>;
type Edges = def::OrientationCoord<12, 2>;
type Slice = def::CombinationCoord<12, 4>;

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

    pub fn gen_to_htm(gen: usize) -> Htm {
        Htm::try_from(gen).unwrap()
    }
}

impl Search for Cube {
    type Iter = std::vec::IntoIter<(Self, Self::Edge)>;
    type Edge = usize;
    type HeuristicData = PruningTable;
    type TransitionData = Table;

    fn heuristic(self, table: &Self::HeuristicData) -> Depth {
        table.lookup(self)
    }

    fn transition(self, table: &Self::TransitionData) -> Self::Iter {
        (0..Htm::COUNT)
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

impl From<Htm> for Cube {
    fn from(htm: Htm) -> Self {
        let corners: &htm::Corners = htm.into();
        let edges: &htm::Edges = htm.into();

        Self::new(
            corners.o_coordinate(),
            edges.o_coordinate(),
            edges.c_coordinate(),
        )
    }
}

impl FromIterator<Htm> for Cube {
    fn from_iter<T: IntoIterator<Item = Htm>>(iter: T) -> Self {
        let (corners, edges) = iter
            .into_iter()
            .map(|htm| -> (&htm::Corners, &htm::Edges) { (htm.into(), htm.into()) })
            .fold(
                (htm::Corners::default(), htm::Edges::default()),
                |(w, x), (y, z)| (w.permute(&y), x.permute(&z)),
            );

        Self::new(
            corners.o_coordinate(),
            edges.o_coordinate(),
            edges.c_coordinate(),
        )
    }
}

impl<'a> FromIterator<&'a Htm> for Cube {
    fn from_iter<T: IntoIterator<Item = &'a Htm>>(iter: T) -> Self {
        let (corners, edges) = iter
            .into_iter()
            .map(|htm| -> (&htm::Corners, &htm::Edges) { ((*htm).into(), (*htm).into()) })
            .fold(
                (htm::Corners::default(), htm::Edges::default()),
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
    trans::Table<Corners, { Corners::BOUND }, { Htm::COUNT }>,
    trans::Table<Edges, { Edges::BOUND }, { Htm::COUNT }>,
    trans::Table<Slice, { Slice::BOUND }, { Htm::COUNT }>,
);

impl Table {
    pub fn new() -> Self {
        Self(
            trans::Table::new(&Htm::CORNER_MOVES, Corners::all(), Corners::permute),
            trans::Table::new(&Htm::EDGE_MOVES, Edges::all(), Edges::permute),
            trans::Table::new(&Htm::EDGE_MOVES, Slice::all(), Slice::permute),
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
            pruning::Table::new(&Htm::GENERATORS, |coord, gen| c_table.lookup(coord, *gen)),
            pruning::Table::new(&Htm::GENERATORS, |coord, gen| e_table.lookup(coord, *gen)),
            pruning::Table::new(&Htm::GENERATORS, |coord, gen| s_table.lookup(coord, *gen)),
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
