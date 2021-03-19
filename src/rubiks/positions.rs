use super::Cube3x3;
use super::FaceTurn::*;

pub const SUPER_FLIP: Cube3x3 = Cube3x3::from_slice(&[
    U, R2, F, B, R, B2, R, U2, L, B2, R, U3, D3, R2, F, R3, L, B2, U2, F2,
]);
