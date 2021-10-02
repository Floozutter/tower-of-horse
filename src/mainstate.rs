pub struct MainState;

impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
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
