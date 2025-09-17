use std::io;
use std::io::Write;
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::{cursor, terminal, ExecutableCommand};
use crossterm::terminal::ClearType;
use crate::app::App;

const TOP_LEFT_BORDER: char = '┌';
const TOP_RIGHT_BORDER: char = '┐';
const BOTTOM_LEFT_BORDER: char = '└';
const BOTTOM_RIGHT_BORDER: char = '┘';
const VERTICAL_BORDER: char = '│';
const HORIZONTAL_BORDER: char = '─';

pub fn render(app: &App, needs_clear: bool) -> io::Result<()> {

    if needs_clear {
        io::stdout().execute(terminal::Clear(ClearType::All))?;
    }

    render_header(app)?;
    render_canvas(app, needs_clear)?;
    render_status_bar(app)?;

    io::stdout().flush()?;
    Ok(())
}

fn render_header(app: &App) -> io::Result<()> {

    let (pen_text, pen_color) = if app.cursor.pen_down {
        (format!("PEN: DOWN [{}]", app.cursor.brush), Color::Green)
    } else {
        (format!("PEN: UP   [{}]", app.cursor.brush), Color::Red)
    };

    io::stdout()
        .execute(cursor::MoveTo(0, 0))?
        .execute(SetForegroundColor(pen_color))?;

    print!("{}", format!("{:<9}", pen_text));

    let fps_text = match app.fps {
        Some(fps) => format!("FPS: {}", fps),
        None => "FPS: --".to_string(),
    };

    let x_position = app.width.saturating_sub(fps_text.len() as u16);

    io::stdout()
        .execute(cursor::MoveTo(x_position, 0))?
        .execute(SetForegroundColor(Color::Green))?;

    print!("{}", fps_text);

    io::stdout().execute(ResetColor)?;

    Ok(())
}

fn render_canvas(app: &App, redraw_border: bool) -> io::Result<()> {

    for y in 2..app.height {
        let canvas_y = (y - 2) as usize;
        if canvas_y < app.canvas.data.len() {
            io::stdout().execute(cursor::MoveTo(1, y))?;

            for (x, &ch) in app.canvas.data[canvas_y].iter().enumerate() {
                if x == app.cursor.x as usize && canvas_y == app.cursor.y as usize {
                    if app.cursor_visible {
                        print!("█");
                    } else {
                        print!("{}", ch);
                    }
                } else {
                    print!("{}", ch);
                }
            }
        }
    }

    if !redraw_border {
        return Ok(());
    }

    io::stdout().execute(cursor::MoveTo(0, 1))?;
    print!("{}", TOP_LEFT_BORDER);
    for _ in 1..app.width - 1 {
        print!("{}", HORIZONTAL_BORDER);
    }
    print!("{}", TOP_RIGHT_BORDER);

    for y in 2..app.height - 2 {
        io::stdout().execute(cursor::MoveTo(0, y))?;
        print!("{}", VERTICAL_BORDER);

        io::stdout().execute(cursor::MoveTo(app.width-1, y))?;
        print!("{}", VERTICAL_BORDER);
    }

    io::stdout().execute(cursor::MoveTo(0, app.height-2))?;
    print!("{}", BOTTOM_LEFT_BORDER);
    for _ in 1..app.width - 1 {
        print!("{}", HORIZONTAL_BORDER);
    }
    print!("{}", BOTTOM_RIGHT_BORDER);

    Ok(())
}

fn render_status_bar(app: &App) -> io::Result<()> {

    let fps_text = format!("[{},{}]", app.cursor.x, app.cursor.y);
    let padded_text = format!("{:>10}", fps_text);
    let x_position = app.width.saturating_sub(10);

    io::stdout()
        .execute(cursor::MoveTo(x_position, app.height - 1))?;

    print!("{}", padded_text);

    Ok(())
}
