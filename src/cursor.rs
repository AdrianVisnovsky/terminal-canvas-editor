use crate::app_event::Direction;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub pen_down: bool,
    pub brush: char
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            pen_down: false,
            brush: '#',
        }
    }

    pub fn clamp_cursor(&mut self, width: u16, height: u16) {
        self.x = self.x.min(width.saturating_sub(3));
        self.y = self.y.min(height.saturating_sub(5));
    }

    pub fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }
    
    pub fn toggle_pen(&mut self) {
        self.pen_down = !self.pen_down;
    }

    fn move_up(&mut self) {
        self.y = self.y.saturating_sub(1);
    }

    fn move_down(&mut self) {
        self.y = self.y.saturating_add(1);
    }

    fn move_left(&mut self) {
        self.x = self.x.saturating_sub(1);
    }

    fn move_right(&mut self) {
        self.x = self.x.saturating_add(1);
    }
}
