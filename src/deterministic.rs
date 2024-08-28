use std::collections::VecDeque;

use lazy_static::lazy_static;
use crate::jigsaw::{Figure, Jigsaw, FIGURES, N, SKIP_ACTION, TERMINAL_STATE, TOTAL_CELLS, TOTAL_FIGURES};
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct State {
    dsts: [f32; TOTAL_FIGURES],
    actions: [u8; TOTAL_FIGURES],
    in_stack: bool,
}

impl State {
    fn new() -> Self {
        Self {
            dsts: [f32::INFINITY; TOTAL_FIGURES],
            actions: [SKIP_ACTION; TOTAL_FIGURES],
            in_stack: false,
        }
    }

    #[inline(always)]
    fn compute_none_dst(&mut self) {
        lazy_static! {
            static ref SETS: Vec<Vec<usize>> = (0..TOTAL_FIGURES).powerset().skip(1).collect();
        };

        let skp_dst = SETS
            .iter()
            .map(|subset| {
                let sum = subset.iter().map(|&idx| self.dsts[idx]).sum::<f32>();
                let len = subset.len();
                // expected rolls to get the set + avg distance
                (TOTAL_FIGURES as f32 + sum) / len as f32
            })
            .reduce(f32::min)
            .unwrap();

        self.actions
            .iter_mut()
            .zip(self.dsts.iter_mut())
            .filter(|(_, &mut d)| d > skp_dst)
            .for_each(|(action, value)| {
                *action = SKIP_ACTION;
                *value = skp_dst;
            });
    }

    #[inline(always)]
    fn avg_dst(&self) -> f32 {
        self.dsts.iter().sum::<f32>() / TOTAL_FIGURES as f32
    }
}

pub struct Deterministic {
    arr: Vec<State>,
}

#[inline(always)]
fn is_possible(board: u32, figure: &Figure, x_offset: u8, y_offset: u8) -> bool {
    // 000CCC|RR
    let mut ilegal = false;

    ilegal |= x_offset > figure.max_offset.0;
    ilegal |= y_offset > figure.max_offset.1;
    ilegal |= (board | figure.value >> (x_offset * N + y_offset)) != board;

    return !ilegal;
}

impl Deterministic {
    pub fn new() -> Self {
        Self {
            arr: vec![State::new(); 1 << TOTAL_CELLS],
        }
    }

    pub fn run(&mut self) {
        let mut stacks = VecDeque::new();

        for _ in 0..=TOTAL_CELLS as usize {
            // could be less memory but it does not really matter.
            let stack = Vec::with_capacity(1 << TOTAL_CELLS);
            stacks.push_back(stack);
        }

        stacks[0].push(TERMINAL_STATE);
        self.arr[TERMINAL_STATE as usize].in_stack = true;
        self.arr[TERMINAL_STATE as usize].dsts = [0.0; TOTAL_FIGURES];
        self.arr[TERMINAL_STATE as usize].actions = [SKIP_ACTION; TOTAL_FIGURES];

        let actions = (0..TOTAL_CELLS as u8).map(|a| (a >> 2, a & 0b11)).collect_vec();

        // the algorithm could be implemented in parallel, but the communication
        // and synchronization overhead between threads has a heavy impact. 
        for height in 0..TOTAL_CELLS {
            let valid_figures = FIGURES
                .iter()
                .filter(|&f| height + f.size <= TOTAL_CELLS)
                .enumerate().collect_vec();

            while let Some(board) = stacks[height as usize].pop() {
                self.arr[board as usize].compute_none_dst();
                let dst = 1.0 + self.arr[board as usize].avg_dst();

                for (f_idx, f) in &valid_figures {
                    for (x, y) in actions.iter()
                        .filter(|&(x, y)| is_possible(board, f, *x, *y))
                    {
                        let new_board = board & !(f.value >> (x * N + y));
                       
                        if dst < self.arr[new_board as usize].dsts[*f_idx] {
                            self.arr[new_board as usize].dsts[*f_idx] = dst;
                            self.arr[new_board as usize].actions[*f_idx] = (x << 2) | y;
                        }

                        if !self.arr[new_board as usize].in_stack {
                            self.arr[new_board as usize].in_stack = true;

                            stacks[(height + f.size) as usize].push(new_board);
                        }
                    }
                }
            }
        }
    }

    /* pub fn solve(&mut self, game: &Jigsaw) -> u8 {
        let (f_idx, _) = FIGURES
            .iter()
            .enumerate()
            .find(|(_, f)| f.value == game.figure().value)
            .unwrap();

        let action = self.arr[game.board as usize].actions[f_idx];
        action
    } */

    pub fn distances(&self, board: u32) -> impl Iterator<Item = (u8, f32)> {
        let state = &self.arr[board as usize];
        state.actions.into_iter().zip(state.dsts).clone()
    }
}

use crate::solver::Solver;

impl Solver for Deterministic {
    fn solve(&self, game: &Jigsaw) -> u8 {
        let (f_idx, _) = FIGURES
            .iter()
            .enumerate()
            .find(|(_, f)| f.value == game.figure().value)
            .unwrap();

        let action = self.arr[game.board as usize].actions[f_idx];
        action
    }
}

