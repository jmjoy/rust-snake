use std::default::Default;
use std::collections::{LinkedList};
use std::ops::Neg;
use rand::Rand;

macro_rules! linked_list {
    [ $( $e:expr ),* ] => {{
        let mut l = ::std::collections::LinkedList::new();
        $(l.push_back($e);)*
        l
    }};
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameStatus {
    Normal,
    Succ,
    Fail,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::Normal
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

pub type SnakeBody = LinkedList<Point>;

#[derive(Default)]
pub struct Game {
    food: Point,
    snake: SnakeBody,
    direction: Direction,
    pub snake_speed: i32,
    pub status: GameStatus,
    score: u32,
}

impl Game {
    pub fn new() -> Self {
        let y = CANVAS_HEIGHT / 2;
        let mut game = Game {
            snake: linked_list![Point(4, y), Point(3, y), Point(2, y), Point(1, y)],
            snake_speed: 3,
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
                        x % (CANVAS_WIDTH - 2),
                        y % (CANVAS_HEIGHT - 2),
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

    pub fn get_snake(&self) -> SnakeBody {
        self.snake.clone()
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if direction != -self.direction {
            self.direction = direction;
        }
    }

    pub fn snake_move(&mut self) -> bool {
        let header = match self.snake.front().unwrap() {
            &Point(x, y) => match self.direction {
                Direction::Left => {
                    if x == 0 {
                        return false;
                    }
                    Point(x - 1, y)
                }

                Direction::Right => {
                    if x == CANVAS_WIDTH - 3 {
                        return false;
                    }
                    Point(x + 1, y)
                }

                Direction::Up => {
                    if y == 0 {
                        return false;
                    }
                    Point(x, y - 1)
                }

                Direction::Down => {
                    if y == CANVAS_HEIGHT - 3 {
                        return false;
                    }
                    Point(x, y + 1)
                }
            }
        };

        self.snake.push_front(header);

        true
    }

    pub fn check_eat_himslef(&self) -> bool {
        let mut iter = self.snake.iter();
        let header = iter.next().unwrap();
        while let Some(body) = iter.next() {
            if header == body {
                return true;
            }
        }

        false
    }

    pub fn eat_food(&mut self) -> bool {
        let result = {
            let header = self.snake.front().unwrap();
            *header != self.food
        };
        if result {
            self.snake.pop_back();
            return true;
        }
        if !self.set_food() {
            return false;
        }
        self.score += 1;
        true
    }

}
