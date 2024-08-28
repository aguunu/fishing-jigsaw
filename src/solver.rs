use crate::jigsaw::Jigsaw;

pub trait Solver {
    fn solve(&self, game: &Jigsaw) -> u8;
}
