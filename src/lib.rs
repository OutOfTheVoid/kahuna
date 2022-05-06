//! Wave Function Collapse
//! 
//! Provides an implementation of the wave function collapse algorithm.
//!
//! Wave function collapse works by iteratively "collapsing" a collecion of
//! cells (such as a square grid) from all possible states to only the states
//! possible with a given ruleset, selecting randomly where ambiguous.

mod space;
mod state;
mod collapse_rule;
pub mod square_grid;

use std::{collections::{BTreeSet, HashSet}};

use rand::{thread_rng, Rng};
pub use space::*;
pub use state::*;
pub use collapse_rule::*;

fn find_next_to_collapse<St: State, Sp: Space<St>>(unresoved_set: &mut HashSet<Sp::Coordinate>, lowest_entropy_set: &mut Vec<Sp::Coordinate>, resolved_set: &mut HashSet<Sp::Coordinate>, space: &Sp) -> Option<Sp::Coordinate> {
	let mut lowest_entropy = std::u32::MAX;
	lowest_entropy_set.clear();
	resolved_set.clear();
	for unresolved in unresoved_set.iter() {
		let entropy = space[*unresolved].entropy();
		if entropy == 0 {
			resolved_set.insert(*unresolved);
		} else if entropy < lowest_entropy {
			lowest_entropy = entropy;
			lowest_entropy_set.clear();
			lowest_entropy_set.push(*unresolved);
		} else if entropy == lowest_entropy {
			lowest_entropy_set.push(*unresolved);
		}
	}
	unresoved_set.retain(|x| !resolved_set.contains(x));
	if lowest_entropy_set.len() == 0 {
		return None;
	} else {
		Some(lowest_entropy_set[thread_rng().gen_range(0..lowest_entropy_set.len())])
	}
}

/// Perform the wave function collapse algorithm on a given state-space with
/// the provided collapse rule.
/// 
/// Since rules are defined on state and space, you can usually call
/// `collapse::<Rule, _, _>(..)`
pub fn collapse<Rule: CollapseRule<St, Sp>, St: State, Sp: Space<St>>(space: &mut Sp) {
	let mut unresolved_set = HashSet::new();
	let mut resolved_set = HashSet::new();
	let mut lowest_entropy_set = Vec::new();
	for coord in &space.coordinate_list()[..] {
		unresolved_set.insert(*coord);
	}
	let mut neighbors = vec![None; Rule::NEIGHBOR_DIRECTIONS.len()].into_boxed_slice();
	let mut neighbor_states = vec![Option::<St>::None; Rule::NEIGHBOR_DIRECTIONS.len()].into_boxed_slice();
	let mut to_propogate = BTreeSet::new();
	while let Some(to_collapse) = find_next_to_collapse(&mut unresolved_set, &mut lowest_entropy_set, &mut resolved_set, space) {
		to_propogate.clear();
		space.neighbors(to_collapse, Rule::NEIGHBOR_DIRECTIONS, &mut neighbors);
		for i in 0 .. Rule::NEIGHBOR_DIRECTIONS.len() {
			neighbor_states[i] = neighbors[i].map(|coord| space[coord].clone());
		}
		Rule::observe(&mut space[to_collapse], &neighbor_states[..]);
		for i in 0..Rule::NEIGHBOR_DIRECTIONS.len() {
			if let Some(neighbor_coord) = neighbors[i] {
				to_propogate.insert(neighbor_coord);
			}
		}
		while let Some(&propogating) = to_propogate.iter().next() {
			to_propogate.remove(&propogating);
			let entropy_before = space[propogating].entropy();
			
			if entropy_before != 0 {
				space.neighbors(propogating, Rule::NEIGHBOR_DIRECTIONS, &mut neighbors);
				for i in 0 .. Rule::NEIGHBOR_DIRECTIONS.len() {
					neighbor_states[i] = neighbors[i].map(|coord| space[coord].clone());
				}
				Rule::collapse(&mut space[propogating], &neighbor_states[..]);
				let entropy_after = space[propogating].entropy();
				
				if entropy_after < entropy_before {
					for i in 0 .. Rule::NEIGHBOR_DIRECTIONS.len() {
						if let Some(neighbor) = neighbors[i] {
							if space[neighbor].entropy() != 0 {
								to_propogate.insert(neighbor);
							}
						}
					}
				}
			}
		}
	}
}
