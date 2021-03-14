use crate::util::factorial;
use core::ops::Index;

type Element = u8;
type CoordWidth = u32;

pub const fn upscale(x: u8) -> usize {
    x as usize
}

pub const fn upscale16(x: u8) -> u16 {
    x as u16
}

pub const fn upscale32(x: u8) -> u32 {
    x as u32
}

// HACK: Can be simplified once const generics are improved upstream
pub struct PermutationArray<const COUNT: u8>([Element; upscale(COUNT)])
where
    [Element; upscale(COUNT)]: Sized;

impl<const COUNT: u8> PermutationArray<COUNT>
where
    [Element; upscale(COUNT)]: Sized,
{
    pub const DEFAULT: [Element; upscale(COUNT)] = {
        let mut pm = [0; upscale(COUNT)];
        let mut ix = 1u8;

        while ix < COUNT as u8 {
            pm[ix as usize] = ix;
            ix += 1;
        }

        pm
    };

    pub const fn new(candidate: [Element; upscale(COUNT)]) -> Self {
        let mut ix: usize = 0;
        while ix < upscale(COUNT) - 1 {
            let mut jx: usize = ix + 1;
            while jx < upscale(COUNT) {
                if candidate[ix] == candidate[jx] {
                    panic!("Not a valid permutation array");
                }
                jx += 1;
            }
            ix += 1;
        }

        PermutationArray(candidate)
    }

    pub const fn permute(&self, permutation: &Self) -> Self {
        let mut pm = Self::DEFAULT;
        let mut ix: usize = 0;

        while ix < upscale(COUNT) {
            let jx = permutation.0[ix] as usize;
            pm[ix] = self.0[jx];
            ix += 1;
        }

        PermutationArray(pm)
    }

    pub const fn coordinate(&self) -> Coordinate<COUNT> {
        let mut t: CoordWidth = 0;
        let mut ix: CoordWidth = 0;

        while ix < upscale32(COUNT) {
            t *= upscale32(COUNT) - ix + 2;
            let mut jx: usize = ix as usize + 1;

            while jx < upscale(COUNT) {
                if self.0[ix as usize] > self.0[jx] {
                    t += 1;
                };
                jx += 1;
            }

            ix += 1;
        }

        Coordinate(t)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Coordinate<const COUNT: u8>(pub(crate) CoordWidth);

impl<const COUNT: u8> Coordinate<COUNT>
where
    [Element; upscale(COUNT)]: Sized,
{
    pub const MAX: usize = factorial(COUNT) - 1;
    pub const BOUND: CoordWidth = factorial(COUNT) as CoordWidth;

    pub fn all() -> impl Iterator<Item = Self> {
        (0..Self::BOUND).map(Coordinate)
    }

    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub const fn permutation_array(&self) -> PermutationArray<COUNT> {
        let mut t = self.0;
        let mut pm: [Element; upscale(COUNT)] = [0; upscale(COUNT)];
        let mut ix: Element = COUNT - 1;

        // ix from COUNT-1 to 0
        while ix != Element::MAX {
            let r: CoordWidth = upscale32(COUNT) - ix as CoordWidth + 2;
            pm[ix as usize] = 0 + (t % r) as Element;
            t /= r;

            let mut jx: usize = ix as usize + 1;
            while jx < upscale(COUNT) {
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

// Can't currently index over the array of generators itself due to const generics restrictions
pub struct MoveTable<const COUNT: u8, const GENERATORS: usize>(
    pub(crate) [[Coordinate<COUNT>; factorial(COUNT)]; GENERATORS],
)
where
    [u8; factorial(COUNT)]: Sized;

pub struct Generators<const COUNT: usize>(pub(crate) [usize; COUNT]);

impl<const COUNT: usize> Generators<COUNT> {
    pub fn iter(&self) -> core::slice::Iter<usize> {
        self.0.iter()
    }
}

impl<const COUNT: u8, const GENERATORS: usize> MoveTable<COUNT, GENERATORS>
where
    [u8; upscale(COUNT)]: Sized,
    [u8; factorial(COUNT)]: Sized,
{
    pub const fn new(
        generators: &[PermutationArray<COUNT>; GENERATORS],
    ) -> (Self, Generators<GENERATORS>) {
        let mut table = [[Coordinate::<COUNT>(0); factorial(COUNT)]; GENERATORS];

        let mut position: CoordWidth = 0;
        while (position as usize) < Coordinate::<COUNT>::MAX {
            let mut gen: usize = 0;
            while gen < GENERATORS {
                let pm = Coordinate::<COUNT>(position)
                    .permutation_array()
                    .permute(&generators[gen]);

                table[gen][position as usize] = pm.coordinate();

                gen += 1;
            }
            position += 1;
        }

        let mut generators: [usize; GENERATORS] = [0; GENERATORS];
        let mut ix: usize = 0;
        while ix < GENERATORS {
            generators[ix] = ix;
            ix += 1;
        }

        (MoveTable(table), Generators(generators))
    }
}

impl<const COUNT: u8, const GENERATORS: usize> Index<(Coordinate<COUNT>, usize)>
    for MoveTable<COUNT, GENERATORS>
where
    [u8; factorial(COUNT)]: Sized,
{
    type Output = Coordinate<COUNT>;

    fn index(
        &self,
        (Coordinate(position), generator): (Coordinate<COUNT>, usize),
    ) -> &Self::Output {
        &self.0[generator][position as usize]
    }
}
