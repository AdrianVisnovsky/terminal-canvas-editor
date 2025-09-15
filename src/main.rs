mod app;
mod renderer;
mod canvas;

use std::io;
use std::io::stdout;
use crossterm::{cursor, ExecutableCommand};
use crossterm::cursor::{Hide};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use app::App;

fn main() -> io::Result<()> {

    stdout()
        .execute(EnterAlternateScreen)?
        .execute(Hide)?;
    enable_raw_mode()?;

    let mut app = App::new();
    app.run()?;

    disable_raw_mode()?;
    stdout()
        .execute(LeaveAlternateScreen)?
        .execute(cursor::Show)?;

    Ok(())
}
