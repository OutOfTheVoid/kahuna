use std::fmt::{Display, self};

use kahuna::*;
use kahuna::square_grid::SquareGrid;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PossibleStates {
	AB,
	A,
	B,
}
/*
impl Display for PossibleStates {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
			match *self {
				PossibleStates::AB => "_",
				PossibleStates::A => "A",
				PossibleStates::B => "B",
			}
		)
    }
}
*/

impl State for PossibleStates {
    fn entropy(&self) -> u32 {
        match self {
			&PossibleStates::AB => 1,
			_ => 0
		}
    }

    fn pick_from_possible(&mut self) {
        if let PossibleStates::AB = self {
			*self = if thread_rng().gen::<bool>() { PossibleStates::A } else { PossibleStates::B };
		}
    }
}

type TestGrid = SquareGrid<PossibleStates>;

fn basic_rule(s: &mut PossibleStates, neighbors: &[Option<PossibleStates>]) {
	if *s == PossibleStates::AB {
		match (neighbors[TestGrid::NEIGHBOR_LEFT], neighbors[TestGrid::NEIGHBOR_RIGHT]) {
			(Some(PossibleStates::A), _) => *s = PossibleStates::B,
			(_, Some(PossibleStates::A)) => *s = PossibleStates::B,
			(_, Some(PossibleStates::B)) => *s = PossibleStates::A,
			(Some(PossibleStates::B), _) => *s = PossibleStates::A,
			_ => {}
		}
	}
}

#[test]
fn test_basic() {
	let mut grid = SquareGrid::new(10, 10, |x, y| PossibleStates::AB);
	collapse(&mut grid, &basic_rule);
	for y in 0..10 {
		for x in 0..9 {
			assert_ne!(grid[(x, y)], PossibleStates::AB);
		}
		for x in 0..9 {
			assert_ne!(grid[(x, y)], grid[(x + 1, y)]);
		}
	}
}
