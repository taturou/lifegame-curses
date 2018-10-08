extern crate cursive;

mod lifegame;

use lifegame::*;
use cursive::Cursive;
use cursive::views::{Dialog, LinearLayout, Panel};
use cursive::Printer;
use cursive::direction::Direction;
use cursive::vec::Vec2;

struct Game {
    game: LifeGame
}

impl Game {
    fn new() -> Game {
        let mut game = LifeGame::new(20, 10);
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
                let text = match cell {
                    true => "\u{25a0}",
                    false => "\u{25a1}"
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
    /*
    let mut game = LifeGame::new(20, 10);
    println!("{}", game);

    game.reset_by_rand();
    println!("{}", game);

    game.evolution();
    println!("{}", game);
    */
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(
        Dialog::new()
            .title("LifeGame")
            .content(
                LinearLayout::horizontal()
                    .child(Panel::new(Game::new())),
            ).button("Quit game", |s| {
                s.quit();
            }),
    );

    siv.run();
}
