use crate::mcts::Environment;
use std::fmt;

const ROWS: u8 = 4;
const COLS: u8 = 6;
const SKIP_ACTION: u8 = 24;
const TOTAL_FIGURES: u8 = 6;
const TOTAL_ACTIONS: u8 = 24 + 1;
pub const ALL_FIGURES: [u32; TOTAL_FIGURES as usize] = [
    0b100000000000000000000000,
    0b100000100000100000000000,
    0b100000110000000000000000,
    0b110000010000000000000000,
    0b110000110000000000000000,
    0b110000011000000000000000,
];

#[derive(Clone)]
pub struct Jigsaw {
    pub board: u32,
    pub figure_index: u8,
    pub quantity: u8,
}

fn random_figure() -> u8 {
    use rand::Rng;
    rand::thread_rng().gen_range(0..ALL_FIGURES.len() as u8)
}

impl Jigsaw {
    pub fn new() -> Self {
        Self {
            board: 0,
            figure_index: random_figure(),
            quantity: 0,
        }
    }

    pub fn toggle_coord(&mut self, coord: (u8, u8)) {
        let x = 1 << (ROWS * COLS - 1);
        self.board ^= x >> (coord.0 * COLS + coord.1);
    }

    pub fn coord(&self, coord: (u8, u8)) -> bool {
        let x = 1 << (ROWS * COLS - 1);
        (self.board & (x >> (coord.0 * COLS + coord.1))) != 0
    }

    pub fn figure(&self) -> u32 {
        ALL_FIGURES[usize::from(self.figure_index)]
    }

    fn is_legal(&self, action: u8) -> bool {
        if action == SKIP_ACTION {
            return true;
        }

        let x_offset: u8 = action % COLS;

        if (self.board & (self.figure() >> action)) != 0 {
            return false;
        }

        if ((self.figure() >> action) << action) != self.figure() {
            return false;
        }

        let column_mask: u8 = !(0xFF << COLS);

        for i in 0..ROWS {
            let figure_row = (self.figure() >> (COLS * i)) as u8 & column_mask;

            if ((figure_row >> x_offset) << x_offset) != figure_row {
                return false;
            }
        }

        return true;
    }
}

impl fmt::Debug for Jigsaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();

        let quantity_str = format!("========={}=========\n", self.quantity);
        string.push_str(&quantity_str);

        let mut board_mask: u32 = 1 << (ROWS * COLS - 1);
        let mut figure_mask: u32 = 1 << (ROWS * COLS - 1);

        for i in 0..ROWS {
            for j in 0..COLS {
                if (self.board & board_mask) != 0 {
                    string.push('🟩');
                } else if self.is_legal(i * COLS + j) {
                    string.push('🟦');
                } else {
                    string.push('⬛');
                }
                board_mask /= 2;
            }

            string.push(' ');

            for _j in 0..COLS {
                if (self.figure() & figure_mask) != 0 {
                    string.push('🟥');
                } else {
                    string.push(' ');
                    string.push(' ');
                }
                figure_mask /= 2;
            }

            string.push('\n');
        }
        write!(f, "{string}")
    }
}

impl Environment for Jigsaw {
    type Action = u8;

    fn has_finished(&self) -> bool {
        self.board == 0xFFFFFF
    }

    fn perform_action(&mut self, action: Self::Action) {
        if action != SKIP_ACTION {
            self.board |= self.figure() >> action;
        }

        self.quantity += 1;

        self.figure_index = random_figure();
    }

    fn legal_actions(&self) -> Vec<Self::Action> {
        (0..TOTAL_ACTIONS)
            .filter(|&action| self.is_legal(action))
            .collect()
    }

    fn eval(&self) -> i32 {
        if self.has_finished() {
            1
        } else {
            0
        }
    }
}
