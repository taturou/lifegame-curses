extern crate cursive;

mod lifegame;

use lifegame::*;
use cursive::traits::*;
use cursive::Cursive;
use cursive::views::{Dialog, Panel};
use cursive::Printer;
use cursive::direction::Direction;
use cursive::vec::Vec2;

struct Game {
    game: LifeGame
}

impl Game {
    fn new(width: isize, height: isize) -> Game {
        let mut game = LifeGame::new(width, height);
        game.reset_by_rand();
        Game {
            game
        }
    }
}

impl cursive::view::View for Game {
    fn draw(&self, printer: &Printer) {
        let width = self.game.width();
        let height = self.game.height();

        for y in 0..height {
            for x in 0..width {
                let cell = self.game.get(x, y);
                /*
                let text = match cell {
                    true => "\u{25a0}",
                    false => "\u{25a1}"
                };
                */
                let text = match cell {
                    true => "o",
                    false => "."
                };
                printer.print(((x * 2) as usize, y as usize), text);
            }
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(
            (self.game.width() * 2) as usize,
            self.game.height() as usize
        )
    }
}

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('r', |s| {
        s.call_on_id("game", |view: &mut Game| {
            view.game.reset_by_rand();
        });
    });

    siv.add_global_callback('n', |s| {
        s.call_on_id("game", |view: &mut Game| {
            view.game.evolution();
        });
    });

    siv.add_global_callback('q', |s| {
        s.quit()
    });

    siv.add_layer(
        Dialog::new()
            .title("LifeGame")
            .content(
                Panel::new(
                    Game::new(80, 40).with_id("game"))
            ).button("Random", |s| {
                s.call_on_id("game", |view: &mut Game| {
                    view.game.reset_by_rand();
                });
            }).button("Evolution", |s| {
                s.call_on_id("game", |view: &mut Game| {
                    view.game.evolution();
                });
            }).button("Quit", |s| {
                s.quit()
            })
    );

    siv.set_fps(60);
    siv.run();
}
