use crate::util::{binomial, factorial, power};
use core::convert::TryFrom;
use core::iter::Product;

type Element = u8;
type Orientation = u8;
type CCoordWidth = u16;
type OCoordWidth = u16;
type PCoordWidth = u32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreationError {
    InvalidOrientation,
    InvalidPermutation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array<const N: usize, const M: Orientation>([(Element, Orientation); N]);

impl<const N: usize, const M: Orientation> Array<N, M> {
    const IDENTITY: Self = {
        let mut pm = [(0, 0); N];

        let mut ix = 1u8;
        while ix < N as u8 {
            pm[ix as usize].0 = ix;
            ix += 1;
        }

        Self(pm)
    };

    pub const fn new(candidate: [(Element, Orientation); N]) -> Self {
        let mut ix = 0;
        while ix < N {
            if candidate[ix].1 >= M {
                panic!("InvalidOrientation");
            };

            let (i, _) = candidate[ix];
            let mut jx: usize = ix + 1;
            while jx < N {
                if i == candidate[jx].0 {
                    panic!("InvalidPermutation");
                };
                jx += 1;
            }
            ix += 1;
        }

        Self(candidate)
    }

    pub const fn create(candidate: [(Element, Orientation); N]) -> Result<Self, CreationError> {
        let mut ix = 0;
        while ix < N {
            if candidate[ix].1 >= M {
                return Err(CreationError::InvalidOrientation);
            };

            let (i, _) = candidate[ix];
            let mut jx: usize = ix + 1;
            while jx < N {
                if i == candidate[jx].0 {
                    return Err(CreationError::InvalidPermutation);
                };
                jx += 1;
            }
            ix += 1;
        }

        Ok(Self(candidate))
    }

    pub const fn permute(&self, Self(that): &Self) -> Self {
        let Self(this) = self;
        let Self(mut base) = Self::IDENTITY;

        let mut ix = 0;
        while ix < N {
            let (jx, o1) = that[ix];
            let (j, o2) = this[jx as usize];
            base[ix] = (j, (o1 + o2) % M);
            ix += 1;
        }

        Array(base)
    }

    pub const fn p_coordinate(&self) -> PermutationCoord<N> {
        debug_assert!(N < 13, "Coordinate space exceeds u32");

        let Self(this) = self;

        let n = N as PCoordWidth;
        let mut t: PCoordWidth = 0;

        let mut ix: PCoordWidth = 0;
        while ix < n - 1 {
            t *= n - ix;

            let mut jx = ix as usize + 1;
            while jx < N {
                if this[ix as usize].0 > this[jx].0 {
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

        PermutationCoord(t)
    }

    pub const fn o_coordinate(&self) -> OrientationCoord<N, M> {
        let Self(this) = self;
        let mut t: OCoordWidth = 0;

        let mut ix = 0;
        while ix < N - 1 {
            t *= M as OCoordWidth;
            t += this[ix].1 as OCoordWidth;

            ix += 1;
        }

        OrientationCoord(t)
    }

    pub const fn c_coordinate<const K: usize>(&self) -> CombinationCoord<N, K> {
        let Self(cm) = self;

        let mut t: CCoordWidth = 0;
        let mut r = K;

        let mut ix = N - 1;
        while ix != usize::MAX {
            if cm[ix].0 as usize >= N - K {
                t += binomial(ix, r) as CCoordWidth;
                r -= 1;
            };
            ix = ix.wrapping_sub(1);
        }

        debug_assert!((t as usize) < binomial(N, K));

        CombinationCoord(t)
    }

    pub const fn coordinate(&self) -> Coordinate<N, M> {
        Coordinate(self.o_coordinate(), self.p_coordinate())
    }
}

impl<const N: usize, const M: Orientation> Product for Array<N, M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::IDENTITY, |x, y| x.permute(&y))
    }
}

impl<const N: usize, const M: Orientation> Default for Array<N, M> {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl<const N: usize, const M: Orientation> TryFrom<[(Element, Orientation); N]> for Array<N, M> {
    type Error = CreationError;

    fn try_from(value: [(Element, Orientation); N]) -> Result<Self, Self::Error> {
        Self::create(value)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Coordinate<const N: usize, const M: Orientation>(
    pub OrientationCoord<N, M>,
    pub PermutationCoord<N>,
);

impl<const N: usize, const M: Orientation> Coordinate<N, M> {
    pub fn array(self) -> Array<N, M> {
        let Coordinate(o, p) = self;
        Array(p.raw_array().zip(o.raw_array()))
    }

    pub fn all() -> impl Iterator<Item = Self> {
        OrientationCoord::<N, M>::all()
            .flat_map(|o| PermutationCoord::<N>::all().map(move |p| Self(o, p)))
    }
}

impl<const N: usize, const M: Orientation> From<Coordinate<N, M>> for usize {
    fn from(Coordinate(OrientationCoord(o), PermutationCoord(p)): Coordinate<N, M>) -> Self {
        p as usize * power(M, N - 1) + o as usize
    }
}

impl<const N: usize, const M: Orientation> TryFrom<usize> for Coordinate<N, M> {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= power(M, N - 1) * factorial(N) {
            return Err(());
        };

        let o = OrientationCoord((value % power(M, N - 1)) as OCoordWidth);
        let p = PermutationCoord((value / power(M, N - 1)) as PCoordWidth);

        Ok(Coordinate(o, p))
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OrientationCoord<const N: usize, const M: Orientation>(OCoordWidth);

impl<const N: usize, const M: Orientation> OrientationCoord<N, M> {
    pub const BOUND: usize = power(M, N - 1);

    fn raw_array(self) -> [Orientation; N] {
        let OrientationCoord(mut t) = self;

        let mut s = 0i8;
        let mut or = [0; N];

        for ix in (0..N - 1).rev() {
            let r = (t % M as OCoordWidth) as Orientation;
            or[ix] = r;
            s -= r as i8;
            if s < 0 {
                s += M as i8
            };
            t /= M as OCoordWidth;
        }

        or[N - 1] = s as Orientation;

        or
    }

    pub fn array(self) -> Array<N, M> {
        let mut nats = 0..;
        Array(self.raw_array().map(|x| (nats.next().unwrap(), x)))
    }

    pub fn all() -> impl Iterator<Item = Self> {
        (0..power(M, N - 1)).map(|x| OrientationCoord(x as OCoordWidth))
    }

    pub fn permute(self, array: &Array<N, M>) -> Self {
        self.array().permute(array).o_coordinate()
    }
}

impl<const N: usize, const M: Orientation> From<OrientationCoord<N, M>> for usize {
    fn from(OrientationCoord(t): OrientationCoord<N, M>) -> Self {
        t as usize
    }
}

impl<const N: usize, const M: Orientation> TryFrom<usize> for OrientationCoord<N, M> {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        (value < power(M, N - 1))
            .then(|| Self(value as OCoordWidth))
            .ok_or(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PermutationCoord<const N: usize>(PCoordWidth);

impl<const N: usize> PermutationCoord<N> {
    pub const BOUND: usize = factorial(N);

    const fn raw_array(self) -> [Element; N] {
        let PermutationCoord(mut t) = self;
        let mut pm: [Element; N] = [0; N];
        let mut ix: Element = N as Element - 2;

        // ix from N-2 to 0, with wrapping subtraction because it is unsigned
        while ix != Element::MAX {
            let r: PCoordWidth = N as PCoordWidth - ix as PCoordWidth;
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

        pm
    }

    pub fn array<const M: Orientation>(self) -> Array<N, M> {
        Array(self.raw_array().map(|x| (x, 0)))
    }

    pub fn all() -> impl Iterator<Item = Self> {
        (0..factorial(N)).map(|x| PermutationCoord(x as PCoordWidth))
    }
}

impl<const N: usize> From<PermutationCoord<N>> for usize {
    fn from(PermutationCoord(t): PermutationCoord<N>) -> Self {
        t as usize
    }
}

impl<const N: usize> TryFrom<usize> for PermutationCoord<N> {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        (value < factorial(N))
            .then(|| Self(value as PCoordWidth))
            .ok_or(())
    }
}

// Due to const generics limitations, currently the elements of interest must be the last `K`
// elements in the array representation. This should change when Rust improves const generics
// support.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CombinationCoord<const N: usize, const K: usize>(CCoordWidth);

impl<const N: usize, const K: usize> Default for CombinationCoord<N, K> {
    fn default() -> Self {
        Self((binomial(N, K) - 1) as CCoordWidth)
    }
}

impl<const N: usize, const K: usize> CombinationCoord<N, K> {
    pub const BOUND: usize = binomial(N, K);

    pub fn array<const M: Orientation>(self) -> Array<N, M> {
        let CombinationCoord(mut t) = self;
        let Array(mut cm) = Array::<N, M>::IDENTITY;
        let mut p = (N - K..N).rev();
        let mut np = (0..N - K).rev();

        let mut r = K;
        for ix in (0..N).rev() {
            let b = binomial(ix, r) as CCoordWidth;
            if t >= b {
                cm[ix].0 = p.next().unwrap() as Element;
                t -= b;
                r -= 1;
            } else {
                cm[ix].0 = np.next().unwrap() as Element;
            }
        }

        Array(cm)
    }

    pub fn all() -> impl Iterator<Item = Self> {
        (0..binomial(N, K)).map(|x| CombinationCoord(x as CCoordWidth))
    }

    pub fn permute<const M: Orientation>(self, array: &Array<N, M>) -> Self {
        self.array().permute(array).c_coordinate()
    }
}

impl<const N: usize, const K: usize> From<CombinationCoord<N, K>> for usize {
    fn from(CombinationCoord(t): CombinationCoord<N, K>) -> Self {
        t as usize
    }
}

impl<const N: usize, const K: usize> TryFrom<usize> for CombinationCoord<N, K> {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < binomial(N, K) {
            Ok(Self(value as CCoordWidth))
        } else {
            Err(())
        }
    }
}
