use std::{ops::IndexMut, hash::Hash, fmt::Debug};

pub trait Space<T>: IndexMut<Self::Coordinate, Output = T> {
	type Coordinate: Copy + Hash + Ord + Debug;
	const NEIGHBOR_DIRECTIONS: usize;
	
	fn coordinate_list(&self) -> Box<[Self::Coordinate]>;
	fn get_neighbors(&self, coord: Self::Coordinate, neighbors: &mut [Option<Self::Coordinate>]);
}


