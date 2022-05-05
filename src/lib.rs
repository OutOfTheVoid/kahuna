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

pub fn collapse<St: State, Sp: Space<St>, Rule: CollapseRule<St>>(space: &mut Sp, rule: &Rule) {
	let mut unresolved_set = HashSet::new();
	let mut resolved_set = HashSet::new();
	let mut lowest_entropy_set = Vec::new();
	for coord in &space.coordinate_list()[..] {
		unresolved_set.insert(*coord);
	}
	let mut neighbors = vec![None; Sp::NEIGHBOR_DIRECTIONS].into_boxed_slice();
	let mut neighbor_states = vec![Option::<St>::None; Sp::NEIGHBOR_DIRECTIONS].into_boxed_slice();
	let mut to_propogate = BTreeSet::new();
	while let Some(to_collapse) = find_next_to_collapse(&mut unresolved_set, &mut lowest_entropy_set, &mut resolved_set, space) {
		to_propogate.clear();
		space[to_collapse].pick_from_possible();
		space.get_neighbors(to_collapse, &mut neighbors);
		for i in 0..Sp::NEIGHBOR_DIRECTIONS {
			if let Some(neighbor_coord) = neighbors[i] {
				to_propogate.insert(neighbor_coord);
			}
		}
		while let Some(&propogating) = to_propogate.iter().next() {
			to_propogate.remove(&propogating);
			let entropy_before = space[propogating].entropy();
			
			if entropy_before != 0 {
				space.get_neighbors(propogating, &mut neighbors);
				for i in 0 .. Sp::NEIGHBOR_DIRECTIONS {
					neighbor_states[i] = neighbors[i].map(|coord| space[coord].clone());
				}
				rule(&mut space[propogating], &neighbor_states[..]);
				let entropy_after = space[propogating].entropy();
				
				if entropy_after < entropy_before {
					for i in 0 .. Sp::NEIGHBOR_DIRECTIONS {
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
