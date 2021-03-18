pub mod phase1;
pub mod phase2;

use crate::rubiks::FaceTurn;
use cube::search::Search;

pub use phase1::Cube as Phase1;
pub use phase2::Cube as Phase2;

pub fn solve(
    scramble: Vec<FaceTurn>,
    move_table_1: phase1::Table,
    _move_table_2: phase2::Table,
    pruning_table_1: phase1::PruningTable,
    _pruning_table_2: phase2::PruningTable,
) -> Vec<FaceTurn> {
    let position = scramble.iter().cloned().collect::<Phase1>();
    let (_path, moves) = position
        .ida_star(
            &pruning_table_1,
            &move_table_1,
            20,
            Some(|depth| println!("(Phase 1) Depth {:?} complete", depth)),
        )
        .unwrap();

    moves.into_iter().map(FaceTurn::from).collect()
}
