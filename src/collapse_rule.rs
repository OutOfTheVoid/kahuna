use crate::{State, Space};

/// Collapse rules define the relationships between a cell's possible state
/// based on it's neighbors.
/// 
/// While this can be anything, it is recommended that collapse rules are
/// purely subtractive in nature, either reducing or maintaining the number
/// possible states that a cell can take on. With addative rules, runtime
/// can be unbounded, the algorithm may (randomly) never converge on a
/// solution.
pub trait CollapseRule<S: State, Sp: 'static + Space<S>> {
	/// Neighbor directions are specified as a list of coordinate deltas.
	fn neighbor_offsets(&self) -> Box<[Sp::CoordinateDelta]>;
	/// The collapse rule, which modifies the possible states of 'cell' based
	/// on the states of neighboring cells.
	/// 
	/// * `cell` - The cell state to modify
	/// * `neighbors` - The states of neighbors in the order specified by
	/// `NEIGHBOR_DIRECTIONS`. `Some(<state>)` if the cell exists, and `None`
	/// otherwise.
	fn collapse(&self, cell: &mut S, neighbors: &[Option<S>]);
	/// The observe rule, which forces a cell into a zero-entropy state.
	/// 
	/// * `cell` - The cell to observe
	/// * `neighbors` - The states of neighbor cells as in `collapse()` above.
	fn observe(&self, cell: &mut S, neighbors: &[Option<S>]);
}
