/// Describes states which have a known "All-possibilities" state
pub trait AllState {
	fn all() -> Self;
}

