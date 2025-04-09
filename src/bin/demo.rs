use rustyvision::core::app::TApplication;
use rustyvision::core::rect::TRect;
use rustyvision::core::focus::FocusManager;
use rustyvision::widgets::{
    button::TButton,
    checkboxes::TCheckBoxes,
    dialog::TDialog,
    inputline::TInputLine,
    label::TLabel,
    listbox::TListBox,
    radiobuttons::TRadioButtons,
    statusline::TStatusLine,
    footerbar::TFooterBar,
};

use rustyvision::core::view::TView;
use std::rc::Rc;
use std::cell::RefCell;

// A simple root container that holds all top-level views
struct RootContainer {
    children: Vec<Box<dyn TView>>,
    bounds: TRect,
    focus: FocusManager,
}

impl RootContainer {
    pub fn new(bounds: TRect) -> Self {
        Self { children: Vec::new(), bounds, focus: FocusManager::new() }
    }

    pub fn add_child(&mut self, mut child: Box<dyn TView>) {
        if self.children.is_empty() && child.is_focusable() {
            child.set_focus(true);
        }
        self.children.push(child);
    }
}

impl TView for RootContainer {
    fn draw(&self, buffer: &mut rustyvision::ui::screenbuffer::ScreenBuffer, offset: (u16, u16)) {
        for child in &self.children {
            child.draw(buffer, offset);
        }
    }

    fn handle_event(&mut self, event: rustyvision::core::event::TEvent) {
        use crossterm::event::KeyCode;

        if let rustyvision::core::event::TEvent::Key(key) = &event {
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
        // nothing
    }

}

fn main() {
    let screen_width = 80;
    let screen_height = 25;

    // Menu bar
    /*
    let menubar = TMenuBar::new(
        TRect { x: 0, y: 0, width: screen_width, height: 1 },
        vec![("File", 1), ("Edit", 2), ("Help", 3)],
    );
    */

    // Dialog
    let mut dialog = TDialog::new("Demo Dialog", TRect { x: 5, y: 2, width: 70, height: 20 });
    dialog.add_child(Box::new(TLabel::new("Name:", TRect { x: 2, y: 1, width: 20, height: 1 })));

    let mut input = TInputLine::new(TRect { x: 2, y: 2, width: 30, height: 1 });
    input.set_value("Rusty User");
    dialog.add_child(Box::new(input));

    dialog.add_child(Box::new(TCheckBoxes::new(
        TRect { x: 2, y: 4, width: 25, height: 3 },
        vec!["Option A", "Option B", "Option C"],
    )));

    dialog.add_child(Box::new(TRadioButtons::new(
        TRect { x: 2, y: 8, width: 25, height: 3 },
        vec!["One", "Two", "Three"],
    )));

    dialog.add_child(Box::new(TListBox::new(
        TRect { x: 40, y: 1, width: 25, height: 5 },
        vec!["Alpha", "Beta", "Gamma", "Delta", "Epsilon"],
    )));

    let mut button = TButton::new("OK", TRect { x: 25, y: 14, width: 10, height: 1 });
    button.set_callback(|| println!("Button pressed!"));
    dialog.add_child(Box::new(button));

    // Footer bar with interactive content
    let mut footer = TFooterBar::new(TRect { x: 0, y: screen_height - 1, width: screen_width, height: 1 });
    let quit_btn = TButton::new("Quit", TRect { x: 65, y: 0, width: 10, height: 1 });
    footer.add_child(Box::new(quit_btn));

    // Root container with all top-level views
    let mut root = RootContainer::new(TRect { x: 0, y: 0, width: screen_width, height: screen_height });
    root.add_child(Box::new(menubar));
    root.add_child(Box::new(dialog));
    root.add_child(Box::new(footer));

    let mut app = TApplication::new(Box::new(root), screen_width, screen_height);
    app.run();
}
