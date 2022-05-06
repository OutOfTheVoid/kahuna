use crate::State;

#[derive(PartialEq, Clone, Hash)]
pub struct BitState<const FINAL_STATE_COUNT: u32>(u64);

impl<const FINAL_STATE_COUNT: u32> BitState<FINAL_STATE_COUNT> {
	pub fn all() -> Self {
		assert!(FINAL_STATE_COUNT <= 64 && FINAL_STATE_COUNT >= 1);
		BitState(
			if FINAL_STATE_COUNT == 64 {
				0xFFFF_FFFF_FFFF_FFFF
			} else {
				1u64 << FINAL_STATE_COUNT
			}.wrapping_sub(1)
		)
	}
	
	pub fn state(n: u32) -> Self {
		assert!(n < FINAL_STATE_COUNT);
		BitState(1u64 << n)
	}
	
	pub fn subset_of(&self, states: &BitState<FINAL_STATE_COUNT>) -> bool {
		let BitState(x) = *self;
		let BitState(y) = *states;
		x & !y == 0
	}
	
	pub fn clear_states(&mut self, states: &BitState<FINAL_STATE_COUNT>) {
		let BitState(x) = self;
		let BitState(y) = *states;
		*x &= !y;
	}
	
	pub fn collect_final_states(&self, states: &mut Vec<u32>) {
		let BitState(x) = *self;
		for i in 0 .. 64 {
			if x & 1u64 << i != 0 {
				states.push(i);
			}
		}
	}
}

impl<const FINAL_STATE_COUNT: u32> State for BitState<FINAL_STATE_COUNT> {
    fn entropy(&self) -> u32 {
        let BitState(x) = *self;
		x.count_ones() - 1
    }
}
