use alloc::vec::Vec;
use core::cmp::Ordering::{Equal, Greater, Less};
use core::ops::Index;

pub trait Search: Copy + Default + Eq + Sized {
    type Generator: Copy;
    type Generators;
    type GenIter: Iterator<Item = Self::Generator>;
    type TransitionTable: Index<(Self, Self::Generator), Output = Self>;
    type HeuristicTable: Index<Self, Output = usize>;

    fn gens(generators: &Self::Generators) -> Self::GenIter;

    fn ida_star(
        &self,
        generators: &Self::Generators,
        transition_table: &Self::TransitionTable,
        heuristic_table: &Self::HeuristicTable,
        max_depth: usize,
    ) -> Option<Vec<Self::Generator>> {
        let goal = Self::default();
        (0..max_depth).find_map(|depth| {
            self.dfs(
                goal,
                generators,
                transition_table,
                heuristic_table,
                0,
                depth,
            )
        })
    }

    fn dfs(
        &self,
        goal: Self,
        generators: &Self::Generators,
        transition_table: &Self::TransitionTable,
        heuristic_table: &Self::HeuristicTable,
        depth: usize,
        max_depth: usize,
    ) -> Option<Vec<Self::Generator>> {
        match (depth + heuristic_table[*self]).cmp(&max_depth) {
            Greater => None,
            Equal => (*self == goal).then(|| Vec::new()),
            Less => Self::gens(generators).find_map(|generator| {
                let next_depth = transition_table[(*self, generator)].dfs(
                    goal,
                    generators,
                    transition_table,
                    heuristic_table,
                    depth + 1,
                    max_depth,
                );
                match next_depth {
                    None => None,
                    Some(mut v) => {
                        v.push(generator);
                        Some(v)
                    }
                }
            }),
        }
    }
}

#[cfg(feature = "std")]
pub trait ParallelSearch: Search + Sync {
    type GenParIter: rayon::iter::ParallelIterator<Item = Self::Generator>;

    fn par_gens(generators: &Self::Generators) -> Self::GenParIter;

    fn ida_star(
        &self,
        generators: &Self::Generators,
        transition_table: &Self::TransitionTable,
        heuristic_table: &Self::HeuristicTable,
        max_depth: usize,
    ) -> Option<Vec<Self::Generator>>
    where
        Self::HeuristicTable: Sync,
        Self::TransitionTable: Sync,
        Self::Generators: Sync,
        Self::Generator: Send,
    {
        let goal = Self::default();
        (0..max_depth).find_map(|depth| {
            self.dfs_parallel(
                goal,
                generators,
                transition_table,
                heuristic_table,
                0,
                depth,
            )
        })
    }

    fn dfs_parallel(
        &self,
        goal: Self,
        generators: &Self::Generators,
        transition_table: &Self::TransitionTable,
        heuristic_table: &Self::HeuristicTable,
        depth: usize,
        max_depth: usize,
    ) -> Option<Vec<Self::Generator>>
    where
        Self::HeuristicTable: Sync,
        Self::TransitionTable: Sync,
        Self::Generators: Sync,
        Self::Generator: Send,
    {
        match (depth + heuristic_table[*self]).cmp(&max_depth) {
            Greater => None,
            Equal => (*self == goal).then(|| Vec::new()),
            Less => {
                let iter = Self::par_gens(generators);

                <Self::GenParIter as rayon::iter::ParallelIterator>::find_map_any(iter, |generator| {
                    let next_depth = transition_table[(*self, generator)].dfs(
                        goal,
                        generators,
                        transition_table,
                        heuristic_table,
                        depth + 1,
                        max_depth,
                    );
                    match next_depth {
                        None => None,
                        Some(mut v) => {
                            v.push(generator);
                            Some(v)
                        }
                    }
                })
            }
        }
    }
}
