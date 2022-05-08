use kahuna::{square_grid::SquareGrid, AllState};
use kahuna::{set_rule::*, collapse};
use kahuna::bitset_state::BitsetState;
use image::{ImageFormat, ColorType, RgbImage};

type S = BitsetState<11>;

const A: S = S::state(0);
const B: S = S::state(1);
const C: S = S::state(2);
const D: S = S::state(3);
const E: S = S::state(4);
const F: S = S::state(5);
const G: S = S::state(6);
const H: S = S::state(7);
const I: S = S::state(8);
const J: S = S::state(9);
const K: S = S::state(10);

const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);

type Grid = SquareGrid<S>;

const WIDTH_TILES: u32 = 40;
const HEIGHT_TILES: u32 = 40;

fn main() {
	//  
	//  A B C J
	//  D E F K
	//  G H I
	//  
	//
	// A-I form a 9-quadrant for rectangles, J is open space around them, and K can touch only J
	
	let rule = SetCollapseRuleBuilder::new(UniformSetCollapseObserver)
		.allow(&E, &[
			(UP, E | B),
			(LEFT, E | D),
			(RIGHT, E | F),
			(DOWN, E | H)
		])
		.allow(&A, &[
			(LEFT, C | F | I),
			(UP, G | H | I)
		])
		.allow(&B, &[
			(LEFT, A | B),
			(RIGHT, C | B),
			(UP, G | H | I)
		])
		.allow(&C, &[
			(UP, G | H | I),
			(RIGHT, A | D | G)
		])
		.allow(&G, &[
			(DOWN, A | B | C),
			(LEFT, C | F | I)
		])
		.allow(&I, &[
			(RIGHT, A | D | G),
			(DOWN, A | B | C)
		])
		.allow(&H, &[
			(LEFT, G | H),
			(RIGHT, I | H),
			(DOWN, A | B | C)
		])
		.allow(&F, &[
			(UP, C | F),
			(DOWN, I | F),
			(RIGHT, A | D | C)
		])
		.allow(&D, &[
			(UP, A | D),
			(DOWN, G | D),
			(LEFT, C | F | I)
		])
		.allow(&J, &[
			(UP, J | G | H | I | K),
			(DOWN, J | A | B | C | K),
			(LEFT, J | C | F | I | K),
			(RIGHT, J | A | D | G | K)
		])
		.build();
		let mut grid = Grid::new(WIDTH_TILES as isize, HEIGHT_TILES as isize, |_, _| S::all());
	collapse(&mut grid, &rule);
	
	let image_bytes = include_bytes!("pattern.png");
	let input_image = image::load_from_memory_with_format(&image_bytes[..], ImageFormat::Png).unwrap().into_rgb8();
	let mut output_image = RgbImage::new(8*WIDTH_TILES, 8*HEIGHT_TILES);
	
	for y in 0..HEIGHT_TILES {
		for x in 0..WIDTH_TILES {
			let image_start_x = x * 8;
			let image_start_y = y * 8;
			let tile_start_x;
			let tile_start_y;
			match grid[(x as isize, y as isize)] {
				A => {
					tile_start_x = 0;
					tile_start_y = 0;
				},
				B => {
					tile_start_x = 8;
					tile_start_y = 0;
				},
				C => {
					tile_start_x = 16;
					tile_start_y = 0;
				},
				D => {
					tile_start_x = 0;
					tile_start_y = 8;
				},
				E => {
					tile_start_x = 8;
					tile_start_y = 8;
				},
				F => {
					tile_start_x = 16;
					tile_start_y = 8;
				},
				G => {
					tile_start_x = 0;
					tile_start_y = 16;
				},
				H => {
					tile_start_x = 8;
					tile_start_y = 16;
				},
				I => {
					tile_start_x = 16;
					tile_start_y = 16;
				},
				J => {
					tile_start_x = 24;
					tile_start_y = 0;
				},
				K => {
					tile_start_x = 24;
					tile_start_y = 8;
				},
				_ => panic!("unknown state!")
			}
			for j in 0..8 {
				for i in 0..8  {
					let pixel = input_image.get_pixel(tile_start_x + i, tile_start_y + j);
					output_image.put_pixel(image_start_x + i, image_start_y + j, *pixel);
				}
			}
		}
	}
	image::save_buffer("procedural_texture.png", &output_image, output_image.width(), output_image.height(), ColorType::Rgb8).unwrap();
}
