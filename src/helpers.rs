use crossterm::{
    cursor::{Hide, Show},
    style::ResetColor,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Write};
use terminal_vec2::Vec2;

pub fn start_display() -> std::io::Result<()> {
    stdout()
        .queue(SetTitle("mmm"))?
        .queue(EnterAlternateScreen)?
        .queue(ResetColor)?
        .queue(Clear(crossterm::terminal::ClearType::All))?
        .flush()?;
    enable_raw_mode()?;
    hide_cursor()?;
    Ok(())
}

pub fn stop_display() -> std::io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    show_cursor()?;
    disable_raw_mode()?;
    Ok(())
}

pub fn get_terminal_size() -> std::io::Result<Vec2> {
    let (rows, columns) = terminal::size()?;
    Ok(Vec2 {
        col: columns,
        row: rows,
    })
}

fn hide_cursor() -> std::io::Result<()> {
    stdout().queue(Hide)?;
    Ok(())
}

fn show_cursor() -> std::io::Result<()> {
    stdout().queue(Show)?;
    Ok(())
}
