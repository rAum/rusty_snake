extern crate rand;
extern crate raylib;

mod game;
mod snake;

use crate::game::Game;

use crate::raylib::prelude::*;

fn map_keyboard_to_action(r : &RaylibHandle) -> Option<snake::Direction> {
    use raylib::consts::KeyboardKey::*;
    use snake::Direction::*;
    if r.is_key_pressed(KEY_UP) {
        return Some(Up);
    }
    if r.is_key_pressed(KEY_DOWN) {
        return Some(Down);
    }
    if r.is_key_pressed(KEY_RIGHT) {
        return Some(Right);
    }
    if r.is_key_pressed(KEY_LEFT) {
        return Some(Left);
    }

    None
}

fn main() {
    let (mut r, thread) : (RaylibHandle, RaylibThread) = raylib::init().size(640, 480).title("RustySNakE").build();
    let mut game = Game::new(640 / 32, 480 / 32);
    while !r.window_should_close() {
        game.update_direction(map_keyboard_to_action(&r));// map_key_to_direction(r.get_key_pressed()));
        game.update(r.get_frame_time() as f64);
        let mut d  = r.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        game.draw(&mut d);
    }
    ()
}
