mod lifegame;

use lifegame::*;

fn main() {
    let mut game = LifeGame::new(20, 10);
    println!("{}", game);

    game.reset_by_rand();
    println!("{}", game);

    game.evolution();
    println!("{}", game);
}
