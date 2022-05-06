use std::fmt::Display;

use kahuna::*;
use kahuna::square_grid::*;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct States(u32);

const ST_CORNER: u32 = 1 << 0;
const ST_VBAR: u32 =   1 << 1;
const ST_HBAR: u32 =   1 << 2;
const ST_NONE: u32 =   1 << 3;
const ST_ALL: u32 =    ST_CORNER | ST_VBAR | ST_HBAR | ST_NONE;

impl Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
			States(ST_CORNER) => write!(f, "*"),
			States(ST_VBAR) => write!(f, "|"),
			States(ST_HBAR) => write!(f, "-"),
			States(ST_NONE) => write!(f, " "),
			_ => write!(f, "?")
		}
    }
}

impl State for States {
    fn entropy(&self) -> u32 {
		let States(x) = *self;
        x.count_ones() - 1
    }
}

type Grid = SquareGrid<States>;

struct Rule();

impl CollapseRule<States, Grid> for Rule {
	const NEIGHBOR_DIRECTIONS: &'static [(isize, isize)] = &[
		(0, -1),
		(-1, 0),
		(1, 0),
		(0, 1)
	];
	
	fn collapse(cell: &mut States, neighbors: &[Option<States>]) {
		let States(x) = cell;
		
		for rule in &RULES[..] {
			if *x & rule.state != 0 {
				for i in 0 .. Self::NEIGHBOR_DIRECTIONS.len() {
					if let Some(States(neighbor)) = neighbors[i] {
						if neighbor & rule.allowed_neighbors[i] == 0 {
							*x &= !rule.state;
						}
					}
				}
			}
		}
	}
	
	fn observe(cell: &mut States, _neighbors: &[Option<States>]) {
		let States(x) = cell;
		let mut bits = vec![];
		for i in 0 .. 4 {
			if *x & (1 << i) != 0 {
				bits.push(i);
			}
		}
		*x = 1 << bits[thread_rng().gen_range(0..bits.len())];
	}
}

struct StateRule {
	state: u32,
	allowed_neighbors: [u32; 4]
}

const RULES: &'static [StateRule] = &[
	StateRule {
		state: ST_CORNER,
		allowed_neighbors: [
							   ST_NONE | ST_VBAR,
			ST_NONE | ST_HBAR,                    ST_NONE | ST_HBAR,
							   ST_NONE | ST_VBAR,
		]
	},
	StateRule {
		state: ST_HBAR,
		allowed_neighbors: [
								 ST_NONE,
			ST_HBAR | ST_CORNER,          ST_HBAR | ST_CORNER,
								 ST_NONE,
		]
	},
	StateRule {
		state: ST_VBAR,
		allowed_neighbors: [
					 ST_VBAR | ST_CORNER,
			ST_NONE,                      ST_NONE,
					 ST_VBAR | ST_CORNER,
		]
	},
	StateRule {
		state: ST_NONE,
		allowed_neighbors: [
					 !ST_VBAR,
			!ST_HBAR,         !ST_HBAR,
					 !ST_VBAR,
		]
	}
];

fn main() {
	let mut grid = Grid::new(40, 20, |_, _| States(ST_ALL));
	collapse::<Rule, _, _>(&mut grid);
	for y in 0..20 {
		for x in 0..40 {
			print!("{}", grid[(x, y)]);
		}
		println!("");
	}
}