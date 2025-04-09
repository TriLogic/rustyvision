use crate::core::rect::TRect;
use crate::core::view::TView;
use crate::core::event::{TEvent, TEventQueue};
use crate::ui::screenbuffer::ScreenBuffer;
use crate::widgets::menu::*;
use crossterm::event::{KeyCode, KeyEvent};

pub struct TDropDownMenu {
    pub menu: TMenu,
    pub bounds: TRect,
    selected_index: usize,
    focused: bool,
}

impl TDropDownMenu {
    pub fn new(menu: TMenu, bounds: TRect) -> Self {
        Self {
            menu,
            bounds,
            selected_index: 0,
            focused: false,
        }
    }

    fn move_up(&mut self) {
        let mut index = self.selected_index;
        while index > 0 {
            index -= 1;
            if !self.menu.items[index].disabled {
                self.selected_index = index;
                break;
            }
        }
    }

    fn move_down(&mut self) {
        let mut index = self.selected_index + 1;
        while index < self.menu.items.len() {
            if !self.menu.items[index].disabled {
                self.selected_index = index;
                break;
            }
            index += 1;
        }
    }

    fn current_item(&self) -> Option<&TMenuItem> {
        self.menu.items.get(self.selected_index)
    }
}

impl TView for TDropDownMenu {
    fn draw(&self, buffer: &mut ScreenBuffer, offset: (u16, u16)) {
        let x0 = offset.0 + self.bounds.x;
        let y0 = offset.1 + self.bounds.y;

        for (i, item) in self.menu.items.iter().enumerate() {
            let y = y0 + i as u16;
            let is_selected = i == self.selected_index;

            let label = item.clean_label();
            let display = if is_selected {
                format!("> {} ", label)
            } else {
                format!("  {} ", label)
            };

            buffer.write_str(x0, y, &display);
        }
    }

    fn handle_event(&mut self, event: TEvent, queue: &TEventQueue) {
        if let TEvent::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Up => self.move_up(),
                KeyCode::Down => self.move_down(),
                KeyCode::Enter => {
                    if let Some(item) = self.current_item() {
                        if !item.disabled {
                            queue.put_event(TEvent::Command(item.command));
                        }
                    }
                }
                KeyCode::Esc => {
                    queue.put_event(TEvent::Command(0)); // 0 = close menu
                }
                KeyCode::Right => {
                    // Submenu logic can go here
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
