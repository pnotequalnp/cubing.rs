use cube::permutations;
use cube::pruning;
use cube::search::*;
use core::ops::Index;

type CornerGenerator = usize;
type CornerGenerators = permutations::Generators<MOVE_COUNT>;
type CornerPermutationArray = permutations::PermutationArray<CORNERS>;
type CornerPermutationCoord = permutations::Coordinate<CORNERS>;
type CornerPermutationMoveTable = permutations::MoveTable<CORNERS, MOVE_COUNT>;
type CornerPermutationPruningTable = pruning::PruningTable<CORNERS>;

const CORNERS: u8 = 8;
const CORNERS_SOLVED: CornerPermutationCoord =
    permutations::PermutationArray::<CORNERS>::new([0, 1, 2, 3, 4, 5, 6, 7]).coordinate();
const CORNER_MOVES: [CornerPermutationArray; MOVE_COUNT] = [
    permutations::PermutationArray::<CORNERS>::new([3, 0, 1, 2, 4, 5, 6, 7]), // U
    permutations::PermutationArray::<CORNERS>::new([2, 3, 0, 1, 4, 5, 6, 7]), // U2
    permutations::PermutationArray::<CORNERS>::new([1, 2, 3, 0, 4, 5, 6, 7]), // U'
];
// const CORNER_DATA: (CornerPermutationMoveTable, CornerGenerators) =
//     permutations::MoveTable::<CORNERS, MOVE_COUNT>::new(&CORNER_MOVES);
// const CORNER_GENS: CornerGenerators = CORNER_DATA.1;
// const CORNER_TABLE: CornerPermutationMoveTable = CORNER_DATA.0;
const MOVE_COUNT: usize = 3;

pub fn debug() {
    let (move_table, gens) = permutations::MoveTable::<CORNERS, MOVE_COUNT>::new(&CORNER_MOVES);
    let pruning_table = pruning::PruningTable::<CORNERS>::new(CORNERS_SOLVED, &gens, &move_table);
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CornerCube {
    corner_permutation: CornerPermutationCoord,
}

impl CornerCube {
    pub fn new(corner_permutation: CornerPermutationCoord) -> Self {
        Self { corner_permutation }
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
    pub fn apply(&self, position: CornerCube, generator: CornerGenerator) -> CornerCube {
        CornerCube {
            corner_permutation: self.0[(position.corner_permutation, generator)],
        }
    }
}

pub struct CornerCubePruningTable(CornerPermutationPruningTable);

impl Index<CornerCube> for CornerCubePruningTable {
    type Output = u8;

    fn index(&self, CornerCube { corner_permutation }: CornerCube) -> &Self::Output {
        &self.0[corner_permutation]
    }
}

impl Search for CornerCube {
    type HeuristicData = CornerCubePruningTable;
    type TransitionData = (CornerCubeMoveTable, CornerGenerators);

    #[inline(always)]
    fn heuristic(self, data: &Self::HeuristicData) -> Depth {
        data[self]
    }

    #[inline(always)]
    fn transition(
        self,
        (table, generators): &Self::TransitionData,
    ) -> Vec<Self> {
            generators
                .iter()
                .map(move |generator| table.apply(self, *generator))
                .collect()
    }
}
