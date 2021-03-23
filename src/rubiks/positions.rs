use super::Cube3x3;
use crate::notation::HTM::*;

pub const SUPER_FLIP: Cube3x3 = Cube3x3::from_slice(&[
    U1, R2, F1, B1, R1, B2, R1, U2, L1, B2, R1, U3, D3, R2, F1, R3, L1, B2, U2, F2,
]);
