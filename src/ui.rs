use crate::game::{Game, GameState};
use crate::{DIMENSIONS, MINES, TILE_SIZE};
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

pub struct Ui {
    game: Game,
}

impl Ui {
    pub fn new(game: Game) -> Ui {
        Ui { game }
    }
}

impl EventHandler for Ui {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.game.result() != GameState::InProgress {
            self.game.show_all();
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _mods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => event::quit(ctx),
            KeyCode::R => {
                self.game = Game::new(MINES, DIMENSIONS);
            }
            _ => {}
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.game.result() == GameState::InProgress {
            let gridx = (x / TILE_SIZE).floor() as i32;
            let gridy = (y / TILE_SIZE).floor() as i32;
            if button == MouseButton::Left {
                self.game.guess(gridx.into(), gridy.into());
            } else if button == MouseButton::Middle {
                self.game.chord(gridx.into(), gridy.into());
            } else if button == MouseButton::Right {
                self.game.flag(gridx.into(), gridy.into());
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let tiles = self.game.render();
        graphics::clear(ctx, Color::WHITE);

        for j in 0..self.game.size.1 {
            for i in 0..self.game.size.0 {
                let char_value = tiles[(j * self.game.size.0 + i) as usize];
                let color = match char_value {
                    '.' => graphics::Color::from_rgb(128, 128, 128),
                    'x' => graphics::Color::from_rgb(255, 0, 0),
                    'f' => graphics::Color::from_rgb(255, 255, 0),
                    '/' => graphics::Color::from_rgb(150, 150, 150),
                    '1' => graphics::Color::from_rgb(0, 0, 255),
                    '2' => graphics::Color::from_rgb(0, 255, 0),
                    '3' => graphics::Color::from_rgb(0, 150, 150),
                    '4' => graphics::Color::from_rgb(150, 50, 255),
                    '5' => graphics::Color::from_rgb(255, 200, 150),
                    '6' => graphics::Color::from_rgb(80, 0, 250),
                    '7' => graphics::Color::from_rgb(0, 80, 150),
                    '8' => graphics::Color::from_rgb(255, 0, 0),
                    _ => graphics::Color::from_rgb(90, 90, 90),
                };
                let mut char_drawable = graphics::Text::new(graphics::TextFragment {
                    text: char_value.into(),
                    color: Some(Color::BLACK),
                    scale: Some(graphics::PxScale::from(80.0)),
                    ..Default::default()
                });
                char_drawable.set_bounds([TILE_SIZE, TILE_SIZE], graphics::Align::Center);
                let fill = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::Fill(graphics::FillOptions::default()),
                    graphics::Rect::new(
                        i as f32 * TILE_SIZE,
                        j as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                    color,
                )
                .unwrap();
                let border = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::Stroke(graphics::StrokeOptions::default()),
                    graphics::Rect::new(
                        i as f32 * TILE_SIZE,
                        j as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                    graphics::Color::BLACK,
                )
                .unwrap();
                graphics::draw(ctx, &fill, graphics::DrawParam::default()).unwrap();
                graphics::draw(ctx, &border, graphics::DrawParam::default()).unwrap();
                graphics::draw(
                    ctx,
                    &char_drawable,
                    graphics::DrawParam::default()
                        .dest([i as f32 * TILE_SIZE, j as f32 * TILE_SIZE]),
                )
                .unwrap();
            }
        }
        graphics::present(ctx)
    }
}
