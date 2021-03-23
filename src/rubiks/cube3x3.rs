use crate::core::definitions as def;
use crate::notation::HTM;
use crate::rubiks::*;
use core::iter::FromIterator;

type Corners = def::Array<CORNERS, TWISTS>;
type Edges = def::Array<EDGES, FLIPS>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cube3x3 {
    pub corners: Corners,
    pub edges: Edges,
}

impl Cube3x3 {
    pub const fn new(corners: Corners, edges: Edges) -> Self {
        Self { corners, edges }
    }

    pub const fn apply(&self, turn: HTM) -> Self {
        let Self { corners, edges } = self;
        let ix = turn.to_usize();

        let corners = corners.permute(&CORNER_MOVES[ix]);
        let edges = edges.permute(&EDGE_MOVES[ix]);

        Self { corners, edges }
    }

    pub const fn apply_slice(self, slice: &[HTM]) -> Self {
        let mut state = self;

        let mut ix = 0;
        while ix < slice.len() {
            state = state.apply(slice[ix]);
            ix += 1;
        }

        state
    }

    pub fn apply_seq(&self, sequence: impl IntoIterator<Item = HTM>) -> Self {
        sequence
            .into_iter()
            .fold(self.clone(), |cube, turn| cube.apply(turn))
    }

    pub const fn from_slice(slice: &[HTM]) -> Self {
        let start = Self::new(Corners::IDENTITY, Edges::IDENTITY);
        start.apply_slice(slice)
    }
}

impl From<HTM> for Cube3x3 {
    fn from(turn: HTM) -> Self {
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

        Self::new(CORNER_MOVES[ix].clone(), EDGE_MOVES[ix].clone())
    }
}

impl FromIterator<HTM> for Cube3x3 {
    fn from_iter<T: IntoIterator<Item = HTM>>(iter: T) -> Self {
        let (corners, edges) = iter
            .into_iter()
            .map(usize::from)
            .map(|ix| (&CORNER_MOVES[ix], &EDGE_MOVES[ix]))
            .fold(
                (def::Array::default(), def::Array::default()),
                |(w, x), (y, z)| (w.permute(&y), x.permute(&z)),
            );

        Self::new(corners, edges)
    }
}
