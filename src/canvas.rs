pub struct Canvas {
    pub data: Vec<Vec<char>>,
    width: u16,
    height: u16,
}

impl Canvas {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Canvas {
        let (width, height) = Self::calculate_dimensions(terminal_width, terminal_height);
        Self {
            data: vec![vec![' '; width as usize]; height as usize],
            width,
            height,
        }
    }

    pub fn resize(&mut self, terminal_width: u16, terminal_height: u16) {
        let old_data = std::mem::take(&mut self.data);
        let (new_width, new_height) = Self::calculate_dimensions(terminal_width, terminal_height);

        self.data = vec![vec![' '; new_width as usize]; new_height as usize];
        self.width = new_width;
        self.height = new_height;

        for (y, row) in old_data.iter().enumerate() {
            if y >= self.height as usize { break; }
            for (x, &ch) in row.iter().enumerate() {
                 if x >= self.width as usize { break; }
                self.data[y][x] = ch;
            }
        }
    }

    pub fn draw(&mut self, x_position: u16, y_position: u16, c: char) {
        if x_position < self.width && y_position < self.height {
            self.data[y_position as usize][x_position as usize] = c;
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.data {
            row.fill(' ')
        }
    }

    fn calculate_dimensions(terminal_width: u16, terminal_height: u16) -> (u16, u16) {
        (
            terminal_width.saturating_sub(2),
            terminal_height.saturating_sub(4)
        )
    }
}