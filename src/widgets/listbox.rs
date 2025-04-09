use crate::core::rect::TRect;
use crate::core::event::{TEvent, TEventQueue};
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::event::{KeyCode, KeyEvent};

pub struct TListBox {
    pub items: Vec<String>,
    pub bounds: TRect,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub focused: bool,
}

impl TListBox {
    pub fn new(bounds: TRect, items: Vec<&str>) -> Self {
        Self {
            items: items.into_iter().map(String::from).collect(),
            bounds,
            selected_index: 0,
            scroll_offset: 0,
            focused: true,
        }
    }

    pub fn selected_item(&self) -> Option<&str> {
        self.items.get(self.selected_index).map(String::as_str)
    }

    fn visible_items(&self) -> usize {
        self.bounds.height as usize
    }

    fn ensure_visible(&mut self) {
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + self.visible_items() {
            self.scroll_offset = self.selected_index - self.visible_items() + 1;
        }
    }
}

impl TView for TListBox {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x = offset.0 + self.bounds.x;
        let y = offset.1 + self.bounds.y;

        let visible_count = self.visible_items();
        let lines = self.items.iter()
            .skip(self.scroll_offset)
            .take(visible_count);

        for (i, item) in lines.enumerate() {
            let global_index = self.scroll_offset + i;
            let prefix = if self.focused && global_index == self.selected_index {
                "▶ "
            } else {
                "  "
            };
            let display = format!("{}{}", prefix, item);
            let line = if display.len() > self.bounds.width as usize {
                display[..self.bounds.width as usize].to_string()
            } else {
                format!("{:<width$}", display, width = self.bounds.width as usize)
            };

            if (i as u16) < self.bounds.height {
                buffer.write_str(x, y + i as u16, &line);
            }
        }
    }

    fn handle_event(&mut self, event: TEvent, queue: &TEventQueue) {
        if let TEvent::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Up => {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                        self.ensure_visible();
                    }
                }
                KeyCode::Down => {
                    if self.selected_index + 1 < self.items.len() {
                        self.selected_index += 1;
                        self.ensure_visible();
                    }
                }
                KeyCode::PageUp => {
                    let n = self.visible_items().min(self.selected_index);
                    self.selected_index -= n;
                    self.ensure_visible();
                }
                KeyCode::PageDown => {
                    let n = self.visible_items();
                    self.selected_index = (self.selected_index + n).min(self.items.len() - 1);
                    self.ensure_visible();
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
