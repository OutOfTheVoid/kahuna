use crate::State;

pub trait CollapseRule<S: State>: Fn(&mut S, &[Option<S>]) {}
impl<T, S: State> CollapseRule<S> for T where T: Fn(&mut S, &[Option<S>]) {}
