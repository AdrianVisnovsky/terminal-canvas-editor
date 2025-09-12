use std::io;
use std::io::stdout;
use crossterm::event::{read, KeyCode};
use crossterm::{ExecutableCommand};
use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

fn main() -> io::Result<()> {

    stdout()
        .execute(EnterAlternateScreen)?
        .execute(Hide)?;
    enable_raw_mode()?;

    while let Ok(event) = read() {

        let Some(event) = event.as_key_event() else {
            continue;
        };

        if event.code == KeyCode::Char('q') {
            break;
        }

    }

    disable_raw_mode()?;
    stdout()
        .execute(LeaveAlternateScreen)?
        .execute(Show)?;

    Ok(())
}
