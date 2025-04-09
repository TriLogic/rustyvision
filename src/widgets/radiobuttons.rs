use crate::core::rect::TRect;
use crate::core::event::{TEvent, TEventQueue};
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::event::{KeyCode, KeyEvent};

pub struct TRadioButtons {
    pub items: Vec<String>,
    pub selected_index: usize,
    pub bounds: TRect,
    pub focus_index: usize,
    pub focused: bool,
}

impl TRadioButtons {
    pub fn new(bounds: TRect, labels: Vec<&str>) -> Self {
        Self {
            items: labels.into_iter().map(String::from).collect(),
            selected_index: 0,
            focus_index: 0,
            bounds,
            focused: true,
        }
    }

    pub fn selected_item(&self) -> Option<&str> {
        self.items.get(self.selected_index).map(String::as_str)
    }
}

impl TView for TRadioButtons {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x = offset.0 + self.bounds.x;
        let y = offset.1 + self.bounds.y;

        for (i, label) in self.items.iter().enumerate() {
            let mark = if i == self.selected_index { "(*)" } else { "( )" };
            let focus_marker = if self.focused && i == self.focus_index { "▶" } else { " " };
            let line = format!("{} {} {}", focus_marker, mark, label);
            if (i as u16) < self.bounds.height {
                buffer.write_str(x, y + i as u16, &line);
            }
        }
    }

    fn handle_event(&mut self, event: TEvent, queue: &TEventQueue) {
        if let TEvent::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Up => {
                    if self.focus_index > 0 {
                        self.focus_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.focus_index + 1 < self.items.len() {
                        self.focus_index += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    self.selected_index = self.focus_index;
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
