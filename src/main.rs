mod game;
use game::Game;

fn main() {
    let mut game = Game::new(8, (8, 8));
    game.render();
    game.guess(1, 0);
    println!("");
    game.show_all();
    game.render();
    println!("{:?}", game.result());
}
