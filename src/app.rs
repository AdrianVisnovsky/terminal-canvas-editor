use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::event::{poll, read, Event};
use crossterm::{terminal};
use crate::app_event;
use crate::app_event::AppEvent;
use crate::canvas::Canvas;
use crate::cursor::Cursor;
use crate::renderer::render;

pub struct App {
    running: bool,
    pub width: u16,
    pub height: u16,
    pub cursor: Cursor,
    pub cursor_visible: bool,
    pub cursor_blink_timer: Instant,
    pub canvas: Canvas,
    pub fps: Option<i32>
}

impl App {
    pub fn new() -> std::io::Result<Self> {
        let (width, height) = terminal::size()?;

        Ok(Self {
            running: false,
            width,
            height,
            cursor: Cursor::new(),
            cursor_visible: false,
            cursor_blink_timer: Instant::now(),
            canvas: Canvas::new(width, height),
            fps: None
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.running = true;
        let mut needs_clear = true;

        let target_frame_time = Duration::from_secs_f32(1.0 / 60.0);
        let mut timer = Instant::now();
        let mut fps = 0;

        while self.running {
            let frame_start = Instant::now();

            if self.cursor_blink_timer.elapsed() > Duration::from_millis(500) {
                self.cursor_visible = !self.cursor_visible;
                self.cursor_blink_timer = Instant::now();
            }

            needs_clear |= self.check_resize()?;

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
        let Some(app_event) = app_event::parse_event(event) else {
            return;
        };

        match app_event {
            AppEvent::Quit => self.running = false,
            AppEvent::MoveCursor(direction) => {
                self.cursor.move_direction(direction);
                self.cursor.clamp_cursor(self.width, self.height);
                if self.cursor.pen_down {
                    self.canvas.draw(self.cursor.x, self.cursor.y, self.cursor.brush)
                }
            },
            AppEvent::TogglePen => {
                self.cursor.toggle_pen();
                if self.cursor.pen_down {
                    self.canvas.draw(self.cursor.x, self.cursor.y, self.cursor.brush)
                }
            },
            AppEvent::ClearCanvas => {
                self.canvas.clear();
            }
        }
    }

    fn check_resize(&mut self) -> std::io::Result<bool> {
        let (new_width, new_height) = terminal::size()?;
        if new_width != self.width || new_height != self.height {
            self.width = new_width;
            self.height = new_height;
            self.canvas.resize(new_width, new_height);
            self.cursor.clamp_cursor(self.width, self.height);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
