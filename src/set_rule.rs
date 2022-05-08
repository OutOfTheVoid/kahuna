use rand::{thread_rng, Rng};
use crate::{SetState, State, Space, AllState, CollapseRule, InvertDelta};

pub trait SetCollapseObserver<S: State> {
	fn observe(&self, cell: &mut S, neighbors: &[Option<S>]);
}

#[derive(Clone)]
pub struct UniformSetCollapseObserver;

impl<S: SetState + State + Clone> SetCollapseObserver<S> for UniformSetCollapseObserver {
    fn observe(&self, cell: &mut S, _: &[Option<S>]) {
        let mut final_states = Vec::new();
		cell.collect_final_states(&mut final_states);
		*cell = final_states[thread_rng().gen_range(0..final_states.len())].clone();
    }
}

pub struct SetCollapseRule<S: SetState + State + Sized, Sp: Space<S>, O: SetCollapseObserver<S>> {
	neighbor_offsets: Box<[Sp::CoordinateDelta]>,
	state_rules: Box<[(S, Box<[Option<S>]>)]>,
	observer: O,
}

struct StateRule<S> {
	state: S,
	allowed_neighbors: Vec<Option<S>>
}

impl<S: SetState + Clone> StateRule<S> {
	fn add_allowed(&mut self, neighbor_index: usize, allowed: &S) {
		while self.allowed_neighbors.len() <= neighbor_index {
			self.allowed_neighbors.push(None);
		}
		if let Some(allowed_neighbors) = &mut self.allowed_neighbors[neighbor_index] {
			allowed_neighbors.set_states(&allowed);
		} else {
			self.allowed_neighbors[neighbor_index] = Some(allowed.clone());
		}
	}
}

/// builder for [SetCollapseRule]
/// 
/// Automatically collects used coordinate deltas and manages creating symmetric rules from asymmetric definitions
pub struct SetCollapseRuleBuilder<S: SetState + State, Sp: Space<S>, O: SetCollapseObserver<S> + Clone> {
	neighbor_offsets: Vec<Sp::CoordinateDelta>,
	state_rules: Vec<StateRule<S>>,
	observer: O
}

impl<S: AllState + SetState + State + PartialEq, Sp: Space<S>, O: SetCollapseObserver<S> + Clone> SetCollapseRuleBuilder<S, Sp, O>
	where Sp::CoordinateDelta: Eq + Clone + InvertDelta {
	pub fn new(observer: O) -> Self {
		Self {
			neighbor_offsets: Vec::new(),
			state_rules: Vec::new(),
			observer
		}
	}
	
	/// Set the allowed neighbors for a cell based on their coordinate deltas
	/// 
	/// This will create symmetric rules. For example, if you set state A to be
	/// allowed to the left of B, then state B will be allowed to the right of
	/// A - you don't have to explicitly set both rules.
	/// 
	/// States which do not have any allowed neighbors for a given coordinate
	/// delta will equire that those coordinates are outside of world-space.
	pub fn allow(mut self, state: &S, neighbors: &[(Sp::CoordinateDelta, S)]) -> Self {
		let mut states = Vec::new();
		state.collect_final_states(&mut states);
		for state in states {
			for (delta, neighbor) in neighbors {
				let mut neighbor_states = Vec::new();
				neighbor.collect_final_states(&mut neighbor_states);
				for n_state in neighbor_states {
					self.allow_symmetric(&state, &n_state, delta);
				}
			}
		}
		self
	}
	
	fn allow_symmetric(&mut self, a: &S, b: &S, offset: &Sp::CoordinateDelta) {
		let offset_index = self.get_offset_index(offset.clone());
		self.get_rule(a).add_allowed(offset_index, b);
		let offset_index = self.get_offset_index(offset.invert_delta());
		self.get_rule(b).add_allowed(offset_index, a);
	}
	
	fn get_offset_index(&mut self, offset: Sp::CoordinateDelta) -> usize {
		for i in 0 .. self.neighbor_offsets.len() {
			if self.neighbor_offsets[i] == offset {
				return i
			}
		}
		let i = self.neighbor_offsets.len();
		self.neighbor_offsets.push(offset);
		i
	}
	
	fn get_rule(&mut self, state: &S) -> &mut StateRule<S> {
		for i in 0 .. self.state_rules.len() {
			if &self.state_rules[i].state == state {
				return &mut self.state_rules[i];
			}
		}
		self.state_rules.push(StateRule {
			state: state.clone(),
			allowed_neighbors: Vec::new()
		});
		let index = self.state_rules.len() - 1;
		&mut self.state_rules[index]
	}
	
	pub fn build(self) -> SetCollapseRule<S, Sp, O> {
		let mut state_rules = Vec::new();
		let mut remaining_state = S::all();
		for mut proto_rule in self.state_rules {
			while proto_rule.allowed_neighbors.len() < self.neighbor_offsets.len() {
				proto_rule.allowed_neighbors.push(None);
			}
			remaining_state.clear_states(&proto_rule.state);
			state_rules.push((proto_rule.state, proto_rule.allowed_neighbors.into_boxed_slice()));
		}
		let mut remaining_states = Vec::new();
		remaining_state.collect_final_states(&mut remaining_states);
		for remaining_state in remaining_states {
			state_rules.push((remaining_state, vec![None; self.neighbor_offsets.len()].into_boxed_slice()));
		}
		SetCollapseRule {
			neighbor_offsets: self.neighbor_offsets.into_boxed_slice(),
			state_rules: state_rules.into_boxed_slice(), 
			observer: self.observer
		}
	}
}

/// A collapse rule implementation that works with implementors of [crate::SetState]
impl<S: SetState + State, Sp: Space<S>, O: SetCollapseObserver<S>> CollapseRule<S, Sp> for SetCollapseRule<S, Sp, O>
	where Sp::CoordinateDelta: Clone {
    fn neighbor_offsets(&self) -> Box<[<Sp as Space<S>>::CoordinateDelta]> {
        self.neighbor_offsets.clone()
    }

    fn collapse(&self, cell: &mut S, neighbors: &[Option<S>]) {
        for (state, allowed_neighbors) in &self.state_rules[..] {
			if cell.has_any_of(state) {
				for i in 0..neighbors.len() {
					if let Some(neighbor_state) = &neighbors[i] {
						let allow = if let Some(allowed_state) = &allowed_neighbors[i] {
							neighbor_state.has_any_of(allowed_state)
						} else {
							false
						};
						if ! allow {
							cell.clear_states(state)
						}
					}
				}
			}
		}
    }

    fn observe(&self, cell: &mut S, neighbors: &[Option<S>]) {
        self.observer.observe(cell, neighbors);
    }
}

