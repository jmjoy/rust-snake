use std::default::Default;
use rustbox::{RustBox, Event};
use rustbox::keyboard::Key;

fn run() {
    let rustbox = RustBox::init(Default::default()).unwrap();

    let game = ::model::Game::new();
    ::view::render(rustbox, game);

    rustbox.present();

    loop {
        let event = rustbox.poll_event(false).unwrap();
        if let Event::KeyEvent(key) = event {
            match key {
                Key::Char('q') => break,
                _ => {},
            }
        }
    }
}
