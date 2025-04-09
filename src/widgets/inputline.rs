use crate::core::rect::TRect;
use crate::core::event::{TEvent, TEventQueue};
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::event::{KeyCode, KeyEvent};

pub struct TInputLine {
    pub text: String,
    pub bounds: TRect,
    pub cursor_pos: usize,
    pub focused: bool,
}

impl TInputLine {
    pub fn new(bounds: TRect) -> Self {
        Self {
            text: String::new(),
            bounds,
            cursor_pos: 0,
            focused: true,
        }
    }

    pub fn get_value(&self) -> &str {
        &self.text
    }

    pub fn set_value(&mut self, value: &str) {
        self.text = value.to_string();
        self.cursor_pos = self.text.len();
    }
}

impl TView for TInputLine {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x = offset.0 + self.bounds.x;
        let y = offset.1 + self.bounds.y;
        let width = self.bounds.width as usize;

        // Ensure display is exactly width characters
        let mut line = self.text.clone();
        if line.len() > width {
            line = line[self.cursor_pos.saturating_sub(width - 1)..self.cursor_pos].to_string();
        } else {
            line = format!("{:<width$}", line, width = width);
        }

        buffer.write_str(x, y, &line);

        // Optional: show cursor (if focused and within bounds)
        if self.focused {
            let cursor_x = x + self.cursor_pos.min(self.bounds.width as usize) as u16;
            if cursor_x < x + self.bounds.width {
                buffer.set(cursor_x, y, '_');
            }
        }
    }

    fn handle_event(&mut self, event: TEvent, queue: &TEventQueue) {
        if let TEvent::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Char(c) => {
                    self.text.insert(self.cursor_pos, c);
                    self.cursor_pos += 1;
                }
                KeyCode::Backspace => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                        self.text.remove(self.cursor_pos);
                    }
                }
                KeyCode::Delete => {
                    if self.cursor_pos < self.text.len() {
                        self.text.remove(self.cursor_pos);
                    }
                }
                KeyCode::Left => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.cursor_pos < self.text.len() {
                        self.cursor_pos += 1;
                    }
                }
                KeyCode::End => {
                    self.cursor_pos = self.text.len();
                }
                KeyCode::Home => {
                    self.cursor_pos = 0;
                }
                _ => {}
            }
        }
    }

    fn get_bounds(&self) -> TRect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: TRect) {
        self.bounds = bounds;
    }

    fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }
    fn is_focusable(&self) -> bool {
        true
    }
}
