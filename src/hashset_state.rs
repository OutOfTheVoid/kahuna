use std::{collections::HashSet, hash::Hash};

use crate::{SetState, State};

/// A state type which represents possible states with a hash set.
///
/// * `T` - The underlying unique state identifier
#[derive(Clone, PartialEq)]
pub struct HashsetState<T: Eq + Hash> {
    pub hashset: HashSet<T>,
}

#[allow(unused)]
impl<T: Eq + Hash + Clone> HashsetState<T> {
    /// Creates a new HashsetState with just the final state `state` inside
    pub fn new_final(state: &T) -> Self {
        let mut hashset = HashSet::new();
        hashset.insert(state.clone());
        Self { hashset }
    }

    /// Creates a new HashsetState which has each state in `states` inside
    pub fn new(states: &[T]) -> Self {
        let mut hashset = HashSet::new();
        states
            .iter()
            .map(|x| hashset.insert(x.clone()))
            .for_each(drop);
        Self { hashset }
    }
}

impl<T: Clone + Eq + Hash> State for HashsetState<T> {
    fn entropy(&self) -> u32 {
        self.hashset.len() as u32 - 1
    }
}

impl<T: Clone + Eq + Hash> SetState for HashsetState<T> {
    fn has_any_of(&self, states: &Self) -> bool {
        !self.hashset.is_disjoint(&states.hashset)
    }

    fn clear_states(&mut self, states: &Self) {
        self.hashset.retain(|x| !states.hashset.contains(x));
    }

    fn set_states(&mut self, states: &Self) {
        for state in states.hashset.iter() {
            self.hashset.insert(state.clone());
        }
    }

    fn collect_final_states(&self, states: &mut Vec<Self>) {
        self.hashset
            .iter()
            .map(|x| {
                states.push(Self::new_final(x));
            })
            .for_each(drop);
    }
}
