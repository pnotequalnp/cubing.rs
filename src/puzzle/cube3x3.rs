use crate::core::definitions as def;
use crate::metric::htm::{Corners, Edges};
use crate::metric::Htm;
use std::iter::FromIterator;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Cube3x3 {
    pub corners: Corners,
    pub edges: Edges,
}

impl Cube3x3 {
    pub const fn new(corners: Corners, edges: Edges) -> Self {
        Self { corners, edges }
    }

    pub const fn apply(&self, htm: Htm) -> Self {
        let Self { corners, edges } = self;

        let corners = corners.permute(htm.to_corners());
        let edges = edges.permute(htm.to_edges());

        Self { corners, edges }
    }

    pub const fn apply_slice(self, slice: &[Htm]) -> Self {
        let mut state = self;

        let mut ix = 0;
        while ix < slice.len() {
            state = state.apply(slice[ix]);
            ix += 1;
        }

        state
    }

    pub fn apply_seq(&self, sequence: impl IntoIterator<Item = Htm>) -> Self {
        sequence
            .into_iter()
            .fold(self.clone(), |cube, turn| cube.apply(turn))
    }

    pub const fn from_slice(slice: &[Htm]) -> Self {
        let start = Self::new(Corners::IDENTITY, Edges::IDENTITY);
        start.apply_slice(slice)
    }

    pub fn random_state() -> Self {
        Self::new(def::Array::random(), def::Array::random())
    }
}

impl From<Htm> for Cube3x3 {
    fn from(htm: Htm) -> Self {
        let corners: &Corners = htm.into();
        let edges: &Edges = htm.into();

        Self::new(corners.clone(), edges.clone())
    }
}

impl FromIterator<Htm> for Cube3x3 {
    fn from_iter<T: IntoIterator<Item = Htm>>(iter: T) -> Self {
        let (corners, edges) = iter
            .into_iter()
            .map(|htm| -> (&Corners, &Edges) { (htm.into(), htm.into()) })
            .fold((Corners::default(), Edges::default()), |(w, x), (y, z)| {
                (w.permute(&y), x.permute(&z))
            });

        Self::new(corners, edges)
    }
}
