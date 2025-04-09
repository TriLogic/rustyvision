use std::fmt::Write;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
}

impl Default for Cell {
    fn default() -> Self {
        Self { ch: ' ' }
    }
}

pub struct ScreenBuffer {
    width: u16,
    height: u16,
    cells: Vec<Cell>,
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); (width as usize) * (height as usize)],
        }
    }

    fn index(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    pub fn set(&mut self, x: u16, y: u16, ch: char) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx].ch = ch;
        }
    }

    pub fn write_str(&mut self, x: u16, y: u16, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            self.set(x + i as u16, y, ch);
        }
    }

    pub fn flush_to_string(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height {
            write!(&mut output, "\x1B[{};1H", y + 1).ok(); // Move cursor
            for x in 0..self.width {
                let ch = self.cells[self.index(x, y)].ch;
                output.push(ch);
            }
        }
        output
    }
}
