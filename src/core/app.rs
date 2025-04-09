use crate::core::event::{TEvent, TEventQueue};
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
    pub queue: EventQueue,
}

impl TApplication {
    pub fn new(root: Box<dyn TView>, width: u16, height: u16) -> Self {
        Self {
            root,
            running: true,
            width,
            height,
            queue: EventQueue::new(),
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

            // Step 1: Handle internal events first
            if let Some(event) = self.queue.get_event() {
                self.handle_event(event);
                continue;
            }

            // Step 2: Poll terminal events
            if event::poll(Duration::from_millis(200)).unwrap() {
                if let Ok(ev) = event::read() {
                    let tevent = match ev {
                        Event::Key(key) => {
                            if key.code == KeyCode::Esc {
                                self.running = false;
                                continue;
                            }
                            TEvent::Key(key)
                        }
                        Event::Mouse(mouse) => TEvent::Mouse(mouse),
                        _ => TEvent::None,
                    };

                    self.handle_event(tevent);
                }
            }
        }

        // Exit cleanly
        execute!(stdout, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }

    fn handle_event(&mut self, event: TEvent) {
        match event {
            TEvent::Command(cmd) => {
                if cmd == 0 {
                    // Command 0 = close menu or cancel
                } else if cmd == 9999 {
                    self.running = false;
                } else {
                    // Your app handles command logic here
                    println!("Command triggered: {}", cmd);
                }
            }
            other => {
                self.root.handle_event(other, &self.queue);
            }
        }
    }

    pub fn put_event(&self, event: TEvent) {
        self.queue.put_event(event);
    }
}
