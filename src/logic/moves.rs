use crate::{requests::Point, responses::Movement, snake::Snake};

use super::State;



pub fn can_move<const WIDTH: i32, const HEIGHT: i32>(p: Point, state: &State) -> bool {
    p.x >= 0 && p.x < WIDTH && p.y >= 0 && p.y < HEIGHT && !state.collides_with(p)
}

pub fn get_moves<const WIDTH: i32, const HEIGHT: i32>(state: &State) -> [bool; 4] {
    [
        can_move::<WIDTH, HEIGHT>(movement_to_move(state.snake.get_head(), Movement::Right), state),
        can_move::<WIDTH, HEIGHT>(movement_to_move(state.snake.get_head(), Movement::Left), state),
        can_move::<WIDTH, HEIGHT>(movement_to_move(state.snake.get_head(), Movement::Up), state),
        can_move::<WIDTH, HEIGHT>(movement_to_move(state.snake.get_head(), Movement::Down), state),
    ]
}

pub const fn movement_to_move(you: Point, movement: Movement) -> Point {
    match movement {
        Movement::Right => Point {
            x: you.x + 1,
            ..you
        },
        Movement::Left => Point {
            x: you.x - 1,
            ..you
        },
        Movement::Up => Point {
            y: you.y + 1,
            ..you
        },
        Movement::Down => Point {
            y: you.y - 1,
            ..you
        },
    }
}