use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub enum AppEvent {
    Quit,
    MoveCursor(Direction)
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
                KeyCode::Up => Some(AppEvent::MoveCursor(Direction::Up)),
                KeyCode::Down => Some(AppEvent::MoveCursor(Direction::Down)),
                KeyCode::Left => Some(AppEvent::MoveCursor(Direction::Left)),
                KeyCode::Right => Some(AppEvent::MoveCursor(Direction::Right)),
                _ => None
            }
        }
        _ => None
    }
}
