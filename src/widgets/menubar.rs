use crate::core::rect::TRect;
use crate::core::event::{TEvent, TEventQueue};
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub hotkey: Option<char>,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        let hotkey = label
            .find('&')
            .and_then(|i| label.chars().nth(i + 1))
            .map(|c| c.to_ascii_lowercase());

        let clean_label = label.replace('&', "");

        Self {
            label: clean_label,
            hotkey,
        }
    }
}

pub struct TMenuBar {
    items: Vec<MenuItem>,
    bounds: TRect,
    active_index: Option<usize>,
    is_active: bool,
}

impl TMenuBar {
    pub fn new(items: Vec<MenuItem>, bounds: TRect) -> Self {
        Self {
            items,
            bounds,
            active_index: None,
            is_active: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.active_index = Some(0);
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.active_index = None;
    }

    fn move_left(&mut self) {
        if let Some(idx) = self.active_index {
            let new_idx = if idx == 0 { self.items.len() - 1 } else { idx - 1 };
            self.active_index = Some(new_idx);
        }
    }

    fn move_right(&mut self) {
        if let Some(idx) = self.active_index {
            let new_idx = (idx + 1) % self.items.len();
            self.active_index = Some(new_idx);
        }
    }
}

impl TView for TMenuBar {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x0 = offset.0 + self.bounds.x;
        let y0 = offset.1 + self.bounds.y;

        let mut x = x0;

        for (i, item) in self.items.iter().enumerate() {
            let selected = self.is_active && self.active_index == Some(i);

            // Add space before the item
            buffer.set(x, y0, ' ');
            x += 1;

            // Render label with optional hotkey highlight
            let mut chars = item.label.chars();
            let mut underline_done = false;

            for ch in chars.by_ref() {
                let is_hotkey = item
                    .hotkey
                    .map(|hk| hk == ch.to_ascii_lowercase())
                    .unwrap_or(false);

                let rendered = if is_hotkey && !underline_done {
                    // Optionally mark with underline or standout logic here
                    underline_done = true;
                    ch.to_ascii_uppercase() // simple "visual" marker
                } else {
                    ch
                };

                if selected {
                    buffer.set(x, y0, rendered);
                } else {
                    buffer.set(x, y0, rendered);
                }

                x += 1;
            }

            // Add space after the item
            buffer.set(x, y0, ' ');
            x += 1;
        }

        // Fill the rest of the line
        while x < x0 + self.bounds.width {
            buffer.set(x, y0, ' ');
            x += 1;
        }
    }

    fn handle_event(&mut self, event: TEvent, queue: &TEventQueue) {
        if let TEvent::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::F(10) => {
                    if self.is_active {
                        self.deactivate();
                    } else {
                        self.activate();
                    }
                }
                KeyCode::Left if self.is_active => {
                    self.move_left();
                }
                KeyCode::Right if self.is_active => {
                    self.move_right();
                }
                KeyCode::Esc if self.is_active => {
                    self.deactivate();
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

    fn set_focus(&mut self, _focused: bool) {
        // Passive: does not respond to focus directly
    }

    fn is_focusable(&self) -> bool {
        false
    }
}
