use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Attribute, Colors, Print, SetAttribute, SetColors},
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
        // Helper struct to store write chunks
        struct StyledChunk {
            pub chars: String,
            pub position: Vec2,
            pub colour: Colors,
            pub bold: bool,
        }

        // Helper function to create working style chunks
        fn wsc_from_char_and_index(
            nc: &StyledChar,
            index: usize,
            cols: u16,
        ) -> Option<StyledChunk> {
            Some(StyledChunk {
                chars: nc.character.to_string(),
                position: vec2!(index as u16 % cols, index as u16 / cols)
                    .expect("Could not make position vec2"),
                colour: nc.colour,
                bold: nc.bold,
            })
        }

        // Old buffer
        let ob_iter = self.buffer.iter();
        // New buffer
        let nb_iter = self.scratch_buffer.iter();

        let mut write_chunks: Vec<StyledChunk> = Vec::new();
        let mut working_styled_chunk: Option<StyledChunk> = None;

        let changed_chars: Vec<Option<&StyledChar>> = ob_iter
            .zip(nb_iter)
            .map(|(oc, nc)| Some(nc).filter(|&nchar| nchar != oc))
            .collect();

        for (index, newchar_option) in changed_chars.into_iter().enumerate() {
            match (newchar_option, working_styled_chunk.as_mut()) {
                (Some(nc), None) => {
                    working_styled_chunk = wsc_from_char_and_index(nc, index, self.buffer_size.col);
                }
                (Some(nc), Some(wsc)) => {
                    if nc.colour != wsc.colour || nc.bold != wsc.bold {
                        working_styled_chunk.take().map(|sc| write_chunks.push(sc));
                        working_styled_chunk =
                            wsc_from_char_and_index(nc, index, self.buffer_size.col);
                    } else {
                        wsc.chars.push(nc.character);
                    }
                }
                (None, _) => {
                    working_styled_chunk.take().map(|sc| write_chunks.push(sc));
                }
            }
        }
        working_styled_chunk.take().map(|sc| write_chunks.push(sc));

        let mut stdout = stdout();
        for write_chunk in write_chunks {
            stdout.queue(MoveTo(write_chunk.position.col, write_chunk.position.row))?;
            if write_chunk.bold {
                stdout.queue(SetAttribute(Attribute::Bold))?;
            } else {
                stdout.queue(SetAttribute(Attribute::Reset))?;
            }
            stdout.queue(SetColors(write_chunk.colour))?;
            stdout.queue(Print(write_chunk.chars))?;
        }
        stdout.flush()?;

        self.buffer = std::mem::replace(
            &mut self.scratch_buffer,
            vec![' '.into(); self.buffer_size.col as usize * self.buffer_size.row as usize],
        );
        Ok(())
    }
}
