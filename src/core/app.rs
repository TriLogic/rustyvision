use crate::core::event::TEvent;
use crate::core::view::TView;
use crate::ui::screenbuffer::ScreenBuffer;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    execute,
};
use std::io::{stdout, Write};
use std::time::Duration;

pub struct TApplication {
    pub root: Box<dyn TView>,
    pub running: bool,
    pub width: u16,
    pub height: u16,
}

impl TApplication {
    pub fn new(root: Box<dyn TView>, width: u16, height: u16) -> Self {
        Self {
            root,
            running: true,
            width,
            height,
        }
    }

    pub fn run(&mut self) {
        let mut stdout = stdout();
        enable_raw_mode().unwrap();
        execute!(stdout, EnterAlternateScreen, Clear(ClearType::All)).unwrap();

        let mut buffer = ScreenBuffer::new(self.width, self.height);

        while self.running {
            // Draw
            buffer.clear();
            self.root.draw(&mut buffer, (0, 0));
            print!("{}", buffer.flush_to_string());
            stdout.flush().unwrap();

            // Handle events
            if event::poll(Duration::from_millis(200)).unwrap() {
                if let Ok(ev) = event::read() {
                    let tevent = match ev {
                        Event::Key(key) => {
                            if key.code == KeyCode::Esc || key.code == KeyCode::F(10) {
                                self.running = false;
                                continue;
                            }
                            TEvent::Key(key)
                        }
                        Event::Mouse(mouse) => TEvent::Mouse(mouse),
                        _ => TEvent::None,
                    };

                    self.root.handle_event(tevent);
                }
            }
        }

        // Exit cleanly
        execute!(stdout, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
