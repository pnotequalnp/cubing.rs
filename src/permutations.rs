use crate::util::factorial;
use alloc::boxed::Box;
use core::convert::TryFrom;
use core::iter::Product;

type Element = u8;
type CoordWidth = u32;

/// An array which represents all permutations of `N` elements. It contains all elements indexed `0`
/// to `N - 1` exactly once, in any order.
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermutationArray<const N: usize>([Element; N]);

impl<const N: usize> PermutationArray<N> {
    /// The identity permutation. `x.permute(IDENTITY) == x`
    pub const IDENTITY: PermutationArray<N> = {
        let mut pm = [0; N];
        let mut ix = 1u8;

        while ix < N as u8 {
            pm[ix as usize] = ix;
            ix += 1;
        }

        PermutationArray(pm)
    };

    /// Construct a new `PermutationArray` from a raw array representation. Panics if the argument
    /// is not a proper permutation array.
    pub const fn new(candidate: [Element; N]) -> Self {
        let mut ix: usize = 0;
        while ix < N - 1 {
            let mut jx: usize = ix + 1;
            while jx < N {
                if candidate[ix] == candidate[jx] {
                    panic!("Not a valid permutation array");
                }
                jx += 1;
            }
            ix += 1;
        }

        PermutationArray(candidate)
    }

    /// Permute this array according to the permutation given by the argument.
    pub const fn permute(&self, permutation: &Self) -> Self {
        let PermutationArray(mut pm) = Self::IDENTITY;
        let mut ix: usize = 0;

        while ix < N {
            let jx = permutation.0[ix] as usize;
            pm[ix] = self.0[jx];
            ix += 1;
        }

        PermutationArray(pm)
    }

    // FIXME: change this to a true in-place algorithm
    /// Permute this array in-place according to the permutation given by the argument.
    pub fn permute_(&mut self, permutation: &Self) {
        let PermutationArray(mut pm) = Self::IDENTITY;
        let mut ix: usize = 0;

        while ix < N {
            let jx = permutation.0[ix] as usize;
            pm[ix] = self.0[jx];
            ix += 1;
        }

        self.0 = pm;
    }

    /// Transform a permutation into its coordinate representation. This is only possible with
    /// arrays less 13 elements long because the coordinate width is currently hardcoded to 32 bits.
    pub const fn coordinate(&self) -> Coordinate<N> {
        debug_assert!(N < 13, "Coordinate space exceeds u32");

        let PermutationArray(pm) = self;

        let n: CoordWidth = N as CoordWidth;
        let mut t: CoordWidth = 0;
        let mut ix: CoordWidth = 0;

        while ix < n - 1 {
            t *= n - ix;

            let mut jx = ix as usize + 1;
            while jx < N {
                if pm[ix as usize] > pm[jx] {
                    t += 1;
                };
                jx += 1;
            }

            ix += 1;
        }

        debug_assert!(
            (t as usize) < factorial(N),
            "Coordinate calculated outside of coordinate space"
        );

        Coordinate(t)
    }
}

impl<const N: usize> Product for PermutationArray<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::IDENTITY, |x, y| x.permute(&y))
    }
}

/// Coordinate representation for all permutations of `N` elements. Because it is hardcoded at 32
/// bits, it only works for `N < 13`. `N >= 13` will panic on overflow.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coordinate<const N: usize>(CoordWidth);

impl<const N: usize> Coordinate<N> {
    /// The highest coordinate value possible for this `N`, computed as `N! - 1`.
    pub const MAX: usize = factorial(N) - 1;

    /// The number of coordinates in the coordinate space for this `N`, computed as `N!`.
    pub const BOUND: CoordWidth = factorial(N) as CoordWidth;

    /// Iterate over all coordinates in the coordinate space.
    pub fn all() -> impl Iterator<Item = Self> {
        (0..Self::BOUND).map(Coordinate)
    }

    /// Extract the permutation array representation which corresponds to this coordinate. This is
    /// inverse of `PermutationArray::coordinate`. `x.coordinate().permutation_array() == x`
    pub const fn permutation_array(&self) -> PermutationArray<N> {
        let Coordinate(mut t) = self;
        let mut pm: [Element; N] = [0; N];
        let mut ix: Element = N as Element - 2;

        // ix from N-2 to 0, with wrapping subtraction because it is unsigned
        while ix != Element::MAX {
            let r: CoordWidth = N as CoordWidth - ix as CoordWidth;
            pm[ix as usize] = (t % r) as Element;
            t /= r;

            let mut jx: usize = ix as usize + 1;
            while jx < N {
                if pm[jx] >= pm[ix as usize] {
                    pm[jx] += 1;
                };
                jx += 1;
            }

            ix = ix.wrapping_sub(1);
        }

        PermutationArray(pm)
    }
}

impl<const N: usize> TryFrom<usize> for Coordinate<N> {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= Coordinate::<N>::MAX {
            Ok(Coordinate(value as CoordWidth))
        } else {
            Err(())
        }
    }
}

impl<const N: usize> From<Coordinate<N>> for usize {
    fn from(Coordinate(value): Coordinate<N>) -> Self {
        value as usize
    }
}

/// A table which allows for direct permutation of coordinates by pre-calculating the effect of `M`
/// particular permutations on the entire space.
///
/// NB: Indexing is based on the index of the coordinate in the `permutations` argument of `new`. It
/// is your responsibility to keep track of what coordinates these indices correlate with.
#[repr(transparent)]
pub struct TransitionTable<const N: usize, const M: usize>(Box<[[Coordinate<N>; factorial(N)]; M]>)
where
    [Coordinate<N>; factorial(N)]: Sized;

impl<const N: usize, const M: usize> TransitionTable<N, M>
where
    [Coordinate<N>; factorial(N)]: Sized,
{
    /// Create a new table from a slice of generators. The table is based on the index of the
    /// permutations in the argument, and it is your responsibility to track them, as you will need
    /// them to look up coordinates with `transition`.
    pub fn new(permutations: &[PermutationArray<N>; M]) -> Self {
        let mut table: Box<[[Coordinate<N>; factorial(N)]; M]> =
            Box::new([[Coordinate::<N>(0); factorial(N)]; M]);

        for position in Coordinate::<N>::all() {
            for (ix, permutation) in permutations.iter().enumerate() {
                let position_index: usize = position.into();
                table[ix][position_index] = position
                    .permutation_array()
                    .permute(permutation)
                    .coordinate();
            }
        }

        TransitionTable(table)
    }

    /// Look up the coordinate resulting from permuting `position` with the permutation at index
    /// `permutation_index` in the slice originally passed to `new` when constructing this table.
    pub fn transition(&self, position: Coordinate<N>, permutation_index: usize) -> Coordinate<N> {
        let TransitionTable(table) = self;
        let position_index: usize = position.into();
        table[permutation_index][position_index]
    }
}
