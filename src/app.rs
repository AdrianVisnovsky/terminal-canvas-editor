use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal;
use crate::renderer::render;

pub struct App {
    running: bool,
    pub width: u16,
    pub height: u16,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub fps: Option<i32>
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            width: 0,
            height: 0,
            cursor_x: 0,
            cursor_y: 0,
            fps: None
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.running = true;
        let mut needs_clear = true;

        let target_frame_time = Duration::from_secs_f32(1.0 / 60.0);
        let mut timer = Instant::now();
        let mut fps = 0;

        while self.running {
            let frame_start = Instant::now();

            let (new_width, new_height) = terminal::size()?;
            if new_width != self.width || new_height != self.height {
                self.width = new_width;
                self.height = new_height;
                needs_clear = true;
            }

            while poll(Duration::ZERO)? {
                let event = read()?;
                self.handle_event(event);
            }

            render(self, needs_clear)?;
            needs_clear = false;

            fps += 1;

            let frame_time = Instant::now().duration_since(frame_start);
            if frame_time < target_frame_time {
                sleep(target_frame_time - frame_time);
            }

            if (Instant::now() - timer).as_millis() > 1000 {
                timer = Instant::now();
                self.fps = Some(fps);
                fps = 0;
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent { code, kind, ..}) => {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('q') => self.running = false,
                        KeyCode::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
                        KeyCode::Down => self.cursor_y += self.cursor_y.saturating_add(1).max(self.height - 2),
                        KeyCode::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
                        KeyCode::Right => self.cursor_x += self.cursor_x.saturating_add(1).max(self.width - 2),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
