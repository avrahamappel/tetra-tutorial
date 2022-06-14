use tetra::graphics::{self, Color, ImageData, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, Result as TetraResult, State};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const MOVEMENT_SPEED: f32 = 8.0;

struct GameState {
    link_texture: Texture,
    link_position: Vec2<f32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> TetraResult<Self> {
        let link_texture = Texture::from_image_data(
            ctx,
            &ImageData::new("assets/3DS_MetroidSamusReturns_char_01.png")?,
        )?;

        let link_position = Vec2::new(16.0, (WINDOW_HEIGHT - link_texture.height() as f32) / 2.0);

        Ok(GameState {
            link_texture,
            link_position,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> TetraResult {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.link_texture.draw(ctx, self.link_position);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> TetraResult {
        if input::is_key_down(ctx, Key::K) {
            self.link_position.y -= MOVEMENT_SPEED;
        }

        if input::is_key_down(ctx, Key::J) {
            self.link_position.y += MOVEMENT_SPEED
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Link", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
