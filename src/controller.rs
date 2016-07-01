use std::time::{Duration, SystemTime};
use std::thread;
use std::default::Default;
use rustbox::{RustBox, Event};
use rustbox::keyboard::Key;

pub fn run() {
    let rustbox = RustBox::init(Default::default()).unwrap();
    let mut game = ::model::Game::new();

    let fps = Duration::from_millis(33);

    let mut speed_counter = 0;

    loop {
        let begin_time = SystemTime::now();

        let event = rustbox.peek_event(fps, false).unwrap();
        if let Event::KeyEvent(key) = event {
            match key {
                Key::Esc | Key::Char('q') => break,
                Key::Char('k') => game.change_direction(::model::Direction::Up),
                Key::Char('j') => game.change_direction(::model::Direction::Down),
                Key::Char('h') => game.change_direction(::model::Direction::Left),
                Key::Char('l') => game.change_direction(::model::Direction::Right),
                _ => {},
            }
        }

        if game.get_status() == ::model::GameStatus::Normal {
            if speed_counter == game.snake_speed {
                if game.snake_move() && !game.check_eat_himslef() {

                }
                speed_counter = 0;

            } else {
                speed_counter += 1;
            }
        }

        ::view::render(&rustbox, &game);

        let elapsed_time = begin_time.elapsed().unwrap();
        if elapsed_time < fps {
            thread::sleep(fps - elapsed_time);
        }
    }
}
