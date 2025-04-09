use crate::core::rect::TRect;
use crate::core::event::{TEvent, TEventQueue};
use crate::core::view::TView;
use crate::core::focus::FocusManager;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::event::KeyCode;

pub struct TFooterBar {
    pub bounds: TRect,
    pub children: Vec<Box<dyn TView>>,
    pub focus: FocusManager,
}

impl TFooterBar {
    pub fn new(bounds: TRect) -> Self {
        Self {
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
}

impl TView for TFooterBar {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let base_x = offset.0 + self.bounds.x;
        let base_y = offset.1 + self.bounds.y;

        for child in &self.children {
            child.draw(buffer, (base_x, base_y));
        }
    }

    fn handle_event(&mut self, event: TEvent, queue: &TEventQueue) {
        match &event {
            TEvent::Key(key) => match key.code {
                KeyCode::Tab => {
                    self.focus.focus_next(&mut self.children);
                    return;
                }
                KeyCode::BackTab => {
                    self.focus.focus_prev(&mut self.children);
                    return;
                }
                _ => {}
            },
            _ => {}
        }

        if let Some(child) = self.focus.current_mut(&mut self.children) {
            child.handle_event(event, queue);
        }
    }

    fn get_bounds(&self) -> TRect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: TRect) {
        self.bounds = bounds;
    }

    fn is_focusable(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool) {
        if let Some(child) = self.focus.current_mut(&mut self.children) {
            child.set_focus(focused);
        }
    }
}