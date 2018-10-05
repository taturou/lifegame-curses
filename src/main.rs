extern crate rand;

use std::fmt;
use rand::Rng;

#[derive(Debug)]
struct LifeGame {
    world :Vec<bool>,
    width: usize,
    height: usize
}

impl LifeGame {
    fn new(width: usize, height: usize) -> LifeGame {
        let len = width * height;
        let world = vec![false; len];

        LifeGame {
            world,
            width,
            height
        }
    }

    fn xy2i(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let i = self.xy2i(x, y);
        self.world[i]
    }
    
    fn set(&mut self, x: usize, y: usize, cell: bool) {
        let i = self.xy2i(x, y);
        self.world[i] = cell;
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn reset_by_rand(&mut self) -> &Self {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, 
                         if rand::thread_rng().gen_range(0, 100) > 50 { true } else { false });
            }
        }
        self
    }
}

impl fmt::Display for LifeGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let summary = format!("({}, {})", self.width, self.height);

        let mut world = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get(x, y);
                let cell = if cell { "o " } else { "x " };
                world.push_str(cell);
            }
            world.push_str("\n");
        }

        write!(f, "{}\n{}", summary, world)
    }
}

fn main() {
    let mut game = LifeGame::new(5, 5);
    println!("{}", game);

    game.reset_by_rand();
    println!("{}", game);
}
