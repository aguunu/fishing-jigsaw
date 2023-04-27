use super::utils::{BuildIdentityHasher, Index};
use std::collections::HashMap;
use std::hash::Hash;

pub struct Node<A: Hash + Eq + PartialEq> {
    pub visits: u32,
    pub wins: i32,
    pub parent: Option<Index>,
    pub children: HashMap<A, Index, BuildIdentityHasher>,
}

impl<A: Hash + Eq + PartialEq> Node<A> {
    pub fn new() -> Self {
        Node {
            visits: 0,
            wins: 0,
            parent: None,
            children: HashMap::with_hasher(BuildIdentityHasher),
        }
    }

    pub fn set_parent(&mut self, parent: Index) {
        self.parent = Some(parent);
    }

    pub fn set_child(&mut self, action: A, child: Index) {
        self.children.insert(action, child);
    }

    pub fn update(&mut self, wins: i32, visits: u32) {
        self.wins += wins;
        self.visits += visits;
    }

    pub fn ucb(&self, c: f32, parent_visits: u32) -> f32 {
        let explotation = self.wins as f32 / self.visits as f32;

        let exploration = c * (parent_visits as f32).ln().sqrt() / self.visits as f32;

        explotation + exploration
    }
}
