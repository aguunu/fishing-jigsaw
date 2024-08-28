use std::fmt;
use rand::prelude::*;

pub const N: u8 = 4;
pub const M: u8 = 6;
pub const SKIP_ACTION: u8 = N * M;
pub const TOTAL_FIGURES: usize = 6;
pub const TOTAL_CELLS: u8 = N * M;
pub const TOTAL_ACTIONS: u8 = TOTAL_CELLS + 1;
pub const TERMINAL_STATE: u32 = (1 << (N * M)) - 1;
pub const INIT_STATE: u32 = 0;

pub struct Figure {
    pub value: u32,
    pub size: u8,
    pub max_offset: (u8, u8),
}

pub const FIGURES: [Figure; TOTAL_FIGURES] = [
    Figure {
        value: 0b1000_0000_0000_0000_0000_0000, // 0b100000_000000_000000_000000,
        size: 1,
        max_offset: (5, 3),
    },
    Figure {
        value: 0b1110_0000_0000_0000_0000_0000, //0b100000_100000_100000_000000,
        size: 3,
        max_offset: (5, 1),
    },
    Figure {
        value: 0b1100_0100_0000_0000_0000_0000, //0b100000_110000_000000_000000,
        size: 3,
        max_offset: (4, 2),
    },
    Figure {
        value: 0b1000_1100_0000_0000_0000_0000, //0b110000_010000_000000_000000,
        size: 3,
        max_offset: (4, 2),
    },
    Figure {
        value: 0b1100_1100_0000_0000_0000_0000, //0b110000_110000_000000_000000,
        size: 4,
        max_offset: (4, 2),
    },
    Figure {
        value: 0b1000_1100_0100_0000_0000_0000, //0b110000_011000_000000_000000,
        size: 4,
        max_offset: (3, 2),
    },
];


#[derive(Clone, Copy)]
pub struct Jigsaw {
    pub board: u32,
    pub figure: u8,
    pub round: u8,
}


impl Jigsaw {
    pub fn set_random_figure(&mut self, rng: &mut rand::rngs::StdRng) {
        self.figure = rng.gen_range(0..FIGURES.len()) as u8;
    }

    pub fn has_finished(&self) -> bool {
        self.board == TERMINAL_STATE
    }

    pub fn perform_action(&mut self, action: u8) {        
        if action != SKIP_ACTION {
            assert!(self.is_legal(action));
            self.board |= self.figure().value >> action;
        }

        self.round += 1;
    }

    pub fn figure(&self) -> &Figure {
        &FIGURES[self.figure as usize]
    }

    fn mask(offsets: (u8, u8)) -> u32 {
        (1 << (TOTAL_CELLS - 1)) >> Self::offset_to_action(offsets)
    }

    pub fn get_value(&self, offsets: (u8, u8)) -> bool {
        let mask = Self::mask(offsets);
        (self.board & mask) != 0
    }

    pub fn toggle(&mut self, offsets: (u8, u8)) {
        let mask = Self::mask(offsets);
        self.board ^= mask;
    }

    pub fn is_legal(&self, action: u8) -> bool {
        if action == SKIP_ACTION { return true; }

        let (x_offset, y_offset) = Jigsaw::action_to_offsets(action);
        assert!(x_offset < M && y_offset < N);
        let figure = self.figure();
        
        let mut ilegal = false;

        // here we try not breaking the CPU pipeline since data is probably already
        // in cache and these operations are blazingly fast in any CPU, thus not
        // breaking the pipeline could help us to get more performance rather than
        // using less instructions.
        ilegal |= x_offset > figure.max_offset.0;
        ilegal |= y_offset > figure.max_offset.1;
        ilegal |= (self.board & figure.value >> (x_offset * N + y_offset)) != 0;

        !ilegal
    }

    pub fn legal_actions(&self) -> Vec<u8> {
        (0..TOTAL_ACTIONS).filter(|&x| self.is_legal(x)).collect()
    }

    pub fn action_to_offsets(action: u8) -> (u8, u8) {
        // given an action in range [0, SKIP_ACTION] represented as 
        // 0bXXXYY in binary, we can use the 2 LSB to get x-offset
        // and the 3 MSB to get y-offset.
        // Note: this does not work for all board sizes, but it does
        // work for fishing-jigsaw board, which has size (4, 6).
        let (x_offset, y_offset) = (action >> 2, action & 0b11);
        (x_offset, y_offset)
    }

    pub fn offset_to_action(offsets: (u8, u8)) -> u8 {
        offsets.0 * N + offsets.1
    }

    pub fn fig_intesect(&self, action: u8, offsets: (u8, u8)) -> bool {
        let f = self.figure().value >> action;
        let m = Self::mask(offsets);
        (f & m) != 0
    }
}

#[cfg(test)]
mod jigsaw_test {
    use super::*;

    #[test]
    fn test_skip_action() {
        let mut state = Jigsaw::default();

        assert_eq!(state.round, 0);
        state.perform_action(SKIP_ACTION);
        assert_eq!(state.round, 1);
    }

    #[test]
    fn test_overlap() {
        let mut state = Jigsaw::default();
        state.figure = 1;

        assert!(state.is_legal(0));
        state.perform_action(0);

        state.figure = 0;
        assert!(!state.is_legal(0));
        assert!(!state.is_legal(1));
        assert!(!state.is_legal(2));

        assert!(state.is_legal(3));
    }
}

impl fmt::Debug for Jigsaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<--------{}-------->", self.round)?;
        
        for i in 0..N {
            // write board first row
            let mut mask = 1 << (N * M - (i + 1));
            for j in 0..M {
                let action = i + j * N;

                let value = (self.board & mask) != 0;
                assert!(!(value && self.is_legal(action)));
                if value {
                    write!(f, "🟥")?;
                }
                else if self.is_legal(action) {
                    write!(f, "🟩")?;
                }
                else {
                    write!(f, "🟨")?;
                }
                mask >>= N;
            }
            
            // write figure first row
            let mut mask = 1 << (N * M - (i + 1));
            for _ in 0..M {
                let value = (self.figure().value & mask) != 0;
                if value {
                    write!(f, "🔳")?;
                }
                else {
                    write!(f, "  ")?;
                }
                mask >>= N;
            }

            write!(f, "\n")?;
        }
        
        Ok(())
    }
}

impl Default for Jigsaw {
    fn default() -> Self {
        Self {
            board: INIT_STATE,
            figure: 0,
            round: 0,
        }
    }
}

