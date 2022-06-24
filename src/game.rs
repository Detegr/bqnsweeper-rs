static SRC: &'static str = include_str!("bqnsweeper.bqn");

use cbqn::BQNValue;

pub struct Game {
    size: (i32, i32),
    chord_fn: BQNValue,
    flag_fn: BQNValue,
    guess_fn: BQNValue,
    render_fn: BQNValue,
    result_fn: BQNValue,
    showall_fn: BQNValue,
}

#[derive(Debug)]
pub enum GameResult {
    Win,
    Loss,
    InProgress,
}

impl Game {
    pub fn new(mines: i32, size: (i32, i32)) -> Game {
        fn nsget(func: &str) -> BQNValue {
            cbqn::eval(&format!(r#"{{•BQN"{{𝕩."∾𝕩∾"}}"}} "{}""#, func))
        }
        let new_game = cbqn::eval(SRC);
        let game = new_game.call2(&mines.into(), &[size.1, size.0].as_slice().into());
        let chord_fn = nsget("Chord").call1(&game);
        let flag_fn = nsget("Flag").call1(&game);
        let guess_fn = nsget("Guess").call1(&game);
        let render_fn = nsget("Render").call1(&game);
        let result_fn = nsget("Result").call1(&game);
        let showall_fn = nsget("ShowAll").call1(&game);
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
        self.chord_fn.call1(&[y, x].as_slice().into());
    }

    pub fn flag(&mut self, x: i32, y: i32) {
        self.flag_fn.call1(&[y, x].as_slice().into());
    }

    pub fn guess(&mut self, x: i32, y: i32) {
        self.guess_fn.call1(&[y, x].as_slice().into());
    }

    pub fn render(&self) {
        let rendered = self
            .render_fn
            .call1(&BQNValue::null())
            .into_char_vec()
            .unwrap();
        for j in 0..self.size.1 {
            for i in 0..self.size.0 {
                print!("{}", rendered[(j * self.size.0 + i) as usize]);
            }
            println!("");
        }
    }

    pub fn result(&self) -> GameResult {
        match self.result_fn.call1(&BQNValue::null()).into_f64() {
            0.0 => GameResult::InProgress,
            x => {
                if x < 0.0 {
                    GameResult::Loss
                } else {
                    GameResult::Win
                }
            }
        }
    }

    pub fn show_all(&mut self) {
        self.showall_fn.call1(&BQNValue::null());
    }
}
