use rustbox::{RustBox, Color};

const MARGIN_TOP: usize = 0;

pub fn render(rustbox: &RustBox, game: &::model::Game) {
    draw_frame(rustbox);
    draw_food(rustbox, game);
    rustbox.present();
}

fn draw_food(rustbox: &RustBox, game: &::model::Game) {
    let ::model::Point(x, y) = game.get_food();
    draw_char_normal(rustbox, x+1, y+MARGIN_TOP+1, '❤');

    debug(rustbox, &*format!("{:?}", game.get_food()));
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
}

fn draw_char_normal(rustbox: &RustBox, x: usize, y: usize, c: char) {
    rustbox.print_char(x, y, ::rustbox::RB_NORMAL, Color::Default, Color::Default, c);
}

fn debug(rustbox: &RustBox, content: &str) {
    let y = ::model::CANVAS_HEIGHT + MARGIN_TOP;
    rustbox.print(0, y, ::rustbox::RB_NORMAL, Color::Default, Color::Default, content);
}
