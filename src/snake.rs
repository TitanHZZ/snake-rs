use std::collections::LinkedList;
use piston_window::{types::Color, Context, G2d};
use crate::draw_rectangle;
use crate::game::{Food, Position};

const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type BodyPart = Position;

pub struct Snake {
    direction: Direction,
    body: LinkedList<BodyPart>,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            direction: Direction::Right,
            body: LinkedList::from([
                BodyPart { x: 3, y: 1 },
                BodyPart { x: 2, y: 1 },
                BodyPart { x: 1, y: 1 },
            ]),
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.body.iter().for_each(|body_part|
            draw_rectangle!(SNAKE_COLOR, body_part.x as f64, body_part.y as f64, ctx, g)
        );
    }

    fn head_position(&self) -> Option<Position> {
        let head = self.body.front()?;
        Some(*head)
    }

    pub fn change_dir(&mut self, direction: Direction) {
        // cannot move into the snake itself
        if self.direction.opposite() != direction {
            self.direction = direction;
        }
    }

    pub fn overlaps_with(&self, x: u64, y: u64) -> bool {
        self.body.iter().any(|body_part| *body_part == Position{x, y})
    }

    pub fn move_snake(&mut self, food: Option<Food>) -> Result<bool, &str> {
        // check overlap with the snake itself
        let head_position = self.head_position().ok_or_else(|| "Could not get the snake's head position!")?;
        let overlap = self.body.iter().skip(1).any(|body_part| *body_part == head_position);
        if overlap {
            // the snake's head has hit it's own body (the player has lost)
            return Err("The snake's head has hit it's body!");
        }

        // determine the position for the new head
        let mut new_head_position = head_position;
        match self.direction {
            Direction::Up    => new_head_position.y -= 1,
            Direction::Down  => new_head_position.y += 1,
            Direction::Left  => new_head_position.x -= 1,
            Direction::Right => new_head_position.x += 1,
        }

        // actually move the snake (also check for food collision)
        self.body.push_front(BodyPart { x: new_head_position.x, y: new_head_position.y });
        let tail = self.body.pop_back().ok_or_else(|| "Could not update the snake to it's new position!")?;

        // add the tail back if the snake ate some food
        if let Some(f) = food {
            if head_position == f {
                self.body.push_back(tail);
                return Ok(true);
            }
        }

        Ok(false)
    }
}
