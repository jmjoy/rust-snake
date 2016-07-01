use std::fmt::Debug;
use rustbox::{RustBox, Color};

const MARGIN_TOP: usize = 0;

pub fn render(rustbox: &RustBox, game: &::model::Game) {
    draw_frame(rustbox);
    match game.status {
        ::model::GameStatus::Normal => {
            draw_snake(rustbox, game);
            draw_food(rustbox, game);
            draw_score(rustbox, game);
        },

        ::model::GameStatus::Succ => {
            unimplemented!();
        },

        ::model::GameStatus::Fail => {
            let (x, y) = (::model::CANVAS_WIDTH / 3, ::model::CANVAS_HEIGHT / 2);
            rustbox.print(x, y, ::rustbox::RB_NORMAL, Color::Red, Color::Default, "You lose!");
        },
    };
    rustbox.present();
}

fn draw_snake(rustbox: &RustBox, game: &::model::Game) {
    let snake = game.get_snake();
    for &::model::Point(x, y) in snake.iter() {
        draw_char_normal(rustbox, x+1, y+MARGIN_TOP+1, 'o'); // '⊙'
    }
}

fn draw_food(rustbox: &RustBox, game: &::model::Game) {
    let ::model::Point(x, y) = game.get_food();
    draw_char_normal(rustbox, x+1, y+MARGIN_TOP+1, '$'); // '❤'

    // debug(rustbox, &*format!("{:?}", game.get_food()));
}

fn draw_score(rustbox: &RustBox, game: &::model::Game) {
    let (x, y) = (::model::CANVAS_WIDTH + 3, 5);
    rustbox.print(x, y, ::rustbox::RB_NORMAL, Color::Default, Color::Default, &*format!("Score: {}", game.get_score()));
}

fn draw_frame(rustbox: &RustBox) {
    let x = ::model::CANVAS_WIDTH - 1;
    let y = ::model::CANVAS_HEIGHT - 1;
    draw_char_normal(rustbox, 0, 0, '┌');
    draw_char_normal(rustbox, x, 0, '┐');
    draw_char_normal(rustbox, 0, y, '└');
    draw_char_normal(rustbox, x, y, '┘');

    let c = '─';
    for i in 1 .. x {
        draw_char_normal(rustbox, i, 0, c);
        draw_char_normal(rustbox, i, y, c);
    }

    let c = '│';
    for i in 1 .. y {
        draw_char_normal(rustbox, 0, i, c);
        draw_char_normal(rustbox, x, i, c);
    }

    for i in 1 .. x {
        for j in 1 .. y {
            draw_char_normal(rustbox, i, j, '.');
        }
    }

    let x = ::model::CANVAS_WIDTH + 3;
    rustbox.print(x, 3, ::rustbox::RB_BOLD, Color::Default, Color::Default, "Snake");
}

fn draw_char_normal(rustbox: &RustBox, x: usize, y: usize, c: char) {
    rustbox.print_char(x, y, ::rustbox::RB_NORMAL, Color::Default, Color::Default, c);
}

#[allow(dead_code)]
pub fn debug<T: Debug>(rustbox: &RustBox, content: T) {
    let y = ::model::CANVAS_HEIGHT + MARGIN_TOP;
    let content = &*format!("{:?}  ", content);
    rustbox.print(0, y, ::rustbox::RB_NORMAL, Color::Default, Color::Default, content);
}
