use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub enum AppEvent {
    Quit,
    MoveCursor(Direction),
    TogglePen,
    ClearCanvas,
    SetBrush(char)
}

pub enum Direction {
    Up, Down, Left, Right
}

pub fn parse_event(event: Event) -> Option<AppEvent> {
    match event {
        Event::Key(KeyEvent { code, kind, ..}) => {
            if kind != KeyEventKind::Press {
                return None;
            }

            match code {
                KeyCode::Char('q') => Some(AppEvent::Quit),
                KeyCode::Char('c') => Some(AppEvent::ClearCanvas),
                KeyCode::Char(' ') => Some(AppEvent::TogglePen),
                KeyCode::Up => Some(AppEvent::MoveCursor(Direction::Up)),
                KeyCode::Down => Some(AppEvent::MoveCursor(Direction::Down)),
                KeyCode::Left => Some(AppEvent::MoveCursor(Direction::Left)),
                KeyCode::Right => Some(AppEvent::MoveCursor(Direction::Right)),
                KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
                    let index = (c as usize) - ('1' as usize);
                    Some(AppEvent::SetBrush(crate::cursor::BRUSHES[index]))
                },
                _ => None
            }
        }
        _ => None
    }
}
