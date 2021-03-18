use crate::core::definitions as def;
use crate::rubiks::*;
use core::iter::FromIterator;

type CornerPermutation = def::PermutationCoord<CORNERS>;
type CornerOrientation = def::OrientationCoord<CORNERS, TWISTS>;
type EdgePermutation = def::PermutationCoord<EDGES>;
type EdgeOrientation = def::OrientationCoord<EDGES, FLIPS>;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Cube {
    pub corner_permutation: CornerPermutation,
    pub corner_orientation: CornerOrientation,
    pub edge_permutation: EdgePermutation,
    pub edge_orientation: EdgeOrientation,
}

impl Cube {
    pub fn new(
        corner_permutation: CornerPermutation,
        corner_orientation: CornerOrientation,
        edge_permutation: EdgePermutation,
        edge_orientation: EdgeOrientation,
    ) -> Self {
        Self {
            corner_permutation,
            corner_orientation,
            edge_permutation,
            edge_orientation,
        }
    }
}

impl From<FaceTurn> for Cube {
    fn from(turn: FaceTurn) -> Self {
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

        let cp = CORNER_MOVES[ix].p_coordinate();
        let co = CORNER_MOVES[ix].o_coordinate();
        let ep = EDGE_MOVES[ix].p_coordinate();
        let eo = EDGE_MOVES[ix].o_coordinate();

        Self::new(cp, co, ep, eo)
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
            corners.p_coordinate(),
            corners.o_coordinate(),
            edges.p_coordinate(),
            edges.o_coordinate(),
        )
    }
}
