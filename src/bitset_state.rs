use std::ops::{BitOr, BitAnd, BitXor};

use crate::{SetState, State, AllState};

/// A state type which uses bits of a u64 to describe up to 64 separate possible final states.
/// 
/// * `FINAL_STATE_COUNT` - the total number of final (fully collapsed) states
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct BitsetState<const FINAL_STATE_COUNT: u32>(u64);

impl<const FINAL_STATE_COUNT: u32> BitsetState<FINAL_STATE_COUNT> {
	/// Creates the `n`th unique state
	pub const fn state(n: u32) -> Self {
		BitsetState(1u64 << n)
	}
	
	/// Creates a state representing the states numbered by members of `states`
	pub fn with_states(states: &[u32]) -> Self {
		let mut x: u64 = 0;
		for i in &states[..] {
			assert!(*i < FINAL_STATE_COUNT);
			x |= 1u64 << i;
		}
		BitsetState(x)
	}
	
	/// const-fn logical or of all states in `states`
	pub const fn const_or(states: &[Self]) -> Self  {
		match states {
			[] => BitsetState(0),
			[first] => *first,
			[first, second, ..] => BitsetState(first.0 | second.0)
		}
	}
}

impl<const FINAL_STATE_COUNT: u32> AllState for BitsetState<FINAL_STATE_COUNT> {
	fn all() -> Self {
		assert!(FINAL_STATE_COUNT <= 64 && FINAL_STATE_COUNT >= 1);
		BitsetState(
			if FINAL_STATE_COUNT == 64 {
				0xFFFF_FFFF_FFFF_FFFF
			} else {
				(1u64 << FINAL_STATE_COUNT).wrapping_sub(1)
			}
		)
	}
}

impl<const FINAL_STATE_COUNT: u32> State for BitsetState<FINAL_STATE_COUNT> {
    fn entropy(&self) -> u32 {
        let BitsetState(x) = *self;
		x.count_ones() - 1
    }
}

impl<const FINAL_STATE_COUNT: u32> SetState for BitsetState<FINAL_STATE_COUNT> {
	fn has_any_of(&self, states: &Self) -> bool {
		self.0 & states.0 != 0
	}
	
	fn clear_states(&mut self, states: &Self) {
		self.0 &= !states.0
	}
	
	fn set_states(&mut self, states: &Self) {
		self.0 |= states.0
	}
	
	fn collect_final_states(&self, states: &mut Vec<Self>) {
		let BitsetState(x) = *self;
		for i in 0 .. 64 {
			let bit = 1u64 << i;
			if x & bit != 0 {
				states.push(BitsetState(bit));
			}
		}
	}
}

impl<const FINAL_STATE_COUNT: u32> BitOr for BitsetState<FINAL_STATE_COUNT> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitsetState(self.0 | rhs.0)
    }
}

impl<const FINAL_STATE_COUNT: u32> BitAnd for BitsetState<FINAL_STATE_COUNT> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitsetState(self.0 & rhs.0)
    }
}

impl<const FINAL_STATE_COUNT: u32> BitXor for BitsetState<FINAL_STATE_COUNT> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitsetState(self.0 ^ rhs.0)
    }
}
