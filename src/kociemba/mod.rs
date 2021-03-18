pub mod phase1;
pub mod phase2;

use crate::rubiks::{Cube, FaceTurn};
use rubiks_rs::search::{ida_iter, Depth};

pub use phase1::Cube as Phase1;
pub use phase2::Cube as Phase2;

pub fn solve(
    scramble: Vec<FaceTurn>,
    move_table_1: &phase1::Table,
    move_table_2: &phase2::Table,
    pruning_table_1: &phase1::PruningTable,
    pruning_table_2: &phase2::PruningTable,
    max_length: Option<Depth>,
) -> Vec<FaceTurn> {
    let initial_position = scramble.iter().cloned().collect::<Phase1>();

    let res =
        ida_iter(initial_position, &pruning_table_1, &move_table_1, None).find_map(|path_1| {
            let sol_1 = path_1.into_iter().map(|(_, e)| e).collect::<Vec<_>>();

            let intermediate_full_position = scramble
                .iter()
                .cloned()
                .chain(sol_1.iter().cloned().map(FaceTurn::from))
                .collect::<Cube>();

            let intermediate_position = Phase2::new(
                intermediate_full_position.corner_permutation,
                intermediate_full_position
                    .edge_permutation
                    .array::<1>()
                    .truncate::<8>()
                    .unwrap()
                    .p_coordinate(),
                intermediate_full_position
                    .edge_permutation
                    .array::<1>()
                    .drop::<4>()
                    .unwrap()
                    .p_coordinate(),
            );

            let max = max_length.map(|l| l - sol_1.len() as Depth);

            let path_2 =
                ida_iter(intermediate_position, &pruning_table_2, &move_table_2, max).next()?;
            let sol_2 = path_2.into_iter().map(|(_, e)| e).collect::<Vec<_>>();

            let solution = sol_1
                .into_iter()
                .map(FaceTurn::from)
                .chain(sol_2.into_iter().map(Phase2::face_turn))
                .collect::<Vec<_>>();

            Some(solution)
        });

    res.unwrap()
}
