extern crate cursive;

mod lifegame;

use lifegame::*;
use cursive::Cursive;
use cursive::views::{Dialog, LinearLayout, Panel};
use cursive::Printer;

struct Game {
}

impl Game {
    fn new() -> Game {
        Game {}
    }
}

impl cursive::view::View for Game {
    fn draw(&self, printer: &Printer) {
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
                s.pop_layer();
            }),
    );

    siv.run();
}
