use crate::{draw_rectangle, snake::{Direction, Snake}};
use piston_window::{types::Color, Context, G2d, Key};
use rand::{thread_rng, Rng};

pub const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
pub const BORDER_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
pub const BLOCK_SIZE: f64 = 12.5;
pub const WINDOW_SIZE: f64 = 40.0;
pub const BORDER_WIDTH: f64 = 1.0;
pub const WAITING_TIME: f64 = 0.01;
pub const GAME_DELAY: f64 = 0.5;

#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}

pub type Food = Position;

pub struct Game {
    snake: Snake,
    food: Option<Food>,
    delay: f64,
}

impl Game {
    pub fn new() -> Self {
        Game {
            snake: Snake::new(),
            food: None,
            delay: GAME_DELAY,
        }
    }

    pub fn update(&mut self, dt: f64) -> Result<(), &str> {
        // create new food it none exists
        if self.food == None {
            // brute force the new position for the food (not really a good approach)
            // a better approach would be to create a map of possible positions and choose one of those
            let mut x: u64 = thread_rng().gen_range((BORDER_WIDTH as u64 * 2)..WINDOW_SIZE as u64 - 1);
            let mut y: u64 = thread_rng().gen_range((BORDER_WIDTH as u64 * 2)..WINDOW_SIZE as u64 - 1);

            while self.snake.overlaps_with(x, y) || self.food == Some(Position{x, y}) {
                x = thread_rng().gen_range(1..WINDOW_SIZE as u64);
                y = thread_rng().gen_range(1..WINDOW_SIZE as u64);
            }

            self.food = Some(Food{x, y});
        }

        // move the snake if the time has come
        self.delay += dt;
        if self.delay >= WAITING_TIME {
            self.delay = 0.0;

            if self.snake.move_snake(self.food)? {
                self.food = None
            }
        }

        Ok(())
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Up    => self.snake.change_dir(Direction::Up),
            Key::Down  => self.snake.change_dir(Direction::Down),
            Key::Left  => self.snake.change_dir(Direction::Left),
            Key::Right => self.snake.change_dir(Direction::Right),
            _ => (),
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        // draw the snake
        self.snake.draw(ctx, g);

        // draw the game border
        draw_rectangle!(BORDER_COLOR, 0.0, 0.0, WINDOW_SIZE * BLOCK_SIZE, BORDER_WIDTH, ctx, g);
        draw_rectangle!(BORDER_COLOR, 0.0, 0.0, BORDER_WIDTH, WINDOW_SIZE * BLOCK_SIZE, ctx, g);
        draw_rectangle!(BORDER_COLOR, WINDOW_SIZE - BORDER_WIDTH, 0.0, WINDOW_SIZE, BORDER_WIDTH, ctx, g);
        draw_rectangle!(BORDER_COLOR, 0.0, WINDOW_SIZE - BORDER_WIDTH, BORDER_WIDTH, WINDOW_SIZE, ctx, g);

        // draw the food
        if let Some(f) = self.food {
            draw_rectangle!(FOOD_COLOR, f.x as f64, f.y as f64, 1.0, 1.0, ctx, g);
        }
    }
}
