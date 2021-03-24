pub mod domino;
mod moves;

use crate::core::definitions as def;
use crate::util;
use std::convert::TryFrom;
use std::str::FromStr;

pub type Corners = def::Array<8, 3>;
pub type Edges = def::Array<12, 2>;
pub type CornerPermutation = def::PermutationCoord<8>;
pub type EdgePermutation = def::PermutationCoord<12>;
pub type CornerOrientation = def::OrientationCoord<8, 3>;
pub type EdgeOrientation = def::OrientationCoord<12, 2>;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Htm {
    U1, U2, U3, R1, R2, R3, F1, F2, F3, L1, L2, L3, D1, D2, D3, B1, B2, B3
}

impl Htm {
    pub const COUNT: usize = 18;

    pub const GENERATORS: [usize; Self::COUNT] = util::count::<{ Self::COUNT }>();

    pub const fn to_corners(self) -> &'static Corners {
        &Self::CORNER_MOVES[self as usize]
    }

    pub const fn to_edges(self) -> &'static Edges {
        &Self::EDGE_MOVES[self as usize]
    }

    pub fn format_seq(iter: impl Iterator<Item = Self>) -> String {
        iter.map(|turn| format!("{}", turn))
            .intersperse(" ".to_string())
            .collect()
    }

    pub fn parse(str: &str) -> Option<Vec<Self>> {
        str.split_whitespace()
            .map(str::parse::<Self>)
            .map(Result::ok)
            .collect::<Option<_>>()
    }
}

impl std::fmt::Display for Htm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Htm::U1 => "U",
            Htm::U2 => "U2",
            Htm::U3 => "U'",
            Htm::R1 => "R",
            Htm::R2 => "R2",
            Htm::R3 => "R'",
            Htm::F1 => "F",
            Htm::F2 => "F2",
            Htm::F3 => "F'",
            Htm::L1 => "L",
            Htm::L2 => "L2",
            Htm::L3 => "L'",
            Htm::D1 => "D",
            Htm::D2 => "D2",
            Htm::D3 => "D'",
            Htm::B1 => "B",
            Htm::B2 => "B2",
            Htm::B3 => "B'",
        };

        write!(f, "{}", symbol)
    }
}

impl FromStr for Htm {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use Htm::*;

        match value {
            "U" => Ok(U1),
            "U2" => Ok(U2),
            "U'" => Ok(U3),
            "R" => Ok(R1),
            "R2" => Ok(R2),
            "R'" => Ok(R3),
            "F" => Ok(F1),
            "F2" => Ok(F2),
            "F'" => Ok(F3),
            "L" => Ok(L1),
            "L2" => Ok(L2),
            "L'" => Ok(L3),
            "D" => Ok(D1),
            "D2" => Ok(D2),
            "D'" => Ok(D3),
            "B" => Ok(B1),
            "B2" => Ok(B2),
            "B'" => Ok(B3),
            _ => Err(()),
        }
    }
}

impl TryFrom<usize> for Htm {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use Htm::*;

        match value {
            0 => Ok(U1),
            1 => Ok(U2),
            2 => Ok(U3),
            3 => Ok(R1),
            4 => Ok(R2),
            5 => Ok(R3),
            6 => Ok(F1),
            7 => Ok(F2),
            8 => Ok(F3),
            9 => Ok(L1),
            10 => Ok(L2),
            11 => Ok(L3),
            12 => Ok(D1),
            13 => Ok(D2),
            14 => Ok(D3),
            15 => Ok(B1),
            16 => Ok(B2),
            17 => Ok(B3),
            _ => Err(()),
        }
    }
}

impl From<Htm> for usize {
    fn from(val: Htm) -> Self {
        val as usize
    }
}

impl TryFrom<&Corners> for Htm {
    type Error = ();

    fn try_from(value: &Corners) -> Result<Self, Self::Error> {
        Self::CORNER_MOVES
            .iter()
            .position(|m| m == value)
            .ok_or(())
            .and_then(Htm::try_from)
    }
}

impl From<Htm> for &Corners {
    fn from(htm: Htm) -> Self {
        &Htm::CORNER_MOVES[usize::from(htm)]
    }
}

impl From<Htm> for &Edges {
    fn from(htm: Htm) -> Self {
        &Htm::EDGE_MOVES[usize::from(htm)]
    }
}
