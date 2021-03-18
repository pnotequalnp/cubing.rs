use alloc::vec::Vec;

pub type Depth = u8;

pub trait Search: Copy + Default + Eq + Sized {
    type Edge: Copy;
    type HeuristicData;
    type TransitionData;
    type Iter: Iterator<Item = (Self, Self::Edge)>;

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
    fn transition(self, data: &Self::TransitionData) -> Self::Iter; //Vec<(Self, Self::Edge)>;

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

pub fn ida_iter<'a, T: 'a + Search>(
    start: T,
    heuristic_data: &'a T::HeuristicData,
    transition_data: &'a T::TransitionData,
    max_depth: Option<Depth>,
) -> impl Iterator<Item = Vec<(T, T::Edge)>> + 'a {
    let goal = T::default();
    let max = max_depth.unwrap_or(Depth::MAX);

    (0..=max).flat_map(move |depth| dfs_iter(start, goal, heuristic_data, transition_data, depth))
}

fn dfs_iter<'a, T: Search>(
    start: T,
    goal: T,
    heuristic_data: &'a T::HeuristicData,
    transition_data: &'a T::TransitionData,
    depth: Depth,
) -> DFSIterator<'a, T> {
    let future = start.transition(transition_data);
    DFSIterator {
        goal,
        future,
        heuristic_data,
        transition_data,
        current_depth: 0,
        target_depth: depth,
        path: Vec::new(),
    }
}

pub struct DFSIterator<'a, T: Search> {
    goal: T,
    future: T::Iter,
    heuristic_data: &'a T::HeuristicData,
    transition_data: &'a T::TransitionData,
    current_depth: Depth,
    target_depth: Depth,
    path: Vec<(T, T::Edge, T::Iter)>,
}

impl<T: Search> DFSIterator<'_, T> {
    fn scan(&mut self) -> Option<<Self as Iterator>::Item> {
        let goal = self.goal;
        if let Some((vertex, edge)) = self.future.find(|(v, _)| *v == goal) {
            let mut path = self
                .path
                .iter()
                .map(|(x, y, _)| (*x, *y))
                .collect::<Vec<_>>();
            path.push((vertex, edge));
            return Some(path);
        }
        None
    }

    fn advance(&mut self) {
        while self.current_depth + 1 < self.target_depth {
            let target = self.target_depth;
            let current = self.current_depth;
            let data = self.heuristic_data;
            if let Some((vertex, edge)) = self
                .future
                .find(|(v, _)| v.heuristic(data) + current < target)
            {
                let past =
                    core::mem::replace(&mut self.future, vertex.transition(self.transition_data));
                self.path.push((vertex, edge, past));
                self.current_depth += 1;
            } else if let Some((_vertex, _edge, past)) = self.path.pop() {
                self.future = past;
                self.current_depth -= 1;
            } else {
                break;
            }
        }
    }
}

impl<T: Search> Iterator for DFSIterator<'_, T> {
    type Item = Vec<(T, T::Edge)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance();

        loop {
            if let Some(path) = self.scan() {
                break Some(path);
            } else if let Some((_vertex, _edge, past)) = self.path.pop() {
                self.future = past;
                self.current_depth -= 1;
                self.advance();
            } else {
                break None;
            }
        }
    }
}
