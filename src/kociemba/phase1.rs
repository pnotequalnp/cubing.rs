use crate::turns::FaceTurn;
use cube::definitions as def;
use cube::search::Depth;
use cube::search::Search;
use std::cmp::max;
use std::iter::FromIterator;

const C_COUNT: usize = 8;
const C_ORI: u8 = 3;
const E_COUNT: usize = 12;
const E_ORI: u8 = 2;
const S_COUNT: usize = 4;
const MOVE_COUNT: usize = 18;
pub const C_MOVES: [def::Array<C_COUNT, C_ORI>; MOVE_COUNT] = [
    def::Array::new([
        (3, 0),
        (0, 0),
        (1, 0),
        (2, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
    ]), // U
    def::Array::new([
        (2, 0),
        (3, 0),
        (0, 0),
        (1, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
    ]), // U2
    def::Array::new([
        (1, 0),
        (2, 0),
        (3, 0),
        (0, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
    ]), // U'
    def::Array::new([
        (4, 2),
        (1, 0),
        (2, 0),
        (0, 1),
        (7, 1),
        (5, 0),
        (6, 0),
        (3, 2),
    ]), // R
    def::Array::new([
        (7, 0),
        (1, 0),
        (2, 0),
        (4, 0),
        (3, 0),
        (5, 0),
        (6, 0),
        (0, 0),
    ]), // R2
    def::Array::new([
        (3, 2),
        (1, 0),
        (2, 0),
        (7, 1),
        (0, 1),
        (5, 0),
        (6, 0),
        (4, 2),
    ]), // R'
    def::Array::new([
        (1, 1),
        (5, 2),
        (2, 0),
        (3, 0),
        (0, 2),
        (4, 1),
        (6, 0),
        (7, 0),
    ]), // F
    def::Array::new([
        (5, 0),
        (4, 0),
        (2, 0),
        (3, 0),
        (1, 0),
        (0, 0),
        (6, 0),
        (7, 0),
    ]), // F2
    def::Array::new([
        (4, 1),
        (0, 2),
        (2, 0),
        (3, 0),
        (5, 2),
        (1, 1),
        (6, 0),
        (7, 0),
    ]), // F'
    def::Array::new([
        (0, 0),
        (2, 1),
        (6, 2),
        (3, 0),
        (4, 0),
        (1, 2),
        (5, 1),
        (7, 0),
    ]), // L
    def::Array::new([
        (0, 0),
        (6, 0),
        (5, 0),
        (3, 0),
        (4, 0),
        (2, 0),
        (1, 0),
        (7, 0),
    ]), // L2
    def::Array::new([
        (0, 0),
        (5, 1),
        (1, 2),
        (3, 0),
        (4, 0),
        (6, 2),
        (2, 1),
        (7, 0),
    ]), // L'
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (4, 0),
    ]), // D
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (6, 0),
        (7, 0),
        (4, 0),
        (5, 0),
    ]), // D2
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (7, 0),
        (4, 0),
        (5, 0),
        (6, 0),
    ]), // D'
    def::Array::new([
        (0, 0),
        (1, 0),
        (3, 1),
        (7, 2),
        (4, 0),
        (5, 0),
        (2, 2),
        (6, 1),
    ]), // B
    def::Array::new([
        (0, 0),
        (1, 0),
        (7, 0),
        (6, 0),
        (4, 0),
        (5, 0),
        (3, 0),
        (2, 0),
    ]), // B2
    def::Array::new([
        (0, 0),
        (1, 0),
        (6, 1),
        (2, 2),
        (4, 0),
        (5, 0),
        (7, 2),
        (3, 1),
    ]), // B'
];
pub const E_MOVES: [def::Array<E_COUNT, E_ORI>; MOVE_COUNT] = [
    def::Array::new([
        (3, 0),
        (0, 0),
        (1, 0),
        (2, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (11, 0),
    ]), // U
    def::Array::new([
        (2, 0),
        (3, 0),
        (0, 0),
        (1, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (11, 0),
    ]), // U2
    def::Array::new([
        (1, 0),
        (2, 0),
        (3, 0),
        (0, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (11, 0),
    ]), // U'
    def::Array::new([
        (8, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (11, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (4, 0),
        (9, 0),
        (10, 0),
        (0, 0),
    ]), // R
    def::Array::new([
        (4, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (0, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (11, 0),
        (9, 0),
        (10, 0),
        (8, 0),
    ]), // R2
    def::Array::new([
        (11, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (8, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (0, 0),
        (9, 0),
        (10, 0),
        (4, 0),
    ]), // R'
    def::Array::new([
        (0, 0),
        (9, 1),
        (2, 0),
        (3, 0),
        (4, 0),
        (8, 1),
        (6, 0),
        (7, 0),
        (1, 1),
        (5, 1),
        (10, 0),
        (11, 0),
    ]), // F
    def::Array::new([
        (0, 0),
        (5, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (1, 0),
        (6, 0),
        (7, 0),
        (9, 0),
        (8, 0),
        (10, 0),
        (11, 0),
    ]), // F2
    def::Array::new([
        (0, 0),
        (8, 1),
        (2, 0),
        (3, 0),
        (4, 0),
        (9, 1),
        (6, 0),
        (7, 0),
        (5, 1),
        (1, 1),
        (10, 0),
        (11, 0),
    ]), // F'
    def::Array::new([
        (0, 0),
        (1, 0),
        (10, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (9, 0),
        (7, 0),
        (8, 0),
        (2, 0),
        (6, 0),
        (11, 0),
    ]), // L
    def::Array::new([
        (0, 0),
        (1, 0),
        (6, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (2, 0),
        (7, 0),
        (8, 0),
        (10, 0),
        (9, 0),
        (11, 0),
    ]), // L2
    def::Array::new([
        (0, 0),
        (1, 0),
        (9, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (10, 0),
        (7, 0),
        (8, 0),
        (6, 0),
        (2, 0),
        (11, 0),
    ]), // L'
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (4, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (11, 0),
    ]), // D
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (6, 0),
        (7, 0),
        (4, 0),
        (5, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (11, 0),
    ]), // D2
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (7, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (11, 0),
    ]), // D'
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (11, 1),
        (4, 0),
        (5, 0),
        (6, 0),
        (10, 1),
        (8, 0),
        (9, 0),
        (3, 1),
        (7, 1),
    ]), // B
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (7, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (3, 0),
        (8, 0),
        (9, 0),
        (11, 0),
        (10, 0),
    ]), // B2
    def::Array::new([
        (0, 0),
        (1, 0),
        (2, 0),
        (10, 1),
        (4, 0),
        (5, 0),
        (6, 0),
        (11, 1),
        (8, 0),
        (9, 0),
        (7, 1),
        (3, 1),
    ]), // B'
];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Cube {
    corners: def::OrientationCoord<C_COUNT, C_ORI>,
    edges: def::OrientationCoord<E_COUNT, E_ORI>,
    slice: def::CombinationCoord<E_COUNT, S_COUNT>,
}

impl Cube {
    pub fn new(
        corners: def::OrientationCoord<C_COUNT, C_ORI>,
        edges: def::OrientationCoord<E_COUNT, E_ORI>,
        slice: def::CombinationCoord<E_COUNT, S_COUNT>,
    ) -> Self {
        Self {
            corners,
            edges,
            slice,
        }
    }

    pub fn create_table() -> Table {
        Table::new()
    }

    pub fn create_pruning_table(move_table: &Table) -> PruningTable {
        PruningTable::new(move_table)
    }
}

impl Search for Cube {
    type Edge = usize;
    type HeuristicData = PruningTable;
    type TransitionData = Table;

    fn heuristic(self, table: &Self::HeuristicData) -> cube::search::Depth {
        table.lookup(self)
    }

    fn transition(self, table: &Self::TransitionData) -> Vec<(Self, Self::Edge)> {
        (0..MOVE_COUNT)
            .map(|ix| (table.lookup(self, ix), ix))
            .collect()
    }
}

impl From<FaceTurn> for Cube {
    fn from(turn: FaceTurn) -> Self {
        let ix = match turn {
            FaceTurn::U => 0,
            FaceTurn::U2 => 1,
            FaceTurn::U3 => 2,
            FaceTurn::R => 3,
            FaceTurn::R2 => 4,
            FaceTurn::R3 => 5,
            FaceTurn::F => 6,
            FaceTurn::F2 => 7,
            FaceTurn::F3 => 8,
            FaceTurn::L => 9,
            FaceTurn::L2 => 10,
            FaceTurn::L3 => 11,
            FaceTurn::D => 12,
            FaceTurn::D2 => 13,
            FaceTurn::D3 => 14,
            FaceTurn::B => 15,
            FaceTurn::B2 => 16,
            FaceTurn::B3 => 17,
        };

        let corners = C_MOVES[ix].o_coordinate();
        let edges = E_MOVES[ix].o_coordinate();
        let slice = E_MOVES[ix].c_coordinate();

        Cube {
            corners,
            edges,
            slice,
        }
    }
}

impl FromIterator<FaceTurn> for Cube {
    fn from_iter<T: IntoIterator<Item = FaceTurn>>(iter: T) -> Self {
        let (corners, edges) = iter
            .into_iter()
            .map(usize::from)
            .map(|ix| (&C_MOVES[ix], &E_MOVES[ix]))
            .fold(
                (def::Array::default(), def::Array::default()),
                |(w, x), (y, z)| (w.permute(&y), x.permute(&z)),
            );

        Self::new(
            corners.o_coordinate(),
            edges.o_coordinate(),
            edges.c_coordinate(),
        )
    }
}

pub struct Table(
    def::OrientationTable<C_COUNT, C_ORI, MOVE_COUNT>,
    def::OrientationTable<E_COUNT, E_ORI, MOVE_COUNT>,
    def::CombinationTable<E_COUNT, S_COUNT, MOVE_COUNT>,
);

impl Table {
    pub fn new() -> Self {
        Self(
            def::OrientationTable::new(&C_MOVES),
            def::OrientationTable::new(&E_MOVES),
            def::CombinationTable::new(&E_MOVES),
        )
    }

    pub fn lookup(
        &self,
        Cube {
            corners,
            edges,
            slice,
        }: Cube,
        index: usize,
    ) -> Cube {
        let Self(c_table, e_table, s_table) = self;

        let corners = c_table.lookup(corners, index);
        let edges = e_table.lookup(edges, index);
        let slice = s_table.lookup(slice, index);

        Cube {
            corners,
            edges,
            slice,
        }
    }
}

pub struct PruningTable(
    def::OrientationPruning<C_COUNT, C_ORI, MOVE_COUNT>,
    def::OrientationPruning<E_COUNT, E_ORI, MOVE_COUNT>,
);

impl PruningTable {
    pub fn new(Table(c_table, e_table, s_table): &Table) -> Self {
        Self(
            def::OrientationPruning::new(c_table),
            def::OrientationPruning::new(e_table),
        )
    }

    pub fn lookup(
        &self,
        Cube {
            corners,
            edges,
            slice,
        }: Cube,
    ) -> Depth {
        let PruningTable(c_table, e_table) = self;

        let corners = c_table.lookup(corners);
        let edges = e_table.lookup(edges);

        max(corners, edges)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;

    #[test]
    pub fn array_cancellation() {
        for ix in (0..18).step_by(3) {
            let x1 = &C_MOVES[ix];
            let x2 = &C_MOVES[ix + 1];
            let x3 = &C_MOVES[ix + 2];
            assert_eq!(
                def::Array::default(),
                x1.permute(x3),
                "{:?}: {:?} {:?}",
                FaceTurn::from(ix),
                x1,
                x3
            );
            assert_eq!(def::Array::default(), x3.permute(x1));
            assert_eq!(def::Array::default(), x2.permute(x2));
            assert_eq!(
                def::Array::default(),
                x1.permute(x1).permute(x1).permute(x1)
            );
            assert_eq!(
                def::Array::default(),
                x3.permute(x3).permute(x3).permute(x3)
            );
            assert_eq!(def::Array::default(), x2.permute(x1).permute(x1));
            assert_eq!(def::Array::default(), x2.permute(x3).permute(x3));

            let x1 = &E_MOVES[ix];
            let x2 = &E_MOVES[ix + 1];
            let x3 = &E_MOVES[ix + 2];
            assert_eq!(
                def::Array::default(),
                x1.permute(x3),
                "{:?}: {:?} {:?}",
                FaceTurn::from(ix),
                x1,
                x3
            );
            assert_eq!(def::Array::default(), x3.permute(x1));
            assert_eq!(def::Array::default(), x2.permute(x2));
            assert_eq!(
                def::Array::default(),
                x1.permute(x1).permute(x1).permute(x1)
            );
            assert_eq!(
                def::Array::default(),
                x3.permute(x3).permute(x3).permute(x3)
            );
            assert_eq!(def::Array::default(), x2.permute(x1).permute(x1));
            assert_eq!(def::Array::default(), x2.permute(x3).permute(x3));
        }
    }

    #[test]
    pub fn zero_orientations() {
        for ix in (1..18).step_by(3) {
            let co = C_MOVES[ix].o_coordinate();
            assert_eq!(
                def::OrientationCoord::default(),
                co,
                "{:?}",
                FaceTurn::from(ix)
            );

            let eo = E_MOVES[ix].o_coordinate();
            assert_eq!(
                def::OrientationCoord::default(),
                eo,
                "{:?}: {:?}",
                FaceTurn::from(ix),
                E_MOVES[ix]
            );
        }

        for ix in 0..18 {
            let array = &C_MOVES[ix];
            assert_eq!(
                def::OrientationCoord::default(),
                array.permute(array).o_coordinate()
            );

            let array = &E_MOVES[ix];
            assert_eq!(
                def::OrientationCoord::default(),
                array.permute(array).o_coordinate()
            );
        }
    }

    #[test]
    pub fn zero_combinations() {
        assert_eq!(
            def::CombinationCoord::try_from(0).unwrap(),
            def::Array::<E_COUNT, E_ORI>::new([
                (8, 0),
                (9, 0),
                (10, 0),
                (11, 0),
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0)
            ])
            .c_coordinate::<S_COUNT>()
        );
    }

    #[test]
    pub fn max_combinations() {
        for ix in (1..18).step_by(3) {
            let cm = E_MOVES[ix].c_coordinate::<S_COUNT>();
            assert_eq!(
                def::CombinationCoord::default(),
                cm,
                "{:?}: {:?}",
                FaceTurn::from(ix),
                E_MOVES[ix]
            );
        }

        for ix in [0, 2, 12, 14].iter() {
            let array = &E_MOVES[*ix];
            assert_eq!(
                def::CombinationCoord::default(),
                array.c_coordinate::<S_COUNT>(),
            );
        }

        for ix in [3, 5, 6, 8, 9, 11, 15, 17].iter() {
            let array = &E_MOVES[*ix];
            assert_ne!(
                def::CombinationCoord::default(),
                array.c_coordinate::<S_COUNT>(),
                "{}: {:?}",
                FaceTurn::from(*ix),
                array.c_coordinate::<S_COUNT>()
            );
        }
    }

    #[test]
    pub fn max_orientations() {
        let array = def::Array::<8, 3>::create([
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 1),
        ])
        .unwrap();
        let coord = def::OrientationCoord::try_from(2186).unwrap();
        assert_eq!(coord, array.o_coordinate());
        assert_eq!(array, coord.array());

        let array = def::Array::<12, 2>::create([
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 1),
            (11, 1),
        ])
        .unwrap();
        let coord = def::OrientationCoord::try_from(2047).unwrap();
        assert_eq!(coord, array.o_coordinate());
        assert_eq!(array, coord.array());
    }
}
