use crate::core::rect::TRect;
use crate::core::event::{TEvent, TEventQueue};
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::event::{KeyCode, KeyEvent};

pub struct TCheckBoxes {
    pub items: Vec<(String, bool)>, // (label, is_checked)
    pub bounds: TRect,
    pub selected: usize,
    pub focused: bool,
}

impl TCheckBoxes {
    pub fn new(bounds: TRect, labels: Vec<&str>) -> Self {
        Self {
            items: labels.into_iter().map(|s| (s.to_string(), false)).collect(),
            bounds,
            selected: 0,
            focused: true,
        }
    }

    pub fn is_checked(&self, index: usize) -> bool {
        self.items
            .get(index)
            .map(|(_, checked)| *checked)
            .unwrap_or(false)
    }
}

impl TView for TCheckBoxes {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x = offset.0 + self.bounds.x;
        let y = offset.1 + self.bounds.y;

        for (i, (label, checked)) in self.items.iter().enumerate() {
            let checkmark = if *checked { "[X]" } else { "[ ]" };
            let indicator = if self.focused && i == self.selected { "▶" } else { " " };
            let line = format!("{} {} {}", indicator, checkmark, label);
            if (i as u16) < self.bounds.height {
                buffer.write_str(x, y + i as u16, &line);
            }
        }
    }

    fn handle_event(&mut self, event: TEvent, queue: &TEventQueue) {
        if let TEvent::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.selected + 1 < self.items.len() {
                        self.selected += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if let Some((_, checked)) = self.items.get_mut(self.selected) {
                        *checked = !*checked;
                    }
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

}
