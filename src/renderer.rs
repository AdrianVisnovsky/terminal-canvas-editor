use std::io;
use std::io::Write;
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::{cursor, terminal, ExecutableCommand};
use crossterm::terminal::ClearType;
use crate::app::App;

pub fn render(app: &App) -> io::Result<()> {

    io::stdout()
        .execute(terminal::Clear(ClearType::All))?;

    render_header(app)?;
    render_canvas(app)?;
    render_status_bar(app)?;

    io::stdout().flush()?;
    Ok(())
}

fn render_header(app: &App) -> io::Result<()> {

    let (width, _) = terminal::size()?;
    let fps_text = format!("FPS: {}", app.fps);
    let x_position = width.saturating_sub(fps_text.len() as u16);

    io::stdout()
        .execute(cursor::MoveTo(x_position, 0))?
        .execute(SetForegroundColor(Color::Green))?;

    print!("{}", fps_text);

    io::stdout().execute(ResetColor)?;

    Ok(())
}

fn render_canvas(app: &App) -> io::Result<()> {

    Ok(())
}

fn render_status_bar(app: &App) -> io::Result<()> {

    Ok(())
}
