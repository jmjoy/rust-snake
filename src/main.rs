extern crate rustbox;
extern crate rand;

mod model;
mod controller;
mod view;

fn main() {
    ::controller::run();
}
