use core::panic;
use rand::Rng;

use crate::mcts::Environment;

/*
    Combination actions correspond to indexes 0..9 (10 actions),
    Skip actions correspond to indexes 10..14 (5 actions).
*/

const COLORS: u8 = 3;
const NUMBERS: u8 = 8;
const TOTAL_CARDS: u8 = COLORS * NUMBERS;

// const SKIP_ACTIONS: usize = 5;
// const COMBINATION_ACTIONS: usize = 10; // SKIP_ACTIONS C 3
// const TOTAL_ACTIONS: usize = SKIP_ACTIONS + COMBINATION_ACTIONS;

type Combination = (u8, u8, u8);

const ALL_COMBINATIONS: [Combination; 10] = [
    (0, 1, 2),
    (0, 1, 3),
    (0, 1, 4),
    (0, 2, 3),
    (0, 2, 4),
    (0, 3, 4),
    (1, 2, 3),
    (1, 2, 4),
    (1, 3, 4),
    (2, 3, 4),
];

fn get_icon(card: u8) -> String {
    let icon = match card / NUMBERS {
        0 => "🟥",
        1 => "🟩",
        2 => "🟦",
        _ => panic!(),
    };
    return icon.to_string();
}

#[derive(Clone, PartialEq, Debug)]
enum Agents {
    MACHINE,
    HUMAN,
}

#[derive(Clone)]
pub struct Okey {
    deck: u32,      // cards that has not been seen (from 0 to 23)
    board: Vec<u8>, // max_largo = 5
    agent: Agents,
    score: u16,
}

impl Okey {
    pub fn new() -> Self {
        Self {
            deck: (1 << TOTAL_CARDS) - 1,
            board: Vec::with_capacity(5),
            agent: Agents::MACHINE,
            score: 0,
        }
    }

    pub fn render(&self) {
        // render deck && board
        println!("Score: {} {:?}", self.score, self.agent);

        for i in 0..TOTAL_CARDS {
            if (self.deck & 1 << i) != 0 {
                // carta no vista
                print!("{}", get_icon(i));
            } else if self.board.contains(&i) {
                print!("⬜")
            } else {
                print!("⬛")
            }

            if (i + 1) % NUMBERS == 0 {
                println!();
            }
        }

        for card in self.board.iter() {
            print!("{}{} |", card % NUMBERS, get_icon(*card));
        }
        println!();
    }

    fn compute_score(&self, combination: Combination) -> u8 {
        /* Computes the score of a legal combination in the current state */

        let cards: (u8, u8, u8) = (
            self.board[combination.0 as usize],
            self.board[combination.1 as usize],
            self.board[combination.2 as usize],
        );

        /* Check if same number combination */
        if cards.0 % NUMBERS == cards.1 % NUMBERS && cards.1 % NUMBERS == cards.2 % NUMBERS {
            return ((cards.0 % NUMBERS) + 1) * 10;
        }

        /* Otherwise, stair combination */
        let mut score = (cards.0 % NUMBERS)
            .min(cards.1 % NUMBERS)
            .min(cards.2 % NUMBERS)
            * 10;

        /* Check if same color */
        if cards.0 / NUMBERS == cards.1 / NUMBERS && cards.1 / NUMBERS == cards.2 / NUMBERS {
            score += 40;
        }

        return score;
    }
}

impl Environment for Okey {
    type Action = u8;

    fn has_finished(&self) -> bool {
        (self.deck & (1 << TOTAL_CARDS) - 1) == 0 && self.board.len() == 0
    } // deck and board empty

    fn perform_action(&mut self, action: Self::Action) {
        match self.agent {
            Agents::MACHINE => {
                assert!((self.deck & (1 << action)) != 0);
                assert!(!self.board.contains(&action));

                // remove card from deck
                self.deck &= !(1 << action);

                // insert card in board
                self.board.push(action);
            }
            Agents::HUMAN => {
                if action <= 9 {
                    let combination = ALL_COMBINATIONS[action as usize];

                    self.score += u16::from(self.compute_score(combination));

                    /* Must remove in reverse order */
                    self.board.swap_remove(combination.2 as usize);
                    self.board.swap_remove(combination.1 as usize);
                    self.board.swap_remove(combination.0 as usize);
                } else {
                    self.board.swap_remove((action % 10) as usize);
                }

                self.board.sort_by(|a, b| (a % NUMBERS).cmp(&(b % NUMBERS)));
            }
        }

        // if deck is not empty and boards is not full, machine must draw
        let board_full = self.board.len() == 5;
        let deck_empty = (self.deck & 0xFFFFFF) == 0;

        self.agent = if !deck_empty && !board_full {
            Agents::MACHINE
        } else {
            Agents::HUMAN
        };
    }

    fn legal_actions(&self) -> Vec<Self::Action> {
        let mut legal_actions = Vec::new();

        match self.agent {
            Agents::MACHINE => {
                for i in 0..TOTAL_CARDS {
                    if ((1 << i) & self.deck) != 0 {
                        legal_actions.push(i);
                    }
                }
            }
            Agents::HUMAN => {
                // combination actions
                for combination_index in 0..=9 {
                    let combination = ALL_COMBINATIONS[combination_index];

                    if !(self.board.len() > combination.0.into()
                        && self.board.len() > combination.1.into()
                        && self.board.len() > combination.2.into())
                    {
                        continue;
                    }

                    let cards: (u8, u8, u8) = (
                        self.board[combination.0 as usize],
                        self.board[combination.1 as usize],
                        self.board[combination.2 as usize],
                    );

                    let same_number = cards.0 % NUMBERS == cards.1 % NUMBERS
                        && cards.1 % NUMBERS == cards.2 % NUMBERS;
                    let stair = cards.0 % NUMBERS + 1 == (cards.1 % NUMBERS)
                        && (cards.1 % NUMBERS) + 1 == (cards.2 % NUMBERS);

                    if same_number || stair {
                        legal_actions.push(combination_index as u8);
                    }
                }

                // skip actions
                for board_index in 0..self.board.len() {
                    legal_actions.push(10 + board_index as u8);
                }
            }
        }

        return legal_actions;
    }

    fn eval(&self, current_depth: u32) -> i32 {
        if self.score >= 400 {
            1
        } else {
            0
        }
        // self.score.into()
    }

    fn draw(&mut self) {
        if (self.deck & 0xFFFFFF) == 0 {
            return; // if deck empty cannot draw
        }

        assert!(self.agent == Agents::MACHINE);

        let mut rng = rand::thread_rng();

        while !self.has_finished() && self.agent == Agents::MACHINE {
            let legal_actions = self.legal_actions();
            let action_index: u8 = rng.gen::<u8>() % u8::try_from(legal_actions.len()).unwrap();
            let action = legal_actions.get(action_index as usize).unwrap();
            self.perform_action(*action);
        }

        self.board.sort_by(|a, b| (a % NUMBERS).cmp(&(b % NUMBERS)));

        assert!(self.agent == Agents::HUMAN);
    }
}
