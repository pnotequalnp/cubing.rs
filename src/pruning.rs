pub use crate::search::Depth;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::marker::PhantomData;

pub struct Table<S: Copy + Default + Into<usize> + TryFrom<usize>, const N: usize>(
    Box<[Depth; N]>,
    PhantomData<S>,
)
where
    [Depth; N]: Sized;

impl<S: Copy + Default + Into<usize> + TryFrom<usize>, const N: usize> Table<S, N>
where
    [Depth; N]: Sized,
{
    pub fn new<T, const M: usize>(
        generators: &[T; M],
        transition: impl Fn(S, &T) -> S,
    ) -> Self {
        let mut table: Box<[u8; N]> = vec![Depth::MAX; N].into_boxed_slice().try_into().unwrap();

        table[S::default().into()] = 0;

        (0..).find(|depth| {
            let positions: Vec<S> = table
                .iter()
                .enumerate()
                .filter_map(|(ix, d)| (d == depth).then(|| ix))
                .filter_map(|ix| ix.try_into().ok())
                .collect();

            for position in &positions {
                for generator in generators.iter() {
                    let ix = transition(*position, generator).into();
                    if table[ix] > *depth + 1 {
                        table[ix] = depth + 1;
                    };
                }
            }

            positions.is_empty()
        });

        for ix in 0..table.len() {
            if table[ix] == Depth::MAX {
                table[ix] = 0;
            }
        }

        Table(table, PhantomData)
    }

    pub fn lookup(&self, position: S) -> Depth {
        let Table(table, _) = self;
        table[position.into()]
    }
}
