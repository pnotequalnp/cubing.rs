use crate::turns::FaceTurn;
use core::iter::FromIterator;
use cube::definitions as def;
use cube::search::{Depth, Search};

const CORNERS: usize = 8;
const ORIENTATIONS: u8 = 3;
const MOVE_COUNT: usize = 9;
pub const MOVES: [def::Array<CORNERS, ORIENTATIONS>; MOVE_COUNT] = [
    def::Array::new([
        (3, 0),
        (0, 0),
        (1, 0),
        (2, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
    ]), // U
    def::Array::new([
        (2, 0),
        (3, 0),
        (0, 0),
        (1, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
    ]), // U2
    def::Array::new([
        (1, 0),
        (2, 0),
        (3, 0),
        (0, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
    ]), // U'
    def::Array::new([
        (4, 2),
        (1, 0),
        (2, 0),
        (0, 1),
        (7, 1),
        (5, 0),
        (6, 0),
        (3, 2),
    ]), // R
    def::Array::new([
        (7, 0),
        (1, 0),
        (2, 0),
        (4, 0),
        (3, 0),
        (5, 0),
        (6, 0),
        (0, 0),
    ]), // R2
    def::Array::new([
        (3, 2),
        (1, 0),
        (2, 0),
        (7, 1),
        (0, 1),
        (5, 0),
        (6, 0),
        (4, 2),
    ]), // R'
    def::Array::new([
        (1, 1),
        (5, 2),
        (2, 0),
        (3, 0),
        (0, 2),
        (4, 1),
        (6, 0),
        (7, 0),
    ]), // F
    def::Array::new([
        (5, 0),
        (4, 0),
        (2, 0),
        (3, 0),
        (1, 0),
        (0, 0),
        (6, 0),
        (7, 0),
    ]), // F2
    def::Array::new([
        (4, 1),
        (0, 2),
        (2, 0),
        (3, 0),
        (5, 2),
        (1, 1),
        (6, 0),
        (7, 0),
    ]), // F'
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Cube(pub def::Coordinate<8, 3>);

impl Search for Cube {
    type Edge = usize;
    type HeuristicData = PruningTable;
    type TransitionData = Table;

    fn heuristic(self, table: &Self::HeuristicData) -> cube::search::Depth {
        table.lookup(self)
    }

    fn transition(self, table: &Self::TransitionData) -> Vec<(Self, Self::Edge)> {
        (0..MOVE_COUNT)
            .map(|ix| (table.lookup(self, ix), ix))
            .collect()
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
            _ => panic!("Not a 2x2 move")
        };

        Cube(MOVES[ix].coordinate())
    }
}

impl FromIterator<FaceTurn> for Cube {
    fn from_iter<T: IntoIterator<Item = FaceTurn>>(iter: T) -> Self {
        let a: def::Array<8, 3> = iter
            .into_iter()
            .map(Self::from)
            .map(|Cube(c)| c.array())
            .product();

        Cube(a.coordinate())
    }
}

impl Cube {
    pub fn index(ix: usize) -> FaceTurn {
        match ix {
            0 => FaceTurn::U,
            1 => FaceTurn::U2,
            2 => FaceTurn::U3,
            3 => FaceTurn::R,
            4 => FaceTurn::R2,
            5 => FaceTurn::R3,
            6 => FaceTurn::F,
            7 => FaceTurn::F2,
            8 => FaceTurn::F3,
            _ => panic!(),
        }
    }
}

pub struct Table(pub def::FullTable<CORNERS, ORIENTATIONS, MOVE_COUNT>);

impl Table {
    pub fn new() -> Self {
        Self(def::FullTable::new(&MOVES))
    }

    pub fn lookup(&self, position: Cube, permutation_index: usize) -> Cube {
        let Self(table) = self;
        let Cube(coord) = position;
        Cube(table.lookup(coord, permutation_index))
    }
}

pub struct PruningTable(def::FullPruning<CORNERS, ORIENTATIONS, MOVE_COUNT>);

impl PruningTable {
    pub fn new(Table(table): &Table) -> Self {
        Self(def::FullPruning::new(table))
    }

    pub fn lookup(&self, Cube(coord): Cube) -> Depth {
        let PruningTable(table) = self;

        table.lookup(coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cube::oriented::Array;

    #[test]
    pub fn array_cancellation() {
        for (x, y) in [(0, 2), (2, 0), (1, 1), (3, 5), (5, 3), (4, 4)].iter() {
            assert_eq!(Array::<8, 3>::default(), MOVES[*x].permute(&MOVES[*y]));
        }
    }

    #[test]
    pub fn coordinate_inverses() {
        for m in MOVES.iter() {
            assert_eq!(*m, m.coordinate().array());
        }
    }

    #[test]
    pub fn coordinate_cancellation() {
        let table = Table::new();

        for (x, y) in [(2, 0), (2, 0), (1, 1), (3, 5), (5, 3), (4, 4)].iter() {
            assert_eq!(
                Cube::default(),
                table.lookup(Cube(MOVES[*x].coordinate()), *y),
                "{} {}",
                x,
                y
            );
        }
    }
}
