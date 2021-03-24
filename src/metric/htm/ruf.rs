use super::Htm;
use crate::core::definitions as def;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HtmRuf {
    U1, U2, U3, R1, R2, R3, F1, F2, F3
}

type Rotation = def::Array<4, 1>;

const ROT_X: Rotation = def::Array::new([(1, 0), (4, 0), (2, 0), (3, 0), (5, 0), (0, 0)]);
const ROT_Y: Rotation = def::Array::new([(0, 0), (2, 0), (5, 0), (1, 0), (4, 0), (3, 0)]);
const ROT_Z: Rotation = ROT_X
    .permute(&ROT_Y)
    .permute(&ROT_X)
    .permute(&ROT_X)
    .permute(&ROT_X);

const fn rotate(axis: usize, direction: usize) -> Rotation {
    let mut rot = match axis % 3 {
        0 => ROT_Y,
        1 => ROT_X,
        2 => ROT_Z,
        _ => unreachable!(),
    };

    let mut x = 0;
    while x < direction % 4 {
        rot = rot.permute(&rot);
        x += 1;
    }

    rot
}

pub struct HtmRufIterator<Iter> {
    orientation: Rotation,
    iter: Iter,
}

impl<I: Iterator<Item = Htm>> Iterator for HtmRufIterator<I> {
    type Item = HtmRuf;

    fn next(&mut self) -> Option<Self::Item> {
        let htm = self.iter.next()?;
        let (ruf, orientation) = HtmRuf::reorient(htm, &self.orientation);
        self.orientation = orientation;
        Some(ruf)
    }
}

impl HtmRuf {
    pub fn from_htm(iter: impl Iterator<Item = Htm>) -> impl Iterator<Item = Self> {
        HtmRufIterator {
            orientation: Rotation::default(),
            iter,
        }
    }

    fn reorient(htm: Htm, orientation: &Rotation) -> (Self, Rotation) {
        #[rustfmt::ignore]
        let rotation = match htm {
            Htm::U1 | Htm::U2 | Htm::U3 |
            Htm::R1 | Htm::R2 | Htm::R3 |
            Htm::F1 | Htm::F2 | Htm::F3 => rotate(0, 0),
            Htm::L1 => rotate(1, 1),
            Htm::L2 => rotate(1, 2),
            Htm::L3 => rotate(1, 3),
            Htm::D1 => rotate(0, 1),
            Htm::D2 => rotate(0, 2),
            Htm::D3 => rotate(0, 3),
            Htm::B1 => rotate(2, 1),
            Htm::B2 => rotate(2, 2),
            Htm::B3 => rotate(2, 3),
        };

        let ruf = match htm {
            Htm::U1 | Htm::D1 => HtmRuf::U1,
            Htm::U2 | Htm::D2 => HtmRuf::U2,
            Htm::U3 | Htm::D3 => HtmRuf::U3,
            Htm::R1 | Htm::L1 => HtmRuf::R1,
            Htm::R2 | Htm::L2 => HtmRuf::R2,
            Htm::R3 | Htm::L3 => HtmRuf::R3,
            Htm::F1 | Htm::B1 => HtmRuf::F1,
            Htm::F2 | Htm::B2 => HtmRuf::F2,
            Htm::F3 | Htm::B3 => HtmRuf::F3,
        };

        let corners = def::Array::from(htm);

        let orientation = orientation.permute(rotation);

        todo!()
    }
}
