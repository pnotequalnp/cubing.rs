pub mod cube3x3;
pub mod moves;
pub mod positions;

use crate::util::count;

pub use cube3x3::Cube3x3;
pub use moves::*;

pub const CORNERS: usize = 8;
pub const TWISTS: u8 = 3;
pub const EDGES: usize = 12;
pub const FLIPS: u8 = 2;
pub const BELT_EDGES: usize = 4;
pub const MOVE_COUNT: usize = 18;
pub const GENERATORS: [usize; MOVE_COUNT] = count::<MOVE_COUNT>();
