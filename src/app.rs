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
    pub cursor_visible: bool,
    pub cursor_blink_timer: Instant,
    pub canvas: Vec<Vec<char>>,
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
            cursor_visible: false,
            cursor_blink_timer: Instant::now(),
            canvas: Vec::new(),
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
        match event {
            Event::Key(KeyEvent { code, kind, ..}) => {
                if kind == KeyEventKind::Press {
                    match code {
                        KeyCode::Char('q') => self.running = false,
                        KeyCode::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
                        KeyCode::Down => self.cursor_y = self.cursor_y.saturating_add(1),
                        KeyCode::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
                        KeyCode::Right => self.cursor_x = self.cursor_x.saturating_add(1),
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        self.clamp_cursor();
    }

    fn clamp_cursor(&mut self) {
        self.cursor_x = self.cursor_x.min(self.width.saturating_sub(3));
        self.cursor_y = self.cursor_y.min(self.height.saturating_sub(5));
    }

    fn resize_canvas(&mut self, old_canvas: Vec<Vec<char>>) {
        let canvas_width = self.width.saturating_sub(2);
        let canvas_height = self.height.saturating_sub(4);

        self.canvas = vec![vec![' '; canvas_width as usize]; canvas_height as usize];

        for (y, row) in old_canvas.iter().enumerate() {
            if y >= canvas_height as usize { break; }

            for (x, &ch) in row.iter().enumerate() {
                if x >= canvas_width as usize { break; }

                self.canvas[y][x] = ch;
            }
        }
    }

    fn check_resize(&mut self) -> std::io::Result<bool> {
        let (new_width, new_height) = terminal::size()?;

        if new_width != self.width || new_height != self.height {
            let old_canvas = std::mem::take(&mut self.canvas);

            self.width = new_width;
            self.height = new_height;

            self.resize_canvas(old_canvas);
            self.clamp_cursor();

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
