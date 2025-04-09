use crate::core::rect::TRect;
use crate::core::event::{TEvent, TEventQueue};
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;

pub struct TLabel {
    pub text: String,
    pub bounds: TRect,
}

impl TLabel {
    pub fn new(text: &str, bounds: TRect) -> Self {
        Self {
            text: text.to_string(),
            bounds,
        }
    }
}

impl TView for TLabel {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x = offset.0 + self.bounds.x;
        let y = offset.1 + self.bounds.y;

        let mut line = self.text.clone();
        if line.len() > self.bounds.width as usize {
            line.truncate(self.bounds.width as usize);
        } else {
            line = format!("{:<width$}", line, width = self.bounds.width as usize);
        }

        buffer.write_str(x, y, &line);
    }

    fn handle_event(&mut self, _event: TEvent, queue: &TEventQueue) {}

    fn get_bounds(&self) -> TRect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: TRect) {
        self.bounds = bounds;
    }

    fn set_focus(&mut self, _focused: bool) {
        // do nothing
    }

}
