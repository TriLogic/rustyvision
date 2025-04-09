use crate::core::rect::TRect;
use crate::core::event::TEvent;
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::event::{KeyCode, KeyEvent};

pub struct TButton {
    pub label: String,
    pub bounds: TRect,
    pub focused: bool,
    pub on_press: Option<Box<dyn FnMut()>>,
}

impl TButton {
    pub fn new(text: &str, bounds: TRect) -> Self {
        Self {
            label: text.to_string(),
            bounds,
            focused: false,
            on_press: None,
        }
    }

    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_press = Some(Box::new(callback));
    }

    fn render_label(&self) -> String {
        let content = format!("[{}]", self.label);
        if content.len() >= self.bounds.width as usize {
            content[..self.bounds.width as usize].to_string()
        } else {
            format!("{:<width$}", content, width = self.bounds.width as usize)
        }
    }
}

impl TView for TButton {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x = offset.0 + self.bounds.x;
        let y = offset.1 + self.bounds.y;

        let content = self.render_label();
        buffer.write_str(x, y, &content);
    }

    fn handle_event(&mut self, event: TEvent) {
        if let TEvent::Key(KeyEvent { code: KeyCode::Enter, .. })
            | TEvent::Key(KeyEvent { code: KeyCode::Char(' '), .. }) = event
        {
            if let Some(callback) = &mut self.on_press {
                callback();
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
