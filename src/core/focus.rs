use crate::core::view::TView;

pub struct FocusManager {
    pub index: usize,
}

impl FocusManager {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn focus_next(&mut self, children: &mut [Box<dyn TView>]) {
        let len = children.len();
        if len == 0 { return; }

        children[self.index].set_focus(false);
        for _ in 0..len {
            self.index = (self.index + 1) % len;
            if children[self.index].is_focusable() {
                break;
            }
        }
        children[self.index].set_focus(true);
    }

    pub fn focus_prev(&mut self, children: &mut [Box<dyn TView>]) {
        let len = children.len();
        if len == 0 { return; }

        children[self.index].set_focus(false);
        for _ in 0..len {
            if self.index == 0 {
                self.index = len - 1;
            } else {
                self.index -= 1;
            }
            if children[self.index].is_focusable() {
                break;
            }
        }
        children[self.index].set_focus(true);
    }

    pub fn set_focus(&mut self, children: &mut [Box<dyn TView>], index: usize) {
        if index < children.len() && children[index].is_focusable() {
            children[self.index].set_focus(false);
            self.index = index;
            children[self.index].set_focus(true);
        }
    }

    pub fn focus_next_raw(&mut self, max: usize) {
        if max == 0 { return; }
        self.index = (self.index + 1) % max;
    }

    pub fn focus_prev_raw(&mut self, max: usize) {
        if max == 0 { return; }
        if self.index == 0 {
            self.index = max - 1;
        } else {
            self.index -= 1;
        }
    }

    pub fn current_mut<'a>(&self, children: &'a mut [Box<dyn TView>]) -> Option<&'a mut Box<dyn TView>> {
        children.get_mut(self.index)
    }

    pub fn current<'a>(&self, children: &'a [Box<dyn TView>]) -> Option<&'a Box<dyn TView>> {
        children.get(self.index)
    }
}
