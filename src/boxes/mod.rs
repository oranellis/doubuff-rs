use box_characters::{thick_border_from_directions, thin_rounded_border_from_directions};
use terminal_vec2::Vec2;

mod box_characters;

pub struct TerminalBoxes {
    terminal_size: Vec2,
    border_bits: Vec<Vec<bool>>,
    border_fn: fn(bool, bool, bool, bool) -> char,
}

impl TerminalBoxes {
    pub fn new_thin(terminal_size: &Vec2) -> TerminalBoxes {
        TerminalBoxes {
            terminal_size: *terminal_size,
            border_bits: vec![vec![false; terminal_size.col as usize]; terminal_size.row as usize],
            border_fn: thin_rounded_border_from_directions,
        }
    }

    pub fn new_thick(terminal_size: &Vec2) -> TerminalBoxes {
        TerminalBoxes {
            terminal_size: *terminal_size,
            border_bits: vec![vec![false; terminal_size.col as usize]; terminal_size.row as usize],
            border_fn: thick_border_from_directions,
        }
    }

    pub fn add_box(&mut self, pos: Vec2, size: Vec2) -> &mut Self {
        for column in pos.col..(pos.col + size.col) {
            let row_top = pos.row;
            let row_bottom = pos.row + size.row.saturating_sub(1);
            self.set_bit(column, row_top);
            self.set_bit(column, row_bottom);
        }
        for row in pos.row..(pos.row + size.row) {
            let column_left = pos.col;
            let column_right = pos.col + size.col.saturating_sub(1);
            self.set_bit(column_left, row);
            self.set_bit(column_right, row);
        }
        self
    }

    fn set_bit(&mut self, column: u16, row: u16) {
        if column < self.terminal_size.col && row < self.terminal_size.row {
            self.border_bits[row as usize][column as usize] = true;
        }
    }

    fn check_bit(&self, column: u16, row: u16) -> bool {
        if column < self.terminal_size.col && row < self.terminal_size.row {
            self.border_bits[row as usize][column as usize]
        } else {
            false
        }
    }

    fn has_north(&self, column: u16, row: u16) -> bool {
        if row == 0 {
            return false;
        }
        self.border_bits[(row - 1) as usize][column as usize]
    }

    fn has_east(&self, column: u16, row: u16) -> bool {
        if column == self.terminal_size.col - 1 {
            return false;
        }
        self.border_bits[row as usize][(column + 1) as usize]
    }

    fn has_south(&self, column: u16, row: u16) -> bool {
        if row == self.terminal_size.row - 1 {
            return false;
        }
        self.border_bits[(row + 1) as usize][column as usize]
    }

    fn has_west(&self, column: u16, row: u16) -> bool {
        if column == 0 {
            return false;
        }
        self.border_bits[row as usize][(column - 1) as usize]
    }
}

impl std::fmt::Display for TerminalBoxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_string = String::with_capacity(self.border_bits.len());
        for row in 0..self.border_bits.len() as u16 {
            for column in 0..self.border_bits[0].len() as u16 {
                if self.check_bit(column, row) {
                    display_string.push((self.border_fn)(
                        self.has_north(column, row),
                        self.has_east(column, row),
                        self.has_south(column, row),
                        self.has_west(column, row),
                    ));
                } else {
                    display_string.push('�');
                }
            }
        }
        write!(f, "{}", display_string)
    }
}
