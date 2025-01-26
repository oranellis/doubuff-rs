use std::io::{stdout, Write};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    style::{Attribute, Print, SetAttribute, SetColors},
    QueueableCommand,
};
use terminal_vec2::{vec2, Vec2};

use crate::{
    boxes::TerminalBoxes,
    styled_char::{add_style_to_string, StyledChar},
};

#[derive(Debug, PartialEq)]
pub struct TerminalBuffer {
    pub buffer: Vec<StyledChar>,
    pub scratch_buffer: Vec<StyledChar>,
    pub buffer_size: Vec2,
    pub cursor_pos: usize,
}

impl TerminalBuffer {
    pub fn new(buffer_size: Vec2) -> Self {
        Self {
            buffer: vec!['�'.into(); buffer_size.col as usize * buffer_size.row as usize],
            scratch_buffer: vec![' '.into(); buffer_size.col as usize * buffer_size.row as usize],
            buffer_size,
            cursor_pos: 0,
        }
    }

    pub fn move_cursor(&mut self, pos: Vec2) -> Result<&mut Self, &str> {
        if pos.col > self.buffer_size.col || pos.row > self.buffer_size.row {
            return Err("cursor outside of terminal bounds");
        }
        self.cursor_pos = (pos.row as usize * self.buffer_size.col as usize) + pos.col as usize;
        Ok(self)
    }

    pub fn styled_print<I, T>(&mut self, iterable: I) -> Result<&mut Self, &str>
    where
        I: IntoIterator<Item = T>,
        T: Into<StyledChar>,
    {
        for sc in iterable {
            if let Some(mc) = self.scratch_buffer.get_mut(self.cursor_pos) {
                let sc = sc.into();
                if sc.character != '�' {
                    *mc = sc;
                }
                self.cursor_pos += 1;
            } else {
                break;
            }
        }
        Ok(self)
    }

    pub fn draw_box(&mut self, boxes: TerminalBoxes) -> Result<&mut Self, &str> {
        let styled_str = add_style_to_string(
            &boxes.to_string(),
            crossterm::style::Color::Reset,
            crossterm::style::Color::Reset,
            false,
        );
        self.move_cursor(vec2!(0, 0)?)?.styled_print(styled_str)
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        let mut stdout = stdout();
        stdout.queue(MoveTo(0, 0))?;
        for (i, c) in self.scratch_buffer.iter().enumerate() {
            if i != 0 && i % self.buffer_size.col as usize == 0 {
                stdout.queue(MoveToNextLine(1))?;
            }
            if c.bold {
                stdout.queue(SetAttribute(Attribute::Bold))?;
            } else {
                stdout.queue(SetAttribute(Attribute::Reset))?;
            }
            stdout.queue(SetColors(c.colour))?;
            stdout.queue(Print(c.character))?;
        }
        stdout.flush()?;
        self.buffer = std::mem::replace(
            &mut self.scratch_buffer,
            vec![' '.into(); self.buffer_size.col as usize * self.buffer_size.row as usize],
        );
        Ok(())
    }
}
