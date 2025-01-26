use std::io::{stdout, Write};

use crossterm::{
    cursor::{Hide, Show},
    style::ResetColor,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
        SetTitle,
    },
    ExecutableCommand, QueueableCommand,
};

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

fn hide_cursor() -> std::io::Result<()> {
    stdout().queue(Hide)?;
    Ok(())
}

fn show_cursor() -> std::io::Result<()> {
    stdout().queue(Show)?;
    Ok(())
}
