use tetra::graphics::{self, Color};
use tetra::{ContextBuilder, Result as TetraResult, State};

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut tetra::Context) -> TetraResult {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", 640, 480)
        .quit_on_escape(true)
        .build()?
        .run(|_| Ok(GameState {}))
}
