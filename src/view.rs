use rustbox::{RustBox, Color};

pub fn render(rustbox: RustBox, game: ::model::Game) {
    rustbox.print(1, 1, ::rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
}
