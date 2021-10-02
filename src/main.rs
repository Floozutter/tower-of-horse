#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct MainState;

impl MainState {
    fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        Ok(MainState)
    }
}
impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
}

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
    let state = MainState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, state)
}
