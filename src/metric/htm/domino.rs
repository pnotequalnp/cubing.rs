use super::Htm;
use crate::core::definitions as def;
use crate::util;
use std::convert::TryFrom;

pub type Corners_ = def::Array<8, 1>;
pub type Edges_ = def::Array<8, 1>;
pub struct Corners(pub Corners_);
pub struct Edges(pub Edges_);
pub type Slice = def::Array<4, 1>;
pub type CornerPermutation = def::PermutationCoord<8>;
pub type EdgePermutation = def::PermutationCoord<8>;
pub type SlicePermutation = def::PermutationCoord<4>;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Domino {
    U1, U2, U3, R2, F2, L2, D1, D2, D3, B2
}

impl Domino {
    pub const COUNT: usize = 10;

    #[rustfmt::skip]
    pub const CORNER_MOVES: [Corners; Self::COUNT] = [
        Corners(Corners_::new([(3, 0), (0, 0), (1, 0), (2, 0), (4, 0), (5, 0), (6, 0), (7, 0)])), // U
        Corners(Corners_::new([(2, 0), (3, 0), (0, 0), (1, 0), (4, 0), (5, 0), (6, 0), (7, 0)])), // U2
        Corners(Corners_::new([(1, 0), (2, 0), (3, 0), (0, 0), (4, 0), (5, 0), (6, 0), (7, 0)])), // U'
        Corners(Corners_::new([(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4, 0)])), // D
        Corners(Corners_::new([(0, 0), (1, 0), (2, 0), (3, 0), (6, 0), (7, 0), (4, 0), (5, 0)])), // D2
        Corners(Corners_::new([(0, 0), (1, 0), (2, 0), (3, 0), (7, 0), (4, 0), (5, 0), (6, 0)])), // D'
        Corners(Corners_::new([(7, 0), (1, 0), (2, 0), (4, 0), (3, 0), (5, 0), (6, 0), (0, 0)])), // R2
        Corners(Corners_::new([(5, 0), (4, 0), (2, 0), (3, 0), (1, 0), (0, 0), (6, 0), (7, 0)])), // F2
        Corners(Corners_::new([(0, 0), (6, 0), (5, 0), (3, 0), (4, 0), (2, 0), (1, 0), (7, 0)])), // L2
        Corners(Corners_::new([(0, 0), (1, 0), (7, 0), (6, 0), (4, 0), (5, 0), (3, 0), (2, 0)])), // B2
    ];

    #[rustfmt::skip]
    pub const EDGE_MOVES: [Edges; Self::COUNT] = [
        Edges(Edges_::new([(3, 0), (0, 0), (1, 0), (2, 0), (4, 0), (5, 0), (6, 0), (7, 0)])), // U
        Edges(Edges_::new([(2, 0), (3, 0), (0, 0), (1, 0), (4, 0), (5, 0), (6, 0), (7, 0)])), // U2
        Edges(Edges_::new([(1, 0), (2, 0), (3, 0), (0, 0), (4, 0), (5, 0), (6, 0), (7, 0)])), // U'
        Edges(Edges_::new([(0, 0), (1, 0), (2, 0), (3, 0), (5, 0), (6, 0), (7, 0), (4, 0)])), // D
        Edges(Edges_::new([(0, 0), (1, 0), (2, 0), (3, 0), (6, 0), (7, 0), (4, 0), (5, 0)])), // D2
        Edges(Edges_::new([(0, 0), (1, 0), (2, 0), (3, 0), (7, 0), (4, 0), (5, 0), (6, 0)])), // D'
        Edges(Edges_::new([(4, 0), (1, 0), (2, 0), (3, 0), (0, 0), (5, 0), (6, 0), (7, 0)])), // R2
        Edges(Edges_::new([(0, 0), (5, 0), (2, 0), (3, 0), (4, 0), (1, 0), (6, 0), (7, 0)])), // F2
        Edges(Edges_::new([(0, 0), (1, 0), (6, 0), (3, 0), (4, 0), (5, 0), (2, 0), (7, 0)])), // L2
        Edges(Edges_::new([(0, 0), (1, 0), (2, 0), (7, 0), (4, 0), (5, 0), (6, 0), (3, 0)])), // B2
    ];

    #[rustfmt::skip]
    pub const SLICE_MOVES: [Slice; Self::COUNT] = [
        Slice::new([(0, 0), (1, 0), (2, 0), (3, 0)]), // U
        Slice::new([(0, 0), (1, 0), (2, 0), (3, 0)]), // U2
        Slice::new([(0, 0), (1, 0), (2, 0), (3, 0)]), // U'
        Slice::new([(0, 0), (1, 0), (2, 0), (3, 0)]), // D
        Slice::new([(0, 0), (1, 0), (2, 0), (3, 0)]), // D2
        Slice::new([(0, 0), (1, 0), (2, 0), (3, 0)]), // D'
        Slice::new([(3, 0), (1, 0), (2, 0), (0, 0)]), // R2
        Slice::new([(1, 0), (0, 0), (2, 0), (3, 0)]), // F2
        Slice::new([(0, 0), (2, 0), (1, 0), (3, 0)]), // L2
        Slice::new([(0, 0), (1, 0), (3, 0), (2, 0)]), // B2
    ];

    pub const GENERATORS: [usize; Self::COUNT] = util::count::<{ Self::COUNT }>();
}

impl TryFrom<usize> for Domino {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use Domino::*;

        match value {
            0 => Ok(U1),
            1 => Ok(U2),
            2 => Ok(U3),
            3 => Ok(R2),
            4 => Ok(F2),
            5 => Ok(L2),
            6 => Ok(D1),
            7 => Ok(D2),
            8 => Ok(D3),
            9 => Ok(B2),
            _ => Err(()),
        }
    }
}

impl From<Domino> for usize {
    fn from(val: Domino) -> Self {
        val as usize
    }
}

impl From<Domino> for Htm {
    fn from(val: Domino) -> Htm {
        match val {
            Domino::U1 => Htm::U1,
            Domino::U2 => Htm::U2,
            Domino::U3 => Htm::U3,
            Domino::R2 => Htm::R2,
            Domino::F2 => Htm::F2,
            Domino::L2 => Htm::L2,
            Domino::D1 => Htm::D1,
            Domino::D2 => Htm::D2,
            Domino::D3 => Htm::D3,
            Domino::B2 => Htm::B2,
        }
    }
}

impl From<Domino> for &Corners {
    fn from(htm: Domino) -> Self {
        &Domino::CORNER_MOVES[usize::from(htm)]
    }
}

impl From<Domino> for &Edges {
    fn from(htm: Domino) -> Self {
        &Domino::EDGE_MOVES[usize::from(htm)]
    }
}

impl From<Domino> for &Slice {
    fn from(htm: Domino) -> Self {
        &Domino::SLICE_MOVES[usize::from(htm)]
    }
}
