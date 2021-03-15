use crate::search::Depth;
use crate::util::factorial;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::marker::PhantomData;

#[repr(transparent)]
pub struct PruningTable<S: Into<usize> + TryFrom<usize>, T, const N: usize>(
    Box<[Depth; factorial(N)]>,
    PhantomData<S>,
    PhantomData<T>,
)
where
    [Depth; factorial(N)]: Sized;

impl<S: Into<usize> + TryFrom<usize>, T, const N: usize> PruningTable<S, T, N>
where
    [Depth; factorial(N)]: Sized,
{
    pub fn new<const M: usize>(
        goal: S,
        generators: &[T; M],
        transition: impl Fn(&S, &T) -> S,
    ) -> Self {
        let mut table = Box::new([Depth::MAX; factorial(N)]);

        table[goal.into()] = 0;

        (0..).find(|depth| {
            let positions: Vec<S> = table
                .iter()
                .enumerate()
                .filter_map(|(ix, d)| (d == depth).then(|| ix))
                .filter_map(|ix| ix.try_into().ok())
                .collect();

            for position in &positions {
                for generator in generators.iter() {
                    let ix = transition(position, generator).into();
                    if table[ix] > *depth + 1 {
                        table[ix] = depth + 1;
                    };
                }
            }

            positions.is_empty()
        });

        PruningTable(table, PhantomData, PhantomData)
    }

    pub fn lookup(&self, position: S) -> Depth {
        let PruningTable(table, _, _) = self;
        table[position.into()]
    }
}
