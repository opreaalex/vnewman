
extern crate rand;

use self::rand::Rng;

#[derive(Debug)]
#[derive(Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position{x: x, y: y}
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

pub fn new_random(start: u64, stop: u64) -> u64 {
    let mut thread_rng = rand::thread_rng();
    return thread_rng.gen_range(start, stop);
}
