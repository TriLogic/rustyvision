use crate::core::rect::TRect;
use crate::core::event::TEvent;
use crate::core::view::TView;
use crate::core::focus::FocusManager;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::event::KeyCode;

pub struct TDialog {
    pub title: String,
    pub bounds: TRect,
    pub children: Vec<Box<dyn TView>>,
    pub focus: FocusManager,
}

impl TDialog {
    pub fn new(title: &str, bounds: TRect) -> Self {
        Self {
            title: title.to_string(),
            bounds,
            children: Vec::new(),
            focus: FocusManager::new(),
        }
    }

    pub fn add_child(&mut self, mut child: Box<dyn TView>) {
        if self.children.is_empty() && child.is_focusable() {
            child.set_focus(true);
        }
        self.children.push(child);
    }

    fn draw_border(&self, buffer: &mut ScreenBuffer) {
        let x = self.bounds.x;
        let y = self.bounds.y;
        let w = self.bounds.width;
        let h = self.bounds.height;

        buffer.write_str(x, y, &format!("┌{}┐", "─".repeat((w - 2) as usize)));
        for i in 1..(h - 1) {
            buffer.set(x, y + i, '│');
            buffer.set(x + w - 1, y + i, '│');
        }
        buffer.write_str(x, y + h - 1, &format!("└{}┘", "─".repeat((w - 2) as usize)));

        // Draw title centered
        let title = format!(" {} ", self.title);
        let start_x = x + (w.saturating_sub(title.len() as u16)) / 2;
        buffer.write_str(start_x, y, &title);
    }
}

impl TView for TDialog {
    fn draw(&self, buffer: &mut ScreenBuffer, _offset: (u16, u16)) {
        self.draw_border(buffer);

        let offset = (self.bounds.x + 1, self.bounds.y + 1);
        for child in &self.children {
            child.draw(buffer, offset);
        }
    }

    fn handle_event(&mut self, event: TEvent) {
        if let TEvent::Key(key) = &event {
            match key.code {
                KeyCode::Tab => {
                    self.focus.focus_next(&mut self.children);
                    return;
                }
                KeyCode::BackTab => {
                    self.focus.focus_prev(&mut self.children);
                    return;
                }
                _ => {}
            }
        }

        if let Some(focused) = self.focus.current_mut(&mut self.children) {
            focused.handle_event(event);
        }
    }

    fn get_bounds(&self) -> TRect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: TRect) {
        self.bounds = bounds;
    }

    fn set_focus(&mut self, focused: bool) {
        if let Some(child) = self.focus.current_mut(&mut self.children) {
            child.set_focus(focused);
        }
    }

    fn is_focusable(&self) -> bool {
        true
    }
}
