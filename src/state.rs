pub trait State: Clone + PartialEq {
	fn entropy(&self) -> u32;
	fn pick_from_possible(&mut self);
}
