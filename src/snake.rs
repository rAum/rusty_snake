use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Block {
        Block{x: x, y: y}
    }
}

pub struct Snake {
    direction : Direction,
    body : VecDeque<Block>,
    tail : Option<Block>,
}

impl Snake {
    pub fn new(x : i32, y : i32) -> Snake {
        let mut body : VecDeque<Block> = VecDeque::with_capacity(32);
        body.push_back(Block { x : x, y : y});
        body.push_back(Block { x : x + 1, y : y});
        body.push_back(Block { x : x + 2, y : y});
        Snake {
            direction: Direction::Down,
            body : body,
            tail : None,
        }
    }

    pub fn get_body(&self) -> VecDeque<Block> {
        self.body.clone()
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn next_head(&self, dir : Direction) -> (i32, i32) {
        let (x, y) = self.head_position();
        use Direction::*;
        match dir {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        }
    }

    pub fn move_forward(&mut self, dir : Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            _ => (),
        }

        let (last_x, last_y) = self.head_position();
        use Direction::*;
        let new_block = match self.direction {
            Up => Block { x: last_x, y: last_y - 1},
            Down => Block { x : last_x, y : last_y + 1},
            Left => Block { x : last_x - 1, y: last_y},
            Right => Block { x : last_x + 1, y: last_y},
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn restore_tail(&mut self) {
        let tail = self.tail.clone().unwrap();
        self.body.push_back(tail);
    }

    pub fn overlap_tail(&self, block : Block) -> bool {
        for i in 0..self.body.len() {
            if block == self.body[i] {
                return true;
            }
        }
        return false
    }
}
