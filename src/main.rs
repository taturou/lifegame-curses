extern crate rand;

use std::fmt;
use rand::Rng;

#[derive(Debug)]
struct LifeGame {
    world :Vec<bool>,
    width: isize,
    height: isize
}

impl LifeGame {
    fn new(width: usize, height: usize) -> LifeGame {
        let len = width * height;
        let world = vec![false; len];

        LifeGame {
            world,
            width: width as isize,
            height: height as isize
        }
    }

    fn xy2i(&self, x: isize, y: isize) -> usize {
        ((self.width * y) + x) as usize
    }

    fn get(&self, x: isize, y: isize) -> bool {
        let x = if x < 0 {
            self.width - x
        } else if x >= self.width {
            x - self.width
        } else {
            x
        };

        let y = if y < 0 {
            self.height - y
        } else if y >= self.height {
            y - self.height
        } else {
            y
        };

        let i = self.xy2i(x, y);
        self.world[i]
    }
    
    fn set(&mut self, x: isize, y: isize, cell: bool) {
        let i = self.xy2i(x, y);
        self.world[i] = cell;
    }
    
    pub fn width(&self) -> isize {
        self.width
    }
    
    pub fn height(&self) -> isize {
        self.height
    }

    pub fn reset_by_rand(&mut self) -> &Self {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = 
                    if rand::thread_rng().gen_range(0, 100) > 50 {
                        true
                    } else {
                        false
                    };
                self.set(x, y, cell);
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
