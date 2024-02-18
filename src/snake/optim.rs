use std::fmt::Debug;

use crate::
    requests::Point
;


pub struct Snake {
    is_first: bool,
    head_index: usize,
    body: Vec<Point>,
}

impl Debug for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Snake: {}, body: [", self.head_index)?;

        for i in self.head_index..self.body.len() {
            write!(f, "{},", self.body[i])?;
        }

        for i in 0..self.head_index {
            write!(f, "{},", self.body[i])?;
        }

        write!(f, "]")
    }
}

impl From<Vec<Point>> for Snake {
    fn from(value: Vec<Point>) -> Self {
        Snake {
            is_first: true,
            head_index: value.len() - 1,
            body: value,
        }
    }
}

impl Snake {
    pub fn push_head(&mut self, p: Point) -> Point {
        if !self.is_first {
            self.head_index = self
                .head_index
                .checked_sub(1)
                .unwrap_or(self.body.len() - 1);
        }
        self.is_first = false;
        std::mem::replace(&mut self.body[self.head_index], p)
    }

    pub fn pop_head(&mut self, p: Point) {
        if self.is_first {
            panic!("Cant pop before a push");
        }
        self.head_index = (self.head_index + 1) % self.body.len();
        self.body[self.head_index] = p;
    }

    pub fn get_head(&self) -> Point {
        if self.is_first {
            self.body[0]
        } else {
            self.body[self.head_index]
        }
        
    }

    pub fn collides_with(&self, p: Point) -> bool {
        let tail_index = self
            .head_index
            .checked_sub(1)
            .unwrap_or(self.body.len() - 1);

        for index in 0..self.body.len() {
            if index != tail_index && p == self.body[index] {
                return true;
            }
        }

        false
    }

    #[cfg(test)]
    pub fn set_unchecked(&mut self, index: usize, p: Point) {
        self.body[index] = p;
    }
    #[cfg(test)]
    pub fn get_body(&self) -> Vec<Point> {
        self.body.clone()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn snake_recursion() {
        let mut snake: Snake = vec![
            Point {x: 1, y:0},
            Point {x: 2, y:0},
            Point {x: 3, y:0},
        ].into();

        let h1 = snake.push_head(Point { x: 0, y: 1 });
        let h2 = snake.push_head(Point { x: 0, y: 2 });
        let h3 = snake.push_head(Point { x: 0, y: 3 });
        let h4 = snake.push_head(Point { x: 0, y: 4 });
        let h5 = snake.push_head(Point { x: 0, y: 5 });
    
        snake.pop_head(h5);
        snake.pop_head(h4);
        snake.pop_head(h3);
        snake.pop_head(h2);
        snake.pop_head(h1);

        body_eq(&snake, &vec![
            Point {x: 1, y:0},
            Point {x: 2, y:0},
            Point {x: 3, y:0},
        ]);
    }

    #[test]
    fn snake_works() {
        /*
        ..... ..... .....
        ..... ..... .....
        ...32 ...43 ....4
        ...41 ...12 ..123
        ..... ..... .....


        move left should be best
        */

        let _ = tracing_subscriber::fmt::try_init();

        let mut snake: Snake = vec![
            Point { x: 4, y: 1 },
            Point { x: 4, y: 2 },
            Point { x: 3, y: 2 },
            Point { x: 3, y: 1 },
        ]
        .into();

        snake.push_head(Point { x: 3, y: 1 });
        dbg!(&snake);
        body_eq(
            &snake,
            &vec![
                Point { x: 4, y: 1 },
                Point { x: 4, y: 2 },
                Point { x: 3, y: 2 },
                Point { x: 3, y: 1 },
            ],
        );

        snake.push_head(Point { x: 2, y: 1 });
        dbg!(&snake);

        body_eq(
            &snake,
            &vec![
                Point { x: 4, y: 1 },
                Point { x: 4, y: 2 },
                Point { x: 3, y: 1 },
                Point { x: 2, y: 1 },
            ],
        );
    }

    fn body_eq(snake: &Snake, points: &[Point]) {
        let mut points: Vec<Point> = points.iter().cloned().collect();
        points.sort();

        let mut snake_points = snake.body.clone();
        snake_points.sort();

        assert_eq!(snake_points, points);
    }
}
