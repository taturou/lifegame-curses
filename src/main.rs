extern crate cursive;

mod lifegame;

use std::sync::Arc;
use std::sync::RwLock;
use lifegame::*;
use cursive::Cursive;
use cursive::views::{Dialog, Panel, LinearLayout, TextView, TextContent};
use cursive::Printer;
use cursive::direction::Direction;
use cursive::vec::Vec2;
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::align::Align;

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

    let gen = TextContent::new("Generation: 0");
    let mut gen_evo = gen.clone();

    let game = Arc::new(
                RwLock::new(
                    LifeGame::new(((screen_size.x as isize) / 2) - 6,
                                  (screen_size.y as isize) - 11)
                        .on_evolution(move |generation| {
                            let str = format!("Generation: {}", generation);
                            gen_evo.set_content(str);
                        })
                    ));
    let game_key_c = game.clone();
    let game_key_r = game.clone();
    let game_key_e = game.clone();
    let game_btn_c = game.clone();
    let game_btn_r = game.clone();
    let game_btn_e = game.clone();

    siv.add_global_callback('c', move |_| {
        game_key_c.write().unwrap().reset();
    });

    siv.add_global_callback('r', move |_| {
        game_key_r.write().unwrap().reset_by_rand();
    });

    siv.add_global_callback('e', move |_| {
        game_key_e.write().unwrap().evolution();
    });

    siv.add_global_callback('q', |s| {
        s.quit()
    });

    siv.add_layer(
        Dialog::new()
            .title("LifeGame")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new_with_content(gen.clone()).align(Align::top_right()))
                    .child(Panel::new(Game::new(game.clone())))
            ).button("Clear", move |_| {
                game_btn_c.write().unwrap().reset();
            }).button("Random", move |_| {
                game_btn_r.write().unwrap().reset_by_rand();
            }).button("Evolution", move |_| {
                game_btn_e.write().unwrap().evolution();
            }).button("Quit", |s| {
                s.quit()
            })
    );

    siv.set_fps(60);
    siv.run();
}
