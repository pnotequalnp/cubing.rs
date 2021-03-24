use crate::core::definitions as def;

type Permutation = def::PermutationCoord<8>;
type Orientation = def::OrientationCoord<8, 3>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cube2x2 {
    pub permutation: Permutation,
    pub orientation: Orientation,
}

impl Cube2x2 {
    pub fn new(permutation: Permutation, orientation: Orientation) -> Self {
        Self {
            permutation,
            orientation,
        }
    }

    pub fn random_state() -> Self {
        let array = def::Array::random();
        Self::new(array.p_coordinate(), array.o_coordinate())
    }
}
