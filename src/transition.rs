use alloc::boxed::Box;
use core::convert::TryInto;

/// A table which allows for the precomputation of all transitions of a set of generators.
pub struct Table<T: Copy + Into<usize>, const SIZE: usize, const G: usize>(Box<[T; SIZE * G]>)
where
    [T; SIZE * G]: Sized;

impl<T, const SIZE: usize, const G: usize> Table<T, SIZE, G>
where
    [T; SIZE * G]: Sized,
    T: Copy + Into<usize>,
{
    /// Construct a new transition table.
    ///
    /// WARNING: the iterator `all` *must* produce `SIZE` values or this function will panic.
    ///
    /// NB: The indices of the generators are the caller's responsibility to track: they are needed
    /// for lookup later.
    pub fn new<Alt>(
        generators: &[Alt; G],
        all: impl Iterator<Item = T>,
        transition: impl Fn(T, &Alt) -> T,
    ) -> Self
    where
        T: Copy + core::fmt::Debug + Default,
        [T; SIZE * G]: Sized,
    {
        let mut table: Box<[T; SIZE * G]> = vec![T::default(); SIZE * G]
            .into_boxed_slice()
            .try_into()
            .unwrap();

        let mut c = 0;

        for state in all {
            if c >= SIZE {
                panic!("Iterator produced too many values!");
            }

            for (column, generator) in generators.iter().enumerate() {
                let row: usize = state.into();
                table[row * G + column] = transition(state, generator);
            }
            c += 1;
        }

        if c != SIZE {
            panic!("Iterator produced too few values!");
        }

        Self(table)
    }

    /// Look up the result of the precomputed transition. The `generator_index` argument is the
    /// index of the generator in the array passed to `Table::new`.
    pub fn lookup(&self, t: T, generator_index: usize) -> T {
        debug_assert!(
            generator_index < G,
            "`generator_index` out of bounds: {:?}",
            generator_index
        );

        let column = generator_index;
        let row: usize = t.into();
        self.0[row * G + column]
    }
}
