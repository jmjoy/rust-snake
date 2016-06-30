use std::default::Default;
use std::collections::LinkedList;

macro_rules! linked_list {
    [] => {{
        let l = ::std::collections::LinkedList::new();
        l
    }};
}

#[derive(Debug, Copy, Clone)]
enum GameElement {
    Empty,
    Snake{isHeader: bool},
    Food,
}

impl Default for GameElement {
    fn default() -> Self {
        GameElement::Empty
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Point(usize, usize);

pub const CANVAS_WIDTH: usize = 40;
pub const CANVAS_HEIGHT: usize = 20;

type Canvas = [[GameElement; CANVAS_WIDTH]; CANVAS_HEIGHT];

#[derive(Default)]
pub struct Game {
    food: Point,
    snake: LinkedList<Point>,
    direction: Direction,
}

impl Game {
    pub fn new() -> Self {
        let y = CANVAS_HEIGHT / 2;
        let snake = [Point(4, y), Point(3, y), Point(2, y), Point(1, y)];
        Game {
            // snake: snake.iter().collect::<LinkedList<Point>>(),
            .. Default::default()
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
