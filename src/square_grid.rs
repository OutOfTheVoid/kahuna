use std::ops::{IndexMut, Index};

use crate::Space;

/// Basic square grid implementing [crate::Space]
/// 
/// coordinates and coordinate directions are specified as `(isize, isize)`.
pub struct SquareGrid<T> {
	cells: Box<[T]>,
	width: isize,
	height: isize,
}

impl<T> SquareGrid<T> {
	/// Create a new SquareGrid
	/// 
	/// * `width` - width of the grid
	/// * `height` - height of the grid
	/// * `init_fn` - callback to set the initial state of each cell based on
	/// coordinate
	pub fn new(width: isize, height: isize, init_fn: impl Fn(isize, isize) -> T) -> Self {
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
}

impl<T: 'static> Index<<SquareGrid<T> as Space<T>>::Coordinate> for SquareGrid<T> {
    type Output = T;

    fn index(&self, index: <SquareGrid<T> as Space<T>>::Coordinate) -> &Self::Output {
		let (x, y) = index;
        &self.cells[(x + y * self.width) as usize]
    }
}

impl<T: 'static> IndexMut<<SquareGrid<T> as Space<T>>::Coordinate> for SquareGrid<T> {
	fn index_mut(&mut self, index: <SquareGrid<T> as Space<T>>::Coordinate) -> &mut Self::Output {
		let (x, y) = index;
        &mut self.cells[(x + y * self.width) as usize]
    }
}

impl<T: 'static> Space<T> for SquareGrid<T> {
    type Coordinate = (isize, isize);
	type CoordinateDelta = (isize, isize);

    fn coordinate_list(&self) -> Box<[Self::Coordinate]> {
        let mut coords = Vec::new();
		for y in 0..self.height {
			for x in 0..self.width {
				coords.push((x, y));
			}
		}
		coords.into_boxed_slice()
    }

    fn neighbors(&self, coord: Self::Coordinate, neighbor_directions: &[Self::CoordinateDelta], neighbors: &mut [Option<Self::Coordinate>]) {
		assert!(neighbor_directions.len() <= neighbors.len());
		
		let (x, y) = coord;
		for i in 0..neighbor_directions.len() {
			let (dx, dy) = neighbor_directions[i];
			let (nx, ny) = (x + dx, y + dy);
			if nx.clamp(0, self.width - 1) == nx && ny.clamp(0, self.height - 1) == ny {
				neighbors[i] = Some((nx, ny));
			} else {
				neighbors[i] = None;
			}
		}
    }
}
