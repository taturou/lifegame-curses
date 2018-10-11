extern crate cursive;

mod lifegame;

use std::sync::Arc;
use std::sync::RwLock;
use lifegame::*;
use cursive::traits::*;
use cursive::Cursive;
use cursive::views::{Dialog, Panel};
use cursive::Printer;
use cursive::direction::Direction;
use cursive::vec::Vec2;
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};

struct Game {
    game: Arc<RwLock<LifeGame>>
}

impl Game {
    fn new(game: Arc<RwLock<LifeGame>>) -> Game {
        Game {
            game
        }
    }
}

impl cursive::view::View for Game {
    fn draw(&self, printer: &Printer) {
        let game = self.game.read().unwrap();

        let width = game.width();
        let height = game.height();

        for y in 0..height {
            for x in 0..width {
                let cell = game.get(x, y);
                let (text, color) = match cell {
                    true => ("o", Color::Dark(BaseColor::Red)),
                    false => (".", Color::Light(BaseColor::Black))
                };
                printer.with_color(
                    ColorStyle::new(color, Color::Dark(BaseColor::White)),
                    |printer| printer.print(((x * 2) as usize, y as usize), text));
            }
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let game = self.game.read().unwrap();

        Vec2::new(
            (game.width() * 2) as usize,
            game.height() as usize
        )
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Release(button)
            } => {
                match button {
                    MouseButton::Left => {
                        if (position.x >= offset.x) && (position.y >= offset.y) {
                            let mut game = self.game.write().unwrap();

                            let x = ((position.x - offset.x) / 2) as isize;
                            let y = (position.y - offset.y) as isize;

                            if (x < game.width()) && (y < game.height()) {
                                let cell = game.get(x, y);
                                game.set(x, y, if cell { false } else { true});
                            }
                        }
                        return EventResult::Consumed(None);
                    },
                    _ => ()
                }
            },
            _ => ()
        };
        EventResult::Ignored
    }
}

fn main() {
    let mut siv = Cursive::default();
    let screen_size = siv.screen_size();

    let game = Arc::new(
                RwLock::new(
                    LifeGame::new(
                        ((screen_size.x as isize) / 2) - 6,
                        (screen_size.y as isize) - 10)));

    siv.add_global_callback('c', |s| {
        s.call_on_id("game", |view: &mut Game| {
            view.game.write().unwrap().reset();
        });
    });

    siv.add_global_callback('r', |s| {
        s.call_on_id("game", |view: &mut Game| {
            view.game.write().unwrap().reset_by_rand();
        });
    });

    siv.add_global_callback('e', |s| {
        s.call_on_id("game", |view: &mut Game| {
            view.game.write().unwrap().evolution();
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
                    Game::new(game.clone())
                    .with_id("game"))
            ).button("Clear", |s| {
                s.call_on_id("game", |view: &mut Game| {
                    view.game.write().unwrap().reset();
                });
            }).button("Random", |s| {
                s.call_on_id("game", |view: &mut Game| {
                    view.game.write().unwrap().reset_by_rand();
                });
            }).button("Evolution", |s| {
                s.call_on_id("game", |view: &mut Game| {
                    view.game.write().unwrap().evolution();
                });
            }).button("Quit", |s| {
                s.quit()
            })
    );

    siv.set_fps(60);
    siv.run();
}
