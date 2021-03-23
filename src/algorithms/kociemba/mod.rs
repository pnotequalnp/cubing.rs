pub mod phase1;
pub mod phase2;

use crate::core::search::{ida_iter, Depth};
use crate::rubiks::{Cube3x3, FaceTurn};
use alloc::vec::Vec;
use core::convert::TryFrom;

pub use phase1::Cube as Phase1;
pub use phase2::Cube as Phase2;

pub type Tables = (
    phase1::Table,
    phase1::PruningTable,
    phase2::Table,
    phase2::PruningTable,
);

impl Cube3x3 {
    pub fn kociemba(
        &self,
        (move_table_1, pruning_table_1, move_table_2, pruning_table_2): &Tables,
        max_length: Option<Depth>,
    ) -> Vec<FaceTurn> {
        let initial_phase_1 = Phase1::from(self);

        let res =
            ida_iter(initial_phase_1, pruning_table_1, move_table_1, None).find_map(|path_1| {
                let sol_1 = path_1.into_iter().map(|(_, e)| e).collect::<Vec<_>>();

                let intermediate_position =
                    self.apply_seq(sol_1.iter().cloned().map(FaceTurn::from));

                let initial_phase_2 = Phase2::try_from(&intermediate_position).unwrap();

                let max = max_length.map(|l| l - sol_1.len() as Depth);

                let path_2 =
                    ida_iter(initial_phase_2, pruning_table_2, move_table_2, max).next()?;
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
}

pub fn generate_tables() -> Tables {
    let move_1 = phase1::Table::new();
    let move_2 = phase2::Table::new();

    let pruning_1 = phase1::PruningTable::new(&move_1);
    let pruning_2 = phase2::PruningTable::new(&move_2);

    (move_1, pruning_1, move_2, pruning_2)
}
