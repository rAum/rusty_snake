extern crate rand;
extern crate piston;

use piston_window::*;
use piston_window::types::Color;

mod game;
mod draw;
mod snake;

use crate::draw::to_coord_u32;
use crate::game::Game;

const BACK_COLOR : Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (w, h) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new("Sssnake", [to_coord_u32(w), to_coord_u32(h)])
        .exit_on_esc(true)
        .build().unwrap();

    let mut game = Game::new(w as u32, h as u32);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _d| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
