use std::{ops::IndexMut, hash::Hash};

/// Defines the space or "world" to run WFC on.
/// 
/// This is the primary data structure behind WFC, and is modified in-place by
/// the algorithm. The only expectation placed on Space is that it's size and
/// shape do not change during calls to [crate::collapse].
/// 
/// In order to support arbitrary dimension and shape, two associated types are
/// defined:
/// - `Coordinate` is the index type for this space. Cells in the space are
/// uniquely identified by coordinates.
/// - `CoordinateDelta` represents adjacency relations between cells. In
/// general, a collapse rule supplies a list of coordinate deltas to get
/// neighbor cell coordinates.
pub trait Space<T>: IndexMut<Self::Coordinate, Output = T> + 'static {
	/// Coordinates for cells in the space
	type Coordinate: Copy + Hash + Ord;
	/// Spatial relationship between cells for accessing neighbors
	type CoordinateDelta: 'static;
	
	/// Get every valid coordinate in the space.
	fn coordinate_list(&self) -> Box<[Self::Coordinate]>;
	/// Get the neighbor coordinates of a given cell based on a list of deltas.
	/// 
	/// * `coord` - Coordinate of the cell to find neighbors for
	/// * `neighbor_directions` - List of neighbor cell offsets
	/// * `neighbors` - Output list of neighbor coordinates. Must be at least
	/// as long as neighbor_directions. Set to `None` for neighbors which are
	/// out of bounds for the space.
	fn neighbors(&self, coord: Self::Coordinate, neighbor_directions: &[Self::CoordinateDelta], neighbors: &mut [Option<Self::Coordinate>]);
}


