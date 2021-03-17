use alloc::vec::Vec;

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
        on_depth_completion: Option<impl Fn(Depth)>,
    ) -> Option<(Vec<Self>, Vec<Self::Edge>)> {
        let goal = Self::default();
        let (mut path, mut edges) = (0..=max_depth).find_map(|depth| {
            let res = self.dfs(goal, heuristic_data, transition_data, 0, depth);
            if let None = res {
                if let Some(f) = &on_depth_completion {
                    f(depth);
                };
            };
            res
        })?;
        path.reverse();
        edges.reverse();
        Some((path, edges))
    }

    /// A depth-specific DFS implementation intended as a subroutine for IDA*.
    fn dfs(
        &self,
        goal: Self,
        heuristic_data: &Self::HeuristicData,
        transition_data: &Self::TransitionData,
        depth: Depth,
        max_depth: Depth,
    ) -> Option<(Vec<Self>, Vec<Self::Edge>)> {
        if *self == goal {
            Some((vec![*self], Vec::new()))
        } else if depth + self.heuristic(heuristic_data) < max_depth {
            self.transition(transition_data)
                .into_iter()
                .find_map(|(vertex, edge)| {
                    let (mut path, mut edges) =
                        vertex.dfs(goal, heuristic_data, transition_data, depth + 1, max_depth)?;
                    path.push(*self);
                    edges.push(edge);
                    Some((path, edges))
                })
        } else {
            None
        }
    }
}
