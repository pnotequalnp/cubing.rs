use alloc::vec::Vec;
use core::cmp::Ordering::{Equal, Greater, Less};

pub type Depth = u8;

pub trait Search: Copy + Default + Eq + Sized {
    type Edge: Copy;
    type HeuristicData;
    type TransitionData;

    /// A domain-specific heuristic which gives a *lower bound* on the distance from any vertex to
    /// the goal vertex. If this is not a true lower bound, then suboptimal paths may be
    /// returned.
    ///
    /// # Arguments
    ///
    /// * `data` - Any required data for the heuristic function, such as a pre-computed table
    fn heuristic(self, data: &Self::HeuristicData) -> Depth;

    /// A transition function which calculates the next vertices of the graph to search given the
    /// current vertex.
    fn transition(self, data: &Self::TransitionData) -> Vec<(Self, Self::Edge)>;

    /// A basic IDA* implementation, if the provided heuristic is a true lower bound, the paths it
    /// finds are the shortest possible.
    fn ida_star(
        &self,
        heuristic_data: &Self::HeuristicData,
        transition_data: &Self::TransitionData,
        max_depth: Depth,
    ) -> Option<Vec<(Self, Self::Edge)>> {
        let goal = Self::default();
        (0..max_depth).find_map(|depth| self.dfs(goal, heuristic_data, transition_data, 0, depth))
    }

    /// A depth-specific DFS implementation intended as a subroutine for IDA*.
    fn dfs(
        &self,
        goal: Self,
        heuristic_data: &Self::HeuristicData,
        transition_data: &Self::TransitionData,
        depth: Depth,
        max_depth: Depth,
    ) -> Option<Vec<(Self, Self::Edge)>> {
        match (depth + self.heuristic(heuristic_data)).cmp(&max_depth) {
            Greater => None,
            Equal => (*self == goal).then(|| Vec::new()),
            Less => self.transition(transition_data).into_iter().find_map(|(vertex, edge)| {
                let mut path =
                    vertex.dfs(goal, heuristic_data, transition_data, depth + 1, max_depth)?;
                path.push((*self, edge));
                Some(path)
            }),
        }
    }
}
