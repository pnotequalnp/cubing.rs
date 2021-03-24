use crate::core::pruning;
use crate::core::search::Search;
use crate::core::transition as trans;
use crate::metric::htm::domino::{CornerPermutation, EdgePermutation, SlicePermutation};
use crate::metric::Domino as Metric;
use crate::puzzle::Cube3x3;
use std::cmp::max;
use std::convert::TryFrom;

pub type TransitionTable = (
    trans::Table<CornerPermutation, { CornerPermutation::BOUND }, { Metric::COUNT }>,
    trans::Table<EdgePermutation, { EdgePermutation::BOUND }, { Metric::COUNT }>,
    trans::Table<SlicePermutation, { SlicePermutation::BOUND }, { Metric::COUNT }>,
);

pub type PruningTable = (
    pruning::Table<CornerPermutation, { CornerPermutation::BOUND }>,
    pruning::Table<EdgePermutation, { EdgePermutation::BOUND }>,
    pruning::Table<SlicePermutation, { SlicePermutation::BOUND }>,
);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Domino {
    corners: CornerPermutation,
    edges: EdgePermutation,
    slice: SlicePermutation,
}

impl Domino {
    pub fn new(
        corners: CornerPermutation,
        edges: EdgePermutation,
        slice: SlicePermutation,
    ) -> Self {
        Self {
            corners,
            edges,
            slice,
        }
    }

    pub fn generate_tables() -> (TransitionTable, PruningTable) {
        let corners = trans::Table::new(
            &Metric::CORNER_MOVES,
            CornerPermutation::all(),
            |coord, gen| coord.permute(&gen.0),
        );

        let corner_pruning =
            pruning::Table::new(&Metric::GENERATORS, |coord, ix| corners.lookup(coord, *ix));

        let edges = trans::Table::new(
            &Metric::EDGE_MOVES,
            CornerPermutation::all(),
            |coord, gen| coord.permute(&gen.0),
        );

        let edge_pruning =
            pruning::Table::new(&Metric::GENERATORS, |coord, ix| edges.lookup(coord, *ix));

        let slice = trans::Table::new(
            &Metric::SLICE_MOVES,
            SlicePermutation::all(),
            |coord, gen| coord.permute(&gen),
        );

        let slice_pruning =
            pruning::Table::new(&Metric::GENERATORS, |coord, ix| slice.lookup(coord, *ix));

        (
            (corners, edges, slice),
            (corner_pruning, edge_pruning, slice_pruning),
        )
    }

    pub fn bound() {}
}

impl TryFrom<&Cube3x3> for Domino {
    type Error = ();

    fn try_from(cube: &Cube3x3) -> Result<Self, Self::Error> {
        let corners = cube.corners.truncate::<8>().map_err(|_| ())?;
        let edges = cube.edges.truncate::<8>().map_err(|_| ())?;
        let slice = cube.edges.drop::<4>().map_err(|_| ())?;

        Ok(Self::new(
            corners.p_coordinate(),
            edges.p_coordinate(),
            slice.p_coordinate(),
        ))
    }
}

impl Search for Domino {
    type Edge = usize;
    type HeuristicData = PruningTable;
    type TransitionData = TransitionTable;
    type Iter = std::vec::IntoIter<(Self, Self::Edge)>;

    fn heuristic(self, tables: &Self::HeuristicData) -> crate::core::search::Depth {
        let Self {
            corners,
            edges,
            slice,
        } = self;

        let (c_table, e_table, s_table) = tables;

        let c = c_table.lookup(corners);
        let e = e_table.lookup(edges);
        let s = s_table.lookup(slice);

        max(c, max(e, s))
    }

    fn transition(self, tables: &Self::TransitionData) -> Self::Iter {
        let Self {
            corners,
            edges,
            slice,
        } = self;

        let (c_table, e_table, s_table) = tables;

        (0..Metric::COUNT)
            .map(|ix| {
                let c = c_table.lookup(corners, ix);
                let e = e_table.lookup(edges, ix);
                let s = s_table.lookup(slice, ix);

                let position = Self::new(c, e, s);

                (position, ix)
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}
