use piston_window::UpdateEvent;
use piston_window::{clear, color::BLACK, PistonWindow, WindowSettings, Button, PressEvent};
use game::{BLOCK_SIZE, WINDOW_SIZE};
use game::Game;
mod draw;
mod game;
mod snake;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Snake Game",
        [(WINDOW_SIZE * BLOCK_SIZE) as f64, (WINDOW_SIZE * BLOCK_SIZE) as f64],
    )
    .resizable(false)
    .exit_on_esc(true)
    .build()
    .expect("Could not create game window!");

    let mut game = Game::new();
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&e, |ctx, g, _gfx| {
            clear(BLACK, g);
            game.draw(&ctx, g);
        });

        e.update(|arg|
            game.update(arg.dt).expect("Could not move the snake")
        );
    }
}
