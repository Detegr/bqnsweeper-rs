mod game;
mod ui;

use game::Game;
use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
};
use ui::Ui;

const TILE_SIZE: f32 = 100.0;
const DIMENSIONS: (i32, i32) = (8, 8);
const MINES: i32 = 10;

fn main() {
    let dimensions = (8, 8);
    let mines = 10;
    let (ctx, event_loop) = ContextBuilder::new("bqnsweeper", "")
        .window_setup(WindowSetup::default().title("BQNsweeper"))
        .window_mode(WindowMode::default().dimensions(
            dimensions.0 as f32 * TILE_SIZE,
            dimensions.1 as f32 * TILE_SIZE,
        ))
        .build()
        .expect("could not create ggez context!");

    let game = Game::new(mines, dimensions);
    let ui = Ui::new(game);

    // Run!
    event::run(ctx, event_loop, ui);
}
