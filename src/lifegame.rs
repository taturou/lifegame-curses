extern crate rand;

use std::fmt;
use self::rand::Rng;

pub struct LifeGame {
    generation: usize,
    world :Vec<bool>,
    width: isize,
    height: isize,
    callback: Box<FnMut(CallbackInfo)>,
}

pub struct CellInfo {
    pub x: isize,
    pub y: isize,
    pub live: bool
}

pub enum CallbackEvent {
    Set,
    Evolution
}

pub struct CallbackInfo {
    pub event: CallbackEvent,
    pub generation: usize,
    pub width: isize,
    pub height: isize,
    pub num_cells: usize,
    pub cell: Option<CellInfo>
}

impl LifeGame {
    pub fn new(width: isize, height: isize) -> LifeGame {
        let len = (width * height) as usize;
        let world = vec![false; len];

        LifeGame {
            generation: 0,
            world,
            width,
            height,
            callback: Box::new(|_| {}),
        }
    }

    fn xy2i(&self, x: isize, y: isize) -> usize {
        ((self.width * y) + x) as usize
    }

    pub fn get(&self, x: isize, y: isize) -> bool {
        let x = if x < 0 {
            self.width + x
        } else if x >= self.width {
            x - self.width
        } else {
            x
        };

        let y = if y < 0 {
            self.height + y
        } else if y >= self.height {
            y - self.height
        } else {
            y
        };

        let i = self.xy2i(x, y);
        self.world[i]
    }

    fn get_as_num(&self, x: isize, y: isize) -> usize {
        let live = self.get(x, y);
        match live {
            true => 1,
            false => 0
        }
    }

    fn set_without_callback(&mut self, x: isize, y: isize, live: bool) {
        let i = self.xy2i(x, y);
        self.world[i] = live;
    }

    pub fn set(&mut self, x: isize, y: isize, live: bool) {
        self.set_without_callback(x, y, live);
        self.on_set(x, y, live);
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }

    pub fn evolution(&mut self) -> &Self {
        fn cell_evolution(game: &LifeGame , x: isize, y: isize) -> bool {
            let mut count: usize = 0;
            for j in (y-1)..(y+2) {
                for i in (x-1)..(x+2) {
                    count += game.get_as_num(i, j);
                }
            }
            count -= game.get_as_num(x, y);

            if game.get(x, y) {
                match count {
                    2 | 3 => true,
                    0 | 1 => false,
                    _     => false
                }
            } else {
                match count {
                    3 => true,
                    _ => false
                }
            }
        }

        let mut new = LifeGame::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let live = cell_evolution(self, x, y);
                new.set_without_callback(x, y, live);
            }
        }
        self.world = new.world;
        let new_generation = self.generation() + 1;
        self.set_generation(new_generation);
        self
    }

    pub fn reset(&mut self) -> &Self {
        for live in &mut self.world {
            *live = false;
        }
        self.set_generation(0);
        self
    }

    pub fn reset_by_rand(&mut self) -> &Self {
        for y in 0..self.height {
            for x in 0..self.width {
                let live =
                    if rand::thread_rng().gen_range(0, 100) > 50 {
                        true
                    } else {
                        false
                    };
                self.set_without_callback(x, y, live);
            }
        }
        self.set_generation(0);
        self
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    fn set_generation(&mut self, generation: usize) {
        self.generation = generation;
        self.on_evolution();
    }

    pub fn set_callback<F>(mut self, callback: F) -> Self
        where F: FnMut(CallbackInfo) + 'static {
        self.callback = Box::new(callback);
        self
    }

    fn on_evolution(&mut self) {
        let num_cells = self.num_cells();
        (self.callback)(
            CallbackInfo {
                event: CallbackEvent::Evolution,
                generation: self.generation,
                width: self.width,
                height: self.height,
                num_cells: num_cells,
                cell: None
            });
    }

    fn on_set(&mut self, x: isize, y: isize, live: bool) {
        let num_cells = self.num_cells();
        (self.callback)(
            CallbackInfo {
                event: CallbackEvent::Set,
                generation: self.generation,
                width: self.width,
                height: self.height,
                num_cells: num_cells,
                cell: Some(CellInfo { x, y, live })
            });
    }

    pub fn num_cells(&self) -> usize {
        self.world.iter().fold(0, |n, &live| if live { n+1 } else { n })
    }
}

impl fmt::Display for LifeGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let summary = format!("({}, {})", self.width, self.height);

        let mut world = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let live = self.get(x, y);
                let cell = if live { "o " } else { "x " };
                world.push_str(cell);
            }
            world.push_str("\n");
        }

        write!(f, "{}\n{}", summary, world)
    }
}
