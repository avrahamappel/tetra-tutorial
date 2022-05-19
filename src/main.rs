use tetra::{ContextBuilder, State};

struct GameState;

impl State for GameState {}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", 640, 480)
        .quit_on_escape(true)
        .build()?
        .run(|_| Ok(GameState {}))
}
