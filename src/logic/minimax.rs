use tracing::debug;

use crate::{
    logic::moves::{get_moves, movement_to_move},
    requests::Point,
};

use super::State;

pub fn minimax<const DEPTH: u8, const WIDTH: i32, const HEIGHT: i32>(state: &mut State) -> isize {
    minimax_impl::<DEPTH, WIDTH, HEIGHT>(state, 0, 0)
}

fn minimax_impl<const DEPTH: u8, const WIDTH: i32, const HEIGHT: i32>(
    mut state: &mut State,
    depth: u8,
    mut current_score: isize,
) -> isize {
    let head = state.snake.get_head();

    debug!(?state.snake);

    let ate = state.try_eat();

    // Score for eating
    if let Some(_) = ate {
        current_score += 100;
    }

    // Exit condition
    if depth == DEPTH {
        return match ate {
            Some(p) => {
                state.uneat(Some(p));
                current_score
            }
            None => {
                let distance_to_food = state.distance_to_food();
                current_score + distance_to_food
            }
        };
    }

    let possibilities = get_moves::<WIDTH, HEIGHT>(&state);
    possibilities
        .iter()
        .enumerate()
        .filter(|(_, p)| **p)
        .map(|(i, _)| i.into())
        .map(|movement| {
            let new_head = movement_to_move(head, movement);
            let old_head = state.snake.push_head(new_head);

            let score = minimax_impl::<DEPTH, WIDTH, HEIGHT>(&mut state, depth + 1, current_score);

            state.uneat(ate);
            state.snake.pop_head(old_head);

            score
        })
        .max()
        .unwrap_or(isize::MIN)
}

#[cfg(test)]
mod tests {
    use tracing::info_span;

    use super::*;

    #[test]
    fn minimax_move_left() {
        /*
        .....
        ...x.
        ...x.
        f..x.
        .....

        move left should be best
        */

        let mut snake: Snake = vec![
            Point { x: 3, y: 1 },
            Point { x: 3, y: 2 },
            Point { x: 3, y: 3 },
        ]
        .into();
        let food = vec![Point { x: 0, y: 1 }];

        let h = snake.push_head(Point { x: 2, y: 1 });
        let min_left = minimax::<0, 5, 5>(&mut snake, &food, 0);

        snake.pop_head(h);
        snake.push_head(Point { x: 3, y: 0 });
        let min_down = minimax::<0, 5, 5>(&mut snake, &food, 0);

        snake.pop_head(h);
        snake.push_head(Point { x: 4, y: 1 });
        let min_right = minimax::<0, 5, 5>(&mut snake, &food, 0);

        assert_eq!(min_left, 2);
        assert_eq!(min_down, 4);
        assert_eq!(min_right, 4);
    }

    #[test]
    fn minimax_best_move_no_lookahead() {
        /*
        .....
        .....
        .....
        ...21
        ..f3.

        move left should be best
        */

        let mut snake: Snake = vec![
            Point { x: 4, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 3, y: 0 },
        ]
        .into();
        let food = vec![Point { x: 2, y: 0 }];

        let h = snake.push_head(Point { x: 4, y: 0 });
        let min_down = minimax::<0, 5, 5>(&mut snake, &food, 0);

        snake.pop_head(h);
        snake.push_head(Point { x: 4, y: 2 });
        let min_up = minimax::<0, 5, 5>(&mut snake, &food, 0);

        assert!(min_down < min_up);
    }

    #[test]
    fn minimax_best_move_1_lookahead() {
        /*
        .....
        .....
        .....
        ...21
        ..f3.

        .....
        .....
        ....1
        ...32
        ..f.1

        .....
        ....1
        ...12
        ....3
        ..f12

        move left should be best
        */
        let mut snake: Snake = vec![
            Point { x: 4, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 3, y: 0 },
        ]
        .into();
        let food = vec![Point { x: 2, y: 0 }];

        let h = snake.push_head(Point { x: 4, y: 0 });
        let min_down = info_span!("down").in_scope(|| minimax::<1, 5, 5>(&mut snake, &food, 1));

        snake.pop_head(h);
        snake.push_head(Point { x: 4, y: 2 });
        let min_up = info_span!("up").in_scope(|| minimax::<1, 5, 5>(&mut snake, &food, 1));

        assert!(min_down < min_up);
        assert_eq!(min_down, 1);
        assert_eq!(min_up, 3);
    }

    #[test]
    fn minimax_best_move_2_lookahead() {
        /*
        DOWN
        ..... ..... ..... .....
        ..... ..... ..... .....
        ...21 ...32 ...43 ....4
        ..f3. ..f41 ..f12 ..123
        ...4. ..... ..... .....

        UP
        ..... ..... ..... .....
        ..... ....1 ...12 ..123
        ...21 ...32 ...43 ...14
        ..f3. ..f4. ..f.. ..f..
        ...4. ..... ..... .....

        move down should be best
        */

        let _ = tracing_subscriber::fmt::try_init();

        let mut snake: Snake = vec![
            Point { x: 4, y: 2 },
            Point { x: 3, y: 2 },
            Point { x: 3, y: 1 },
            Point { x: 3, y: 0 },
        ]
        .into();
        let food = vec![Point { x: 2, y: 1 }];

        let h = snake.push_head(Point { x: 4, y: 1 });
        let min_down = info_span!("down").in_scope(|| minimax::<2, 5, 5>(&mut snake, &food, 2));

        snake.pop_head(h);
        snake.push_head(Point { x: 4, y: 3 });
        let min_up = info_span!("up").in_scope(|| minimax::<2, 5, 5>(&mut snake, &food, 2));

        assert!(min_down < min_up);
        assert_eq!(min_down, 0);
        assert_eq!(min_up, 2);
    }
}
