use crate::core::definitions as def;
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

    pub const fn apply(&self, turn: FaceTurn) -> Self {
        let Self { corners, edges } = self;
        let ix = turn.to_usize();

        let corners = corners.permute(&CORNER_MOVES[ix]);
        let edges = edges.permute(&EDGE_MOVES[ix]);

        Self { corners, edges }
    }

    pub const fn apply_slice(self, slice: &[FaceTurn]) -> Self {
        let mut state = self;

        let mut ix = 0;
        while ix < slice.len() {
            state = state.apply(slice[ix]);
            ix += 1;
        }

        state

    }

    pub fn apply_seq(&self, sequence: impl IntoIterator<Item = FaceTurn>) -> Self {
        sequence
            .into_iter()
            .fold(self.clone(), |cube, turn| cube.apply(turn))
    }

    pub const fn from_slice(slice: &[FaceTurn]) -> Self {
        let start = Self::new(Corners::IDENTITY, Edges::IDENTITY);
        start.apply_slice(slice)
    }
}

impl From<FaceTurn> for Cube3x3 {
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

        Self::new(CORNER_MOVES[ix].clone(), EDGE_MOVES[ix].clone())
    }
}

impl FromIterator<FaceTurn> for Cube3x3 {
    fn from_iter<T: IntoIterator<Item = FaceTurn>>(iter: T) -> Self {
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
