use tracing::debug;

use crate::logic::moves::{get_moves, movement_to_move};

use super::State;

pub fn minimax<const DEPTH: u8, const WIDTH: i32, const HEIGHT: i32>(state: &mut State) -> isize {
    minimax_impl2::<DEPTH, WIDTH, HEIGHT>(state, 0, 0, Minimax::Maximize)
}

fn minimax_impl<const DEPTH: u8, const WIDTH: i32, const HEIGHT: i32>(
    mut state: &mut State,
    depth: u8,
    mut current_score: isize,
) -> isize {
    let head = state.snake.get_head();

    debug!(?state.snake);

    let ate = state.try_eat(head);

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

    let possibilities = get_moves::<WIDTH, HEIGHT>(&state.snake, &state);
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

enum Minimax {
    Minimize,
    Maximize,
}

fn minimax_impl2<const DEPTH: u8, const WIDTH: i32, const HEIGHT: i32>(
    mut state: &mut State,
    depth: u8,
    mut current_score: isize,
    minimax: Minimax,
) -> isize {
    let head = state.snake.get_head();

    debug!(?state.snake);

    let ate = state.try_eat(state.snake.get_head());
    let other_ate: Vec<_> = state.other_snakes.iter().map(|s| s.get_head()).collect();

    let other_ate: Vec<_> = other_ate
        .into_iter()
        .filter_map(|p| state.try_eat(p))
        .collect();

    // Score for you eating
    if let Some(_) = ate {
        current_score += 100;
    }

    current_score -= (other_ate.len() * 10) as isize;

    // Exit condition
    if depth == DEPTH {
        other_ate.iter().for_each(|&f| state.uneat(Some(f)));

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

    match minimax {
        Minimax::Minimize => {
            let mut value = isize::MAX;
            let mut old_snakes = Vec::with_capacity(state.other_snakes.len());

            let move_sets: Vec<Vec<_>> = state
                .other_snakes
                .iter()
                .map(|s| {
                    get_moves::<WIDTH, HEIGHT>(s, &state)
                        .iter()
                        .enumerate()
                        .filter(|(_, p)| **p)
                        .map(|(i, _)| i.into())
                        .collect()
                })
                .collect();

            let moves = permutations(&move_sets);

            for move_set in moves {
                for s in 0..state.other_snakes.len() {
                    let movement = move_set[s];
                    let new_head = movement_to_move(state.other_snakes[s].get_head(), movement);
                    let old_head = state.other_snakes[s].push_head(new_head);
                    old_snakes.push(old_head);
                }

                let score = minimax_impl2::<DEPTH, WIDTH, HEIGHT>(
                    &mut state,
                    depth + 1,
                    current_score,
                    Minimax::Maximize,
                );

                value = value.min(score);

                for (i, &h) in old_snakes.iter().enumerate() {
                    state.other_snakes[i].pop_head(h);
                }
                old_snakes.clear();
            }

            state.uneat(ate);
            other_ate.iter().for_each(|&f| state.uneat(Some(f)));

            return value;
        }
        Minimax::Maximize => {
            let possibilities = get_moves::<WIDTH, HEIGHT>(&state.snake, &state);
            possibilities
                .iter()
                .enumerate()
                .filter(|(_, p)| **p)
                .map(|(i, _)| i.into())
                .map(|movement| {
                    let new_head = movement_to_move(head, movement);
                    let old_head = state.snake.push_head(new_head);

                    let score = minimax_impl2::<DEPTH, WIDTH, HEIGHT>(
                        &mut state,
                        depth + 1,
                        current_score,
                        Minimax::Minimize,
                    );

                    state.uneat(ate);
                    other_ate.iter().for_each(|&f| state.uneat(Some(f)));
                    state.snake.pop_head(old_head);

                    score
                })
                .max()
                .unwrap_or(isize::MIN)
        }
    }
}

fn permutations<T: Clone>(input: &[Vec<T>]) -> Vec<Vec<T>> {
    fn generate_permutations<T: Clone>(input: &[Vec<T>], index: usize) -> Vec<Vec<T>> {
        if index >= input.len() {
            return vec![vec![]];
        }

        let mut permutations = Vec::new();
        let next_permutations = generate_permutations(input, index + 1);

        for item in &input[index] {
            for permutation in &next_permutations {
                let mut new_permutation = vec![item.clone()];
                new_permutation.extend_from_slice(permutation);
                permutations.push(new_permutation);
            }
        }

        permutations
    }

    generate_permutations(input, 0)
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
