mod minimax;
mod moves;

use crate::logic::minimax::minimax;
use crate::logic::moves::{get_moves, movement_to_move};

use std::time::Instant;

use tracing::{debug, error, info};

use crate::{
    requests::{Point, Turn},
    responses::{Move, Movement},
    snake::Snake,
};

const WIDTH: i32 = 11;
const HEIGHT: i32 = 11;
const DEPTH: u8 = 7;

pub(crate) struct State {
    pub(crate) snake: Snake,
    pub(crate) food: Vec<Point>,
    pub(crate) other_snakes: Vec<Snake>,
}

impl State {
    pub fn distance_to_food(&self) -> isize {
        let head = self.snake.get_head();
        self.food
            .iter()
            .map(|p| head.manhattan_distance(p) as isize)
            .min()
            .unwrap_or(0)
    }

    pub fn uneat(&mut self, food: Option<Point>) {
        if let Some(food) = food {
            self.food.push(food)
        }
    }

    pub fn try_eat(&mut self, head: Point) -> Option<Point> {
        for i in 0..self.food.len() {
            if self.food[i] == head {
                return Some(self.food.remove(i));
            }
        }

        None
    }

    pub fn collides_with(&self, p: Point) -> bool {
        if self.snake.collides_with(p) {
            return true;
        }

        self.other_snakes.iter().any(|s| s.collides_with(p))
    }
}

pub fn get_move(turn: Turn) -> Move {
    let budget = Instant::now();

    let mut state = State {
        snake: turn.you.body.into(),
        food: turn.board.food,
        other_snakes: turn
            .board
            .snakes
            .into_iter()
            .filter(|s| s.id != turn.you.id)
            .map(|s| s.body.into())
            .collect(),
    };
    //      x+1  x-1  y+1  y-1
    // x+1  .... .... .... ....
    // x-1  .... .... .... ....
    // y+1  .yx. .xy. ..x. ..y.
    // y-1  .... .... ..y. ..x.

    let possibilities = get_moves::<WIDTH, HEIGHT>(&state.snake, &state);

    let mut max_score = isize::MIN;
    let mut max = None;

    for movement in possibilities
        .iter()
        .enumerate()
        .filter(|(_, p)| **p)
        .map(|(i, _)| i.into())
    {
        debug!(?state.snake, ?movement);
        let head = movement_to_move(state.snake.get_head(), movement);
        let old_head = state.snake.push_head(head);

        let score = minimax::<DEPTH, WIDTH, HEIGHT>(&mut state);
        info!(?score, ?movement);
        if max.is_none() || score > max_score {
            max_score = score;
            max = Some(movement);
        }

        state.snake.pop_head(old_head);
        debug!(?state.snake, ?movement);
    }

    match max {
        Some(movement) => Move::new(movement),
        None => {
            error!("Found no best move!");
            return Move::new(Movement::Up);
        }
    }
}
