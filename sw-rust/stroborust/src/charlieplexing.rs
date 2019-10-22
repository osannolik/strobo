// const generics would be cool...
const N_COLS: usize = 5;
const N_ROWS: usize = 6;
const N_PINS: usize = N_ROWS;

type Matrix<T> = [[T; N_COLS]; N_ROWS];
pub type PatternMatrix = Matrix<bool>;

pub const ALL_ON: PatternMatrix = [[true; N_COLS]; N_ROWS];
pub const ALL_OFF: PatternMatrix = [[false; N_COLS]; N_ROWS];

const ROW_COL_TO_PIN: Matrix<usize> = [
    [1, 2, 3, 4, 5],
    [0, 2, 3, 4, 5],
    [0, 1, 3, 4, 5],
    [0, 1, 2, 4, 5],
    [0, 1, 2, 3, 5],
    [0, 1, 2, 3, 4],
];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PinLevel {
    HiZ,
    Low,
    High,
}

type PinStates = [PinLevel; N_PINS];
pub const PINS_HIZ: PinStates = [PinLevel::HiZ; N_PINS];
const PINS_HIGH: PinStates = [PinLevel::High; N_PINS];

pub type PinSequence = [PinStates; N_ROWS];
pub const PIN_SEQUENCE_HIZ: PinSequence = [PINS_HIZ; N_ROWS];

fn row_to_pin_states(row: usize, pattern: &[bool]) -> PinStates {
    let mut pin_states = PINS_HIZ;

    for (col, &is_on) in pattern.iter().enumerate() {
        let pin = ROW_COL_TO_PIN[row][col];
        pin_states[pin] = if is_on { PinLevel::High } else { PinLevel::HiZ };
    }

    pin_states[row] = if pattern.contains(&true) {
        PinLevel::Low
    } else {
        PinLevel::HiZ
    };

    return pin_states;
}

fn to_pin_sequence(pattern: PatternMatrix) -> PinSequence {
    let mut seq = [PINS_HIZ; N_ROWS];

    for (row, pattern) in pattern.iter().enumerate() {
        seq[row] = row_to_pin_states(row, pattern);
    }

    return seq;
}

#[derive(Copy, Clone)]
pub struct Sequencer {
    sequence: PinSequence,
    index: usize,
}

impl Sequencer {
    pub fn pins(&self) -> PinStates {
        self.sequence[self.index]
    }

    pub fn next(&mut self) -> Sequencer {
        self.index = (self.index + 1) % self.sequence.len();
        *self
    }
}

impl core::convert::From<PatternMatrix> for Sequencer {
    fn from(item: PatternMatrix) -> Sequencer {
        Sequencer {
            sequence: to_pin_sequence(item),
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use PinLevel::{HiZ, High, Low};

    fn set_diag(m: &mut [[PinLevel; N_PINS]; N_PINS], val: PinLevel) {
        for (i, r) in m.iter_mut().enumerate() {
            r[i] = val;
        }
    }

    #[test]
    fn pin_state_for_row() {
        const ALL_OFF_PATTERN: [bool; N_COLS] = [false; N_COLS];
        const ALL_OFF_PINS: [PinStates; N_PINS] = [PINS_HIZ; N_PINS];

        const ALL_ON_PATTERN: [bool; N_COLS] = [true; N_COLS];
        let mut all_on_pins = [PINS_HIGH; N_PINS];
        set_diag(&mut all_on_pins, Low);

        for row in 0..N_ROWS {
            let all_off_result = row_to_pin_states(row, &ALL_OFF_PATTERN);
            assert_eq!(all_off_result, ALL_OFF_PINS[row]);

            let all_on_result = row_to_pin_states(row, &ALL_ON_PATTERN);
            assert_eq!(all_on_result, all_on_pins[row]);
        }
    }

    #[test]
    fn pin_sequence() {
        const MATRIX_PATTERN: PatternMatrix = [
            [false, false, false, false, false],
            [true, false, false, false, false],
            [true, true, false, false, false],
            [true, true, true, false, false],
            [true, true, true, true, false],
            [true, true, true, true, true],
        ];
        const PIN_SEQUENCE: PinSequence = [
            PINS_HIZ,
            [High, Low, HiZ, HiZ, HiZ, HiZ],
            [High, High, Low, HiZ, HiZ, HiZ],
            [High, High, High, Low, HiZ, HiZ],
            [High, High, High, High, Low, HiZ],
            [High, High, High, High, High, Low],
        ];

        let sequence = Sequencer::from(MATRIX_PATTERN);

        assert_eq!(sequence.pins(), PIN_SEQUENCE);

        for (t, &pin_states) in sequence.pins().iter().enumerate() {
            assert_eq!(pin_states, PIN_SEQUENCE[t]);
        }
    }
}
