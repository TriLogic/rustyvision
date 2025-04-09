use std::cell::RefCell;
use std::rc::Rc;
use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Debug, Clone)]
pub enum TEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Command(u16),
    None,
}

#[derive(Clone)]
pub struct TEventQueue {
    inner: Rc<RefCell<Vec<TEvent>>>,
}

impl TEventQueue {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn put_event(&self, event: TEvent) {
        self.inner.borrow_mut().push(event);
    }

    pub fn get_event(&self) -> Option<TEvent> {
        self.inner.borrow_mut().pop()
    }

    pub fn has_pending(&self) -> bool {
        !self.inner.borrow().is_empty()
    }
}