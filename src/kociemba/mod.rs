pub mod phase1;
pub mod phase2;

use crate::rubiks::{Cube, FaceTurn};
use rubiks_rs::search::{Depth, Search};

pub use phase1::Cube as Phase1;
pub use phase2::Cube as Phase2;

pub fn solve(
    scramble: Vec<FaceTurn>,
    move_table_1: phase1::Table,
    move_table_2: phase2::Table,
    pruning_table_1: phase1::PruningTable,
    pruning_table_2: phase2::PruningTable,
    _max_length: Option<Depth>,
) -> Vec<FaceTurn> {
    let position = scramble.iter().cloned().collect::<Phase1>();
    let (_path_1, moves_1) = position
        .ida_star(
            &pruning_table_1,
            &move_table_1,
            13,
            Some(|_| {}),
        )
        .unwrap();

    let intermediate = scramble
        .iter()
        .cloned()
        .chain(moves_1.iter().cloned().map(FaceTurn::from))
        .collect::<Cube>();

    let position = Phase2::new(
        intermediate.corner_permutation,
        intermediate
            .edge_permutation
            .array::<1>()
            .truncate::<8>()
            .unwrap()
            .p_coordinate(),
        intermediate
            .edge_permutation
            .array::<1>()
            .drop::<4>()
            .unwrap()
            .p_coordinate(),
    );

    let (_path_2, moves_2) = position
        .ida_star(
            &pruning_table_2,
            &move_table_2,
            19,
            Some(|_| {}),
        )
        .unwrap();

    moves_1
        .into_iter()
        .map(FaceTurn::from)
        .chain(moves_2.into_iter().map(Phase2::face_turn))
        .collect()
}
