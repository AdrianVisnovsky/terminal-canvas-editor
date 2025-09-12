use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

pub struct App {
    running: bool,
    fps: f32
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            fps: 0f32
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.running = true;

        while self.running {
            if poll(Duration::from_millis(16))? {
                let event = read()?;
                self.handle_event(event);
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent { code, ..}) => {
                match code {
                    KeyCode::Char('q') => self.running = false,
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
