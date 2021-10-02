enum HorseKind {
    Brown,
    Gray,
    Gold,
    Book,
}

struct Horse {
    kind: HorseKind,
    x: f32,
    y: f32,
}

pub struct MainState {
    horses: Vec<Horse>,
}

impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        Ok(MainState{
            horses: Vec::new(),
        })
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
