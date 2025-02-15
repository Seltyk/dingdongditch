//! Terminal setup and restoration functions

use std::io::{self, Stdout};

use anyhow::{Context, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// Setup the terminal. This is where you would enable raw mode, enter the
/// alternate screen, and hide the cursor. This example does not handle errors.
/// A more robust application would probably want to handle errors and ensure
/// that the terminal is restored to a sane state before exiting.
pub(crate) fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout))
        .context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the
/// alternate screen, and show the cursor.
pub(crate) fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}
