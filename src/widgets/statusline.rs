use crate::core::rect::TRect;
use crate::core::event::TEvent;
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;

pub struct TStatusLine {
    pub text: String,
    pub bounds: TRect,
}

impl TStatusLine {
    pub fn new(text: &str, bounds: TRect) -> Self {
        Self {
            text: text.to_string(),
            bounds,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl TView for TStatusLine {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x = offset.0 + self.bounds.x;
        let y = offset.1 + self.bounds.y;
        let width = self.bounds.width as usize;

        let line = if self.text.len() > width {
            self.text[..width].to_string()
        } else {
            format!("{:<width$}", self.text, width = width)
        };

        buffer.write_str(x, y, &line);
    }

    fn handle_event(&mut self, _event: TEvent) {
        // passive; no events
    }

    fn get_bounds(&self) -> TRect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: TRect) {
        self.bounds = bounds;
    }

    fn set_focus(&mut self, _focused: bool) {
        // do nothing
    }
    fn is_focusable(&self) -> bool {
        true
    }

    // set_focus not overridden — default no-op
}
