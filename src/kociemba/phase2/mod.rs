mod moves;

use crate::rubiks::FaceTurn;
use rubiks_rs::definitions as def;
use rubiks_rs::pruning;
use rubiks_rs::search::Depth;
use rubiks_rs::search::Search;
use rubiks_rs::transition as trans;
use rubiks_rs::util::count;
use moves::*;
use std::cmp::max;
use std::convert::TryFrom;
use std::iter::FromIterator;

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
    type Edge = usize;
    type HeuristicData = PruningTable;
    type TransitionData = Table;

    fn heuristic(self, table: &Self::HeuristicData) -> Depth {
        table.lookup(self)
    }

    fn transition(self, table: &Self::TransitionData) -> Vec<(Self, Self::Edge)> {
        (0..MOVE_COUNT)
            .map(|ix| (table.lookup(self, ix), ix))
            .collect()
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
                (def::Array::default(), def::Array::default(), def::Array::default()),
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

// #[cfg(test)]
// mod tests {
//     use std::convert::TryFrom;

//     use super::*;

//     #[test]
//     pub fn array_cancellation() {
//         for ix in (0..18).step_by(3) {
//             let x1 = &CORNER_MOVES[ix];
//             let x2 = &CORNER_MOVES[ix + 1];
//             let x3 = &CORNER_MOVES[ix + 2];
//             assert_eq!(
//                 def::Array::default(),
//                 x1.permute(x3),
//                 "{:?}: {:?} {:?}",
//                 FaceTurn::from(ix),
//                 x1,
//                 x3
//             );
//             assert_eq!(def::Array::default(), x3.permute(x1));
//             assert_eq!(def::Array::default(), x2.permute(x2));
//             assert_eq!(
//                 def::Array::default(),
//                 x1.permute(x1).permute(x1).permute(x1)
//             );
//             assert_eq!(
//                 def::Array::default(),
//                 x3.permute(x3).permute(x3).permute(x3)
//             );
//             assert_eq!(def::Array::default(), x2.permute(x1).permute(x1));
//             assert_eq!(def::Array::default(), x2.permute(x3).permute(x3));

//             let x1 = &EDGE_MOVES[ix];
//             let x2 = &EDGE_MOVES[ix + 1];
//             let x3 = &EDGE_MOVES[ix + 2];
//             assert_eq!(
//                 def::Array::default(),
//                 x1.permute(x3),
//                 "{:?}: {:?} {:?}",
//                 FaceTurn::from(ix),
//                 x1,
//                 x3
//             );
//             assert_eq!(def::Array::default(), x3.permute(x1));
//             assert_eq!(def::Array::default(), x2.permute(x2));
//             assert_eq!(
//                 def::Array::default(),
//                 x1.permute(x1).permute(x1).permute(x1)
//             );
//             assert_eq!(
//                 def::Array::default(),
//                 x3.permute(x3).permute(x3).permute(x3)
//             );
//             assert_eq!(def::Array::default(), x2.permute(x1).permute(x1));
//             assert_eq!(def::Array::default(), x2.permute(x3).permute(x3));
//         }
//     }

//     #[test]
//     pub fn zero_orientations() {
//         for ix in (1..18).step_by(3) {
//             let co = CORNER_MOVES[ix].o_coordinate();
//             assert_eq!(
//                 def::OrientationCoord::default(),
//                 co,
//                 "{:?}",
//                 FaceTurn::from(ix)
//             );

//             let eo = EDGE_MOVES[ix].o_coordinate();
//             assert_eq!(
//                 def::OrientationCoord::default(),
//                 eo,
//                 "{:?}: {:?}",
//                 FaceTurn::from(ix),
//                 EDGE_MOVES[ix]
//             );
//         }

//         for ix in 0..18 {
//             let array = &CORNER_MOVES[ix];
//             assert_eq!(
//                 def::OrientationCoord::default(),
//                 array.permute(array).o_coordinate()
//             );

//             let array = &EDGE_MOVES[ix];
//             assert_eq!(
//                 def::OrientationCoord::default(),
//                 array.permute(array).o_coordinate()
//             );
//         }
//     }

//     #[test]
//     pub fn zero_combinations() {
//         assert_eq!(
//             def::CombinationCoord::try_from(0).unwrap(),
//             def::Array::<EDGES, FLIPS>::new([
//                 (8, 0),
//                 (9, 0),
//                 (10, 0),
//                 (11, 0),
//                 (0, 0),
//                 (1, 0),
//                 (2, 0),
//                 (3, 0),
//                 (4, 0),
//                 (5, 0),
//                 (6, 0),
//                 (7, 0)
//             ])
//             .c_coordinate::<BELT_EDGES>()
//         );
//     }

//     #[test]
//     pub fn max_combinations() {
//         for ix in (1..18).step_by(3) {
//             let cm = EDGE_MOVES[ix].c_coordinate::<BELT_EDGES>();
//             assert_eq!(
//                 def::CombinationCoord::default(),
//                 cm,
//                 "{:?}: {:?}",
//                 FaceTurn::from(ix),
//                 EDGE_MOVES[ix]
//             );
//         }

//         for ix in [0, 2, 12, 14].iter() {
//             let array = &EDGE_MOVES[*ix];
//             assert_eq!(
//                 def::CombinationCoord::default(),
//                 array.c_coordinate::<BELT_EDGES>(),
//             );
//         }

//         for ix in [3, 5, 6, 8, 9, 11, 15, 17].iter() {
//             let array = &EDGE_MOVES[*ix];
//             assert_ne!(
//                 def::CombinationCoord::default(),
//                 array.c_coordinate::<BELT_EDGES>(),
//                 "{}: {:?}",
//                 FaceTurn::from(*ix),
//                 array.c_coordinate::<BELT_EDGES>()
//             );
//         }
//     }

//     #[test]
//     pub fn max_orientations() {
//         let array = def::Array::<8, 3>::create([
//             (0, 2),
//             (1, 2),
//             (2, 2),
//             (3, 2),
//             (4, 2),
//             (5, 2),
//             (6, 2),
//             (7, 1),
//         ])
//         .unwrap();
//         let coord = def::OrientationCoord::try_from(2186).unwrap();
//         assert_eq!(coord, array.o_coordinate());
//         assert_eq!(array, coord.array());

//         let array = def::Array::<12, 2>::create([
//             (0, 1),
//             (1, 1),
//             (2, 1),
//             (3, 1),
//             (4, 1),
//             (5, 1),
//             (6, 1),
//             (7, 1),
//             (8, 1),
//             (9, 1),
//             (10, 1),
//             (11, 1),
//         ])
//         .unwrap();
//         let coord = def::OrientationCoord::try_from(2047).unwrap();
//         assert_eq!(coord, array.o_coordinate());
//         assert_eq!(array, coord.array());
//     }
// }
