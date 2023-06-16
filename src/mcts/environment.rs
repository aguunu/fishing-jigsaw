use std::{fmt::Debug, hash::Hash};
pub trait Environment {
    type Action: Hash + Eq + Copy + Debug + Send + Sync;

    fn has_finished(&self) -> bool;
    fn perform_action(&mut self, action: Self::Action);
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn eval(&self) -> i32;
}
