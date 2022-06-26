static SRC: &'static str = include_str!("bqnsweeper.bqn");

use cbqn::BQNValue;

pub struct Game {
    pub size: (i32, i32),
    chord_fn: BQNValue,
    flag_fn: BQNValue,
    guess_fn: BQNValue,
    render_fn: BQNValue,
    result_fn: BQNValue,
    showall_fn: BQNValue,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Win,
    Loss,
    InProgress,
}

impl Game {
    pub fn new(mines: i32, size: (i32, i32)) -> Game {
        let new_game = cbqn::eval(SRC);
        let game = new_game.call2(&mines.into(), &[size.1, size.0].as_slice().into());
        let chord_fn = game.get_field("chord").unwrap();
        let flag_fn = game.get_field("flag").unwrap();
        let guess_fn = game.get_field("guess").unwrap();
        let render_fn = game.get_field("render").unwrap();
        let result_fn = game.get_field("result").unwrap();
        let showall_fn = game.get_field("showall").unwrap();
        Game {
            size,
            chord_fn,
            flag_fn,
            guess_fn,
            render_fn,
            result_fn,
            showall_fn,
        }
    }

    pub fn chord(&mut self, x: i32, y: i32) {
        self.chord_fn.call1(&[y, x].into());
    }

    pub fn flag(&mut self, x: i32, y: i32) {
        self.flag_fn.call1(&[y, x].into());
    }

    pub fn guess(&mut self, x: i32, y: i32) {
        self.guess_fn.call1(&[y, x].into());
    }

    pub fn render(&self) -> Vec<char> {
        self.render_fn
            .call1(&BQNValue::null())
            .into_char_vec()
            .unwrap()
    }

    pub fn result(&self) -> GameState {
        let res = self.result_fn.call1(&BQNValue::null()).into_f64();
        if res < 0.0 {
            GameState::Loss
        } else if res > 0.0 {
            GameState::Win
        } else {
            GameState::InProgress
        }
    }

    pub fn show_all(&mut self) {
        self.showall_fn.call1(&BQNValue::null());
    }
}
