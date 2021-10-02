#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mainstate;

fn main() -> ggez::GameResult {
    let ctx_builder = ggez::ContextBuilder::new("tower-of-horse", "floozutter")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("tower of horse")
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(800.0, 600.0)
                .maximized(false)
                .resizable(false)
        );
    let (mut ctx, event_loop) = ctx_builder.build()?;
    let state = mainstate::MainState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, state)
}
