/// Cell state - represents all possible states a cell can take on
pub trait State: Clone + PartialEq {
	/// Gets the entropy value of this state. Zero means that the state is
	/// final, and cannot be collapsed further, while higher values mean there
	/// are more possible values this state could collapse to.
	fn entropy(&self) -> u32;
}
