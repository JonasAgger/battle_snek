use crate::{requests::Point, responses::Movement, snake::Snake};



pub fn can_move<const WIDTH: i32, const HEIGHT: i32>(p: Point, snake: &Snake) -> bool {
    p.x >= 0 && p.x < WIDTH && p.y >= 0 && p.y < HEIGHT && !snake.collides_with(p)
}

pub fn get_moves<const WIDTH: i32, const HEIGHT: i32>(snake: &Snake) -> [bool; 4] {
    [
        can_move::<WIDTH, HEIGHT>(movement_to_move(snake.get_head(), Movement::Right), snake),
        can_move::<WIDTH, HEIGHT>(movement_to_move(snake.get_head(), Movement::Left), snake),
        can_move::<WIDTH, HEIGHT>(movement_to_move(snake.get_head(), Movement::Up), snake),
        can_move::<WIDTH, HEIGHT>(movement_to_move(snake.get_head(), Movement::Down), snake),
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