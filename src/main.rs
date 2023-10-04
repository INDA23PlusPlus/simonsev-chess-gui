use ggez::*;
use ggez::{
    conf,
    event::*,
    glam::*,
    graphics::*,
    Context, GameResult,
    input::mouse::*,
};

mod gui;
use gui::*;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Bruh", "Bruh Moment").window_mode(
        conf::WindowMode::default().dimensions(1800.0, 1800.0)
            .fullscreen_type(conf::FullscreenType::Windowed)
            .resizable(false),
    );
    let (mut ctx, event_loop) = cb.add_resource_path("./assets").build()?;
    let game_state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, game_state.unwrap());
}

