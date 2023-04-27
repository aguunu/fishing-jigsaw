use std::hash::Hash;
pub trait Environment {
    type Action: Hash + Eq + Copy;

    fn has_finished(&self) -> bool;
    fn perform_action(&mut self, action: Self::Action);
    fn legal_actions(&self) -> Vec<Self::Action>;
    fn eval(&self) -> i32;
}
