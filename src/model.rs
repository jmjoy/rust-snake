use std::default::Default;
use std::collections::{LinkedList, HashSet};
use rand::Rand;

macro_rules! linked_list {
    [ $( $e:expr ),* ] => {{
        let mut l = ::std::collections::LinkedList::new();
        $(l.push_back($e);)*
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

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Point(pub usize, pub usize);

impl Rand for Point {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
        Point(Rand::rand(rng), Rand::rand(rng))
    }
}

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
        let mut game = Game {
            snake: linked_list![Point(4, y), Point(3, y), Point(2, y), Point(1, y)],
            .. Default::default()
        };
        game.set_food();
        game
    }

    pub fn set_food(&mut self) -> bool {
        // TODO check snake is full of screen, return fasle if true

        if self.snake.len() < CANVAS_WIDTH * CANVAS_HEIGHT / 2 {
            loop {
                let point = ::rand::random::<Point>();
                let point = match point {
                    Point(x, y) => Point(
                        x % (CANVAS_WIDTH - 1),
                        y % (CANVAS_HEIGHT - 1),
                    ),
                };
                if let None = self.snake.iter().find(|&&x| x == point) {
                    self.food = point;
                    return true;
                }
            }
        } else {
            unimplemented!();
        }
    }

    pub fn get_food(&self) -> Point {
        self.food
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
