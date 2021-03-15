use core::convert::TryFrom;
use cube::permutations;
use cube::pruning;
use cube::search::*;
use std::time::Instant;

type CornerPermutationArray = permutations::PermutationArray<CORNERS>;
type CornerPermutationCoord = permutations::Coordinate<CORNERS>;
type CornerPermutationMoveTable = permutations::TransitionTable<CORNERS, MOVE_COUNT>;
type CornerCubePruningTable = pruning::PruningTable<CornerCube, Move, CORNERS>;
type Move = usize;

const CORNERS: usize = 8;
const CORNERS_SOLVED: CornerPermutationCoord =
    permutations::PermutationArray::<CORNERS>::IDENTITY.coordinate();
const CORNER_MOVES: [CornerPermutationArray; MOVE_COUNT] = [
    permutations::PermutationArray::<CORNERS>::new([3, 0, 1, 2, 4, 5, 6, 7]), // U
    permutations::PermutationArray::<CORNERS>::new([2, 3, 0, 1, 4, 5, 6, 7]), // U2
    permutations::PermutationArray::<CORNERS>::new([1, 2, 3, 0, 4, 5, 6, 7]), // U'
    permutations::PermutationArray::<CORNERS>::new([4, 1, 2, 0, 7, 5, 6, 3]), // R
    permutations::PermutationArray::<CORNERS>::new([7, 1, 2, 4, 3, 5, 6, 0]), // R2
    permutations::PermutationArray::<CORNERS>::new([3, 1, 2, 7, 0, 5, 6, 4]), // R'
];
const MOVE_COUNT: usize = 6;
const MOVES: [Move; MOVE_COUNT] = {
    let mut xs = [0; MOVE_COUNT];

    let mut ix = 1;
    while ix < MOVE_COUNT {
        xs[ix] = ix;
        ix += 1;
    }

    xs
};

pub fn main() {
    // println!("{:?}", CORNER_MOVES[4].permute(&CORNER_MOVES[3]));

    println!("Generating move table...");
    let now = Instant::now();
    let move_table = CornerCubeMoveTable::new(&CORNER_MOVES);
    println!("Generated move table in {:?}\n", now.elapsed());

    println!("Generating pruning table...");
    let now = Instant::now();
    let pruning_table =
        CornerCubePruningTable::new(CornerCube::default(), &MOVES, |position, turn| {
            move_table.apply(*position, *turn)
        });
    println!("Generated pruning table in {:?}\n", now.elapsed());

    let perm: CornerPermutationArray = [4].iter().map(|ix| &CORNER_MOVES[*ix]).cloned().product();

    let scramble = CornerCube::new(perm.coordinate());

    println!("Solving position {:?}...", scramble);
    let now = Instant::now();
    let res = scramble.ida_star(&pruning_table, &move_table, 15);
    println!("Solved in {:?}", now.elapsed());
    println!("Solution: {:?}", res);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CornerCube {
    corner_permutation: CornerPermutationCoord,
}

impl CornerCube {
    pub fn new(corner_permutation: CornerPermutationCoord) -> Self {
        Self { corner_permutation }
    }
}

impl From<CornerCube> for usize {
    fn from(cube: CornerCube) -> Self {
        cube.corner_permutation.into()
    }
}

impl TryFrom<usize> for CornerCube {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        CornerPermutationCoord::try_from(value).map(Self::new)
    }
}

impl Default for CornerCube {
    fn default() -> Self {
        CornerCube {
            corner_permutation: CORNERS_SOLVED,
        }
    }
}

pub struct CornerCubeMoveTable(CornerPermutationMoveTable);

impl CornerCubeMoveTable {
    pub fn new(moves: &[CornerPermutationArray; MOVE_COUNT]) -> Self {
        Self(CornerPermutationMoveTable::new(moves))
    }

    pub fn apply(&self, position: CornerCube, permutation_index: usize) -> CornerCube {
        let CornerCubeMoveTable(table) = self;
        let CornerCube { corner_permutation } = position;
        let cp = table.transition(corner_permutation, permutation_index);
        CornerCube::new(cp)
    }
}

impl Search for CornerCube {
    type Edge = usize;
    type HeuristicData = CornerCubePruningTable;
    type TransitionData = CornerCubeMoveTable;

    fn heuristic(self, table: &Self::HeuristicData) -> Depth {
        table.lookup(self)
    }

    fn transition(self, table: &Self::TransitionData) -> Vec<(Self, Self::Edge)> {
        (0..MOVE_COUNT)
            .map(|move_index| (table.apply(self, move_index), move_index))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn permutation_array_cancellation() {
        let x = CORNER_MOVES[0].permute(&CORNER_MOVES[2]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[2].permute(&CORNER_MOVES[0]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[1].permute(&CORNER_MOVES[1]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[0]
            .permute(&CORNER_MOVES[0])
            .permute(&CORNER_MOVES[0])
            .permute(&CORNER_MOVES[0]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[2]
            .permute(&CORNER_MOVES[2])
            .permute(&CORNER_MOVES[2])
            .permute(&CORNER_MOVES[2]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[1]
            .permute(&CORNER_MOVES[0])
            .permute(&CORNER_MOVES[0]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[3].permute(&CORNER_MOVES[5]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[5].permute(&CORNER_MOVES[3]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[4].permute(&CORNER_MOVES[4]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[3]
            .permute(&CORNER_MOVES[3])
            .permute(&CORNER_MOVES[3])
            .permute(&CORNER_MOVES[3]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[5]
            .permute(&CORNER_MOVES[5])
            .permute(&CORNER_MOVES[5])
            .permute(&CORNER_MOVES[5]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);

        let x = CORNER_MOVES[4]
            .permute(&CORNER_MOVES[3])
            .permute(&CORNER_MOVES[3]);
        assert_eq!(CornerPermutationArray::IDENTITY, x);
    }

    #[test]
    pub fn coordinate_cancellation() {
        let move_table = CornerCubeMoveTable::new(&CORNER_MOVES);
        let moves = CORNER_MOVES.map(|m| CornerCube::new(m.coordinate()));
        let solved = CornerCube::new(CornerPermutationArray::IDENTITY.coordinate());

        let x = move_table.apply(moves[0], 2);
        assert_eq!(solved, x);

        let x = move_table.apply(moves[1], 1);
        assert_eq!(solved, x);

        let x = move_table.apply(moves[2], 0);
        assert_eq!(solved, x);

        let x = move_table.apply(moves[3], 5);
        assert_eq!(solved, x);

        let x = move_table.apply(moves[4], 4);
        assert_eq!(solved, x);

        let x = move_table.apply(moves[5], 3);
        assert_eq!(solved, x);
    }

    #[test]
    pub fn coordinate_inversion() {
        for m in CORNER_MOVES.iter() {
            assert_eq!(m, &m.coordinate().permutation_array());
        }
    }
}
