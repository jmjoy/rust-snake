use std::time::{Duration, SystemTime};
use std::thread;
use std::default::Default;
use rustbox::{RustBox, Event};
use rustbox::keyboard::Key;

pub fn run() {
    let rustbox = RustBox::init(Default::default()).unwrap();
    let game = ::model::Game::new();

    let fps = Duration::from_millis(33);

    ::view::render(&rustbox, &game);

    loop {
        let begin_time = SystemTime::now();

        let event = rustbox.peek_event(Duration::from_millis(200), false).unwrap();
        match event {
            Event::KeyEvent(key) => {
                match key {
                    Key::Esc => break,
                    _ => {},
                }
            },
            Event::NoEvent => {},
            _ => {},
        }

        let elapsed_time = begin_time.elapsed().unwrap();
        if elapsed_time < fps {
            thread::sleep(fps - elapsed_time);
        }
    }
}
