use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crate::renderer::render;

pub struct App {
    running: bool,
    pub fps: i32
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            fps: 0
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.running = true;

        let target_frame_time = Duration::from_secs_f32(1.0 / 60.0);
        let mut timer = Instant::now();
        let mut fps = 0;

        while self.running {
            let frame_start = Instant::now();

            while poll(Duration::ZERO)? {
                let event = read()?;
                self.handle_event(event);
            }

            render(self)?;
            fps += 1;

            let frame_time = Instant::now().duration_since(frame_start);
            if frame_time < target_frame_time {
                sleep(target_frame_time - frame_time);
            }

            if (Instant::now() - timer).as_millis() > 1000 {
                timer = Instant::now();
                self.fps = fps;
                fps = 0;
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
