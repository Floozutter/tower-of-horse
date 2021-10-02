enum Direction {
    Left,
    Right
}

enum HorseKind {
    Brown,
    Gray,
    Gold,
    Book,
}
struct TowerHorse {
    kind: HorseKind,
    direction: Direction,
    x: i64,
}
struct DropHorse {
    kind: HorseKind,
    direction: Direction,
    x: i64,
    y: i64,
}

pub struct MainState {
    horsetower: Vec<TowerHorse>,
    drophorse: Option<DropHorse>,
}
impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        Ok(MainState{
            horsetower: Vec::new(),
            drophorse: None,
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
