use std::ops::{IndexMut, Index};

use crate::Space;

pub struct SquareGrid<T> {
	cells: Box<[T]>,
	width: usize,
	height: usize,
}

impl<T> SquareGrid<T> {
	pub fn new(width: usize, height: usize, init_fn: impl Fn(usize, usize) -> T) -> Self {
		let mut cells = Vec::new();
		for y in 0..height {
			for x in 0..width {
				cells.push(init_fn(x, y));
			}
		}
		Self {
			cells: cells.into_boxed_slice(),
			width,
			height,
		}
	}
	/*
	Neighbor directions:

	0  1  2
	3  *  4
	5  6  7
	*/
	pub const NEIGHBOR_UP_LEFT: usize = 0;
	pub const NEIGHBOR_UP: usize = 1;
	pub const NEIGHBOR_UP_RIGHT: usize = 2;
	pub const NEIGHBOR_LEFT: usize = 3;
	pub const NEIGHBOR_RIGHT: usize = 4;
	pub const NEIGHBOR_DOWN_LEFT: usize = 5;
	pub const NEIGHBOR_DOWN: usize = 6;
	pub const NEIGHBOR_DOWN_RIGHT: usize = 7;
}

impl<T> Index<<SquareGrid<T> as Space<T>>::Coordinate> for SquareGrid<T> {
    type Output = T;

    fn index(&self, index: <SquareGrid<T> as Space<T>>::Coordinate) -> &Self::Output {
		let (x, y) = index;
        &self.cells[x + y * self.width]
    }
}

impl<T> IndexMut<<SquareGrid<T> as Space<T>>::Coordinate> for SquareGrid<T> {
	fn index_mut(&mut self, index: <SquareGrid<T> as Space<T>>::Coordinate) -> &mut Self::Output {
		let (x, y) = index;
        &mut self.cells[x + y * self.width]
    }
}

impl<T> Space<T> for SquareGrid<T> {
    type Coordinate = (usize, usize);
	const NEIGHBOR_DIRECTIONS: usize = 8;

    fn coordinate_list(&self) -> Box<[Self::Coordinate]> {
        let mut coords = Vec::new();
		for y in 0..self.height {
			for x in 0..self.width {
				coords.push((x, y));
			}
		}
		coords.into_boxed_slice()
    }

    fn get_neighbors(&self, coord: Self::Coordinate, neighbors: &mut [Option<Self::Coordinate>]) {
        let (x, y) = coord;
		
		let left_edge = x == 0;
		let right_edge = x + 1 == self.width;
		let top_edge = y == 0;
		let bottom_edge = y + 1 == self.height;
		
		/*
		Neighbor directions:

		0  1  2
		3  *  4
		5  6  7
		*/
		
		neighbors[0] = if !left_edge & !top_edge     { Some((x - 1, y - 1)) } else { None };
		neighbors[1] = if !top_edge                  { Some((x,     y - 1)) } else { None };
		neighbors[2] = if !right_edge & !top_edge    { Some((x + 1, y - 1)) } else { None };
		neighbors[3] = if !left_edge                 { Some((x - 1, y    )) } else { None };
		neighbors[4] = if !right_edge                { Some((x + 1, y    )) } else { None };
		neighbors[5] = if !left_edge & !bottom_edge  { Some((x - 1, y + 1)) } else { None };
		neighbors[6] = if !bottom_edge               { Some((x,     y + 1)) } else { None };
		neighbors[7] = if !right_edge & !bottom_edge { Some((x + 1, y + 1)) } else { None };
		
    }
}
