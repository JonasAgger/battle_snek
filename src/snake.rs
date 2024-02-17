use tracing::{error, info};

use crate::{requests::{Point, Turn}, responses::{Move, Movement}};



pub fn calculate_move(turn: Turn) -> Move {
    let turn_nr = turn.turn;
    let you = turn.you.head;
    let self_prev = turn.you.body[1];
    let body = &turn.you.body[1..];
    let best_food = turn.board.food.iter().min_by(|x, y| x.manhattan_distance(&you).cmp(&y.manhattan_distance(&you
    )));

    //      x+1  x-1  y+1  y-1
    // x+1  .... .... .... ....
    // x-1  .... .... .... ....
    // y+1  .yx. .xy. ..x. ..y.
    // y-1  .... .... ..y. ..x.

    let possibilities = [
        you.x >= self_prev.x && you.x + 1 < turn.board.width && can_move_for_own_body(Point { x: you.x+1, ..you }, body),
        you.x <= self_prev.x && you.x - 1 >= 0 && can_move_for_own_body(Point { x: you.x-1, ..you }, body),
        you.y >= self_prev.y && you.y + 1 < turn.board.height && can_move_for_own_body(Point { y: you.y+1, ..you }, body),
        you.y <= self_prev.y && you.y - 1 >= 0 && can_move_for_own_body(Point { y: you.y-1, ..you }, body),
    ];

    info!(?turn_nr, ?you, ?self_prev, ?best_food, ?possibilities);

    let movement = if let Some(best_food) = best_food {
        if best_food.x < you.x {
            Movement::Left
        }
        else if best_food.y < you.y {
            Movement::Down
        }
        else if best_food.x == you.x {
            Movement::Up
        }
        else {
            Movement::Right
        }
    } else {
        // lets try to circle.
        if you.x < 0 {
            Movement::Left
        }
        else if you.y < 0 {
            Movement::Up
        }
        else {
            Movement::Down
        }
    };

    let movement = if possibilities[movement as usize] {
        movement
    } else {
        let next = possibilities.iter().enumerate().filter(|(_, p)| **p).map(|(i, _)| i).next();
        match next {
            Some(i) => i.into(),
            None => {
                error!("no possibilities????");
                Movement::Up
            },
        }
    };

    Move::new(movement)
}

fn can_move_for_own_body(p: Point, body: &[Point]) -> bool {
    !body.iter().any(|bp| p.eq(bp))
}