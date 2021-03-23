use core::convert::TryFrom;
use core::str::FromStr;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTM {
    U1, U2, U3, R1, R2, R3, F1, F2, F3, L1, L2, L3, D1, D2, D3, B1, B2, B3
}

impl core::fmt::Display for HTM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let symbol = match self {
            HTM::U1 => "U",
            HTM::U2 => "U2",
            HTM::U3 => "U'",
            HTM::R1 => "R",
            HTM::R2 => "R2",
            HTM::R3 => "R'",
            HTM::F1 => "F",
            HTM::F2 => "F2",
            HTM::F3 => "F'",
            HTM::L1 => "L",
            HTM::L2 => "L2",
            HTM::L3 => "L'",
            HTM::D1 => "D",
            HTM::D2 => "D2",
            HTM::D3 => "D'",
            HTM::B1 => "B",
            HTM::B2 => "B2",
            HTM::B3 => "B'",
        };

        write!(f, "{}", symbol)
    }
}

impl HTM {
    pub const fn to_usize(self) -> usize {
        self as usize
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

impl TryFrom<usize> for HTM {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use HTM::*;

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

impl FromStr for HTM {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use HTM::*;

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

impl From<HTM> for usize {
    fn from(val: HTM) -> Self {
        val as usize
    }
}
