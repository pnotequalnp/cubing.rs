mod moves;

pub use moves::CORNER_MOVES;
pub use moves::EDGE_MOVES;

pub const CORNERS: usize = 8;
pub const TWISTS: u8 = 3;
pub const EDGES: usize = 12;
pub const FLIPS: u8 = 2;
pub const BELT_EDGES: usize = 4;
pub const MOVE_COUNT: usize = 18;
pub const GENERATORS: [usize; MOVE_COUNT] = {
    let mut gens = [0; MOVE_COUNT];

    let mut ix = 0;
    while ix < MOVE_COUNT {
        gens[ix] = ix;
        ix += 1;
    }

    gens
};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceTurn {
    U, U2, U3, R, R2, R3, F, F2, F3, L, L2, L3, D, D2, D3, B, B2, B3
}

impl std::fmt::Display for FaceTurn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            FaceTurn::U => "U",
            FaceTurn::U2 => "U2",
            FaceTurn::U3 => "U'",
            FaceTurn::R => "R",
            FaceTurn::R2 => "R2",
            FaceTurn::R3 => "R'",
            FaceTurn::F => "F",
            FaceTurn::F2 => "F2",
            FaceTurn::F3 => "F'",
            FaceTurn::L => "L",
            FaceTurn::L2 => "L2",
            FaceTurn::L3 => "L'",
            FaceTurn::D => "D",
            FaceTurn::D2 => "D2",
            FaceTurn::D3 => "D'",
            FaceTurn::B => "B",
            FaceTurn::B2 => "B2",
            FaceTurn::B3 => "B'",
        };

        write!(f, "{}", symbol)
    }
}

impl FaceTurn {
    pub fn format_seq(iter: impl Iterator<Item = Self>) -> String {
        iter.map(|turn| format!("{}", turn))
            .intersperse(" ".to_string())
            .collect()
    }
}

impl From<usize> for FaceTurn {
    fn from(value: usize) -> Self {
        use FaceTurn::*;

        match value {
            0 => U,
            1 => U2,
            2 => U3,
            3 => R,
            4 => R2,
            5 => R3,
            6 => F,
            7 => F2,
            8 => F3,
            9 => L,
            10 => L2,
            11 => L3,
            12 => D,
            13 => D2,
            14 => D3,
            15 => B,
            16 => B2,
            17 => B3,
            _ => panic!("Not a face turn"),
        }
    }
}

impl From<FaceTurn> for usize {
    fn from(val: FaceTurn) -> Self {
        val as usize
    }
}
