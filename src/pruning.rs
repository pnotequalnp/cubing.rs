use crate::permutations::*;
use crate::util::factorial;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

#[repr(transparent)]
pub struct PruningTable<const COUNT: u8>([u8; factorial(COUNT)])
where
    [u8; factorial(COUNT)]: Sized;

impl<const COUNT: u8> Index<Coordinate<COUNT>> for PruningTable<COUNT>
where
    [u8; factorial(COUNT)]: Sized,
{
    type Output = u8;

    fn index(&self, Coordinate(position): Coordinate<COUNT>) -> &Self::Output {
        &self.0[position as usize]
    }
}

impl<const COUNT: u8> IndexMut<Coordinate<COUNT>> for PruningTable<COUNT>
where
    [u8; factorial(COUNT)]: Sized,
{
    fn index_mut(&mut self, Coordinate(position): Coordinate<COUNT>) -> &mut Self::Output {
        &mut self.0[position as usize]
    }
}

impl<const COUNT: u8> PruningTable<COUNT>
where
    [u8; factorial(COUNT)]: Sized,
    [u8; upscale(COUNT)]: Sized,
{
    pub fn new<const GENERATORS: usize>(
        goal: Coordinate<COUNT>,
        generators: &Generators<GENERATORS>,
        move_table: &MoveTable<COUNT, GENERATORS>,
    ) -> Box<PruningTable<COUNT>> {
        let mut table = unsafe { Box::<PruningTable<COUNT>>::new_zeroed().assume_init() };

        for ix in 0..factorial(COUNT) {
            table.0[ix] = u8::MAX;
        }

        table[goal] = 0;

        (0..).find(|depth| {
            let positions: Vec<Coordinate<COUNT>> = Coordinate::<COUNT>::all()
                .filter_map(|position| (table[position] == *depth).then(|| position))
                .collect();

            for position in &positions {
                for gen in generators.iter() {
                    let new_position = move_table[(*position, *gen)];
                    if table[new_position] > *depth + 1 {
                        table[new_position] = depth + 1;
                    };
                }
            }

            positions.is_empty()
        });

        table
    }
}
