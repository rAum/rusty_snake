use rand::{thread_rng, Rng};
use crate::snake::{Direction, Snake, Block};

use crate::raylib::prelude::*;

pub struct Game {
    width : u32,
    height: u32,

    game_over : bool,
    waitng_time: f64,
    
    snake: Snake,
    food: Option<Block>,

    speed: f64,
}

impl Game {
    pub fn new(width : u32, height : u32) -> Game {
        Game {
            width: width,
            height: height,
            game_over: false,
            waitng_time: 0.,
            snake: Snake::new(3, 5),
            food: Some(Block::new(2, 2)),
            speed: 0.15,
        }
    }

    pub fn update_direction(&mut self, dir : Option<Direction>) {
        if self.game_over {
            return;
        }

        match dir {
            None => return,
            Some(v) if v == self.snake.head_direction().opposite() => return,
            Some(_v) => { self.update_snake(dir); }, 
        }
    }

    fn update_snake(&mut self, dir : Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waitng_time = 0.;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let w = self.width as i32;
        let h = self.height as i32;
        d.draw_rectangle(0, 0, w * 32, 32, Color::GRAY);
        d.draw_rectangle(0, (h - 1)*32,  w * 32, 32, Color::GRAY);
        d.draw_rectangle(0, 0, 32, h * 32, Color::GRAY);
        d.draw_rectangle((w - 1) * 32, 0, 32, h * 32, Color::GRAY);

        if self.game_over {
            return
        }

        if let Some(block) = self.food {
            d.draw_rectangle(block.x * 32, block.y * 32, 32, 32, Color::RED);
        }

        for block in self.snake.get_body() {
            d.draw_rectangle(block.x * 32, block.y * 32, 32, 32, Color::GREEN);
        }
        let scoring = format!("Score: {}", (self.snake.len() - 3) * 100);
        d.draw_text(&scoring, 0, 0, 24, Color::DARKPURPLE);
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waitng_time = 0.;
        self.game_over = false;
        self.speed = 0.2;
        self.add_food();
    }

    pub fn update(&mut self, delta_time : f64) {
        self.waitng_time += delta_time;
        if self.game_over {
            self.restart();
        }

        if self.food == None {
            self.add_food();
            self.speed -= 0.01
        }

        if self.waitng_time > self.speed {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (x, y) : (i32, i32) = self.snake.head_position();
        if Some(Block::new(x, y)) == self.food {
            self.food = None;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir : Option<Direction>) -> bool {
        let head = match dir {
            Some(v) => self.snake.next_head(v),
            None => self.snake.next_head(self.snake.head_direction()),
        };

        if self.snake.overlap_tail(Block::new(head.0, head.1)) {
            return false;
        }

        let w : i32 = (self.width - 1) as i32;
        let h : i32 = (self.height - 1) as i32;

        head.0 > 0 && head.1 > 0 && head.0 < w && head.1 < h
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        loop {
            let x = rng.gen_range(1, self.width - 1);
            let y = rng.gen_range(1, self.height - 1);
            let block = Block::new(x as i32, y as i32);
            if !self.snake.overlap_tail(block) {
                self.food = Some(block);
                break;
            }
        } 
    }
}

