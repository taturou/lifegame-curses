extern crate cursive;
extern crate lifegame;

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
        let mut game = self.game.write().unwrap();

        for (x, y, cell) in game.iter_as_u8(None) {
            let (text, color) = match cell {
                0 => (".", Color::Light(BaseColor::Black)),
                1 => ("1", Color::Dark(BaseColor::Yellow)),
                2 => ("2", Color::Dark(BaseColor::Cyan)),
                3 => ("3", Color::Dark(BaseColor::Green)),
                4 => ("4", Color::Dark(BaseColor::Blue)),
                5 => ("5", Color::Dark(BaseColor::Magenta)),
                6 => ("6", Color::Dark(BaseColor::Red)),
                7 => ("7", Color::Dark(BaseColor::Red)),
                8 => ("8", Color::Dark(BaseColor::Red)),
                _ => ("x", Color::Dark(BaseColor::Red))
            };
            printer.with_color(
                ColorStyle::new(color, Color::Dark(BaseColor::White)),
                |printer| printer.print(((x * 2) as usize, y as usize), text));
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

                            let x = (position.x - offset.x) / 2;
                            let y = position.y - offset.y;

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

    let info = TextContent::new("Gen:0, Cells:0");
    let mut info_on = info.clone();

    let game = Arc::new(
                RwLock::new(
                    LifeGame::new((screen_size.x / 2) - 6,
                                  screen_size.y - 11)
                        .set_callback(move |info| {
                            let mut str = format!("Gen:{}, Cells:{}",
                                        info.generation,
                                        info.num_cells);
                            match info {
                                CallbackInfo {
                                    event: CallbackEvent::Set,
                                    generation: _,
                                    width: _,
                                    height: _,
                                    num_cells: _,
                                    cell: Some(cell),
                                } => {
                                    let live = if cell.live { "Live" } else { "Dead" };
                                    str = format!("{}, {}:({},{})",
                                            str,
                                            live,
                                            cell.x,
                                            cell.y);
                                },
                                _ => ()
                            }
                            info_on.set_content(str);
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
                    .child(TextView::new_with_content(info.clone()).align(Align::top_right()))
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
