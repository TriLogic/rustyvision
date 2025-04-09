use crate::core::rect::TRect;
use crate::core::event::{TEvent, TEventQueue};
use crate::ui::screenbuffer::ScreenBuffer;

pub trait TView {
    fn draw(&self, buf: &mut ScreenBuffer, offset: (u16, u16));
    fn handle_event(&mut self, event: TEvent, queue: &TEventQueue);
    fn get_bounds(&self) -> TRect;
    fn set_bounds(&mut self, bounds: TRect);
    fn set_focus(&mut self, _focused: bool);
    fn is_focusable(&self) -> bool { false }
}
