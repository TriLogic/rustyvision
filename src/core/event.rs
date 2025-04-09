use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Debug, Clone)]
pub enum TEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Command(u16),
    None,
}
