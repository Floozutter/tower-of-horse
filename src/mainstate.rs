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
    falling: bool,
}

pub struct MainState {
    horsetower: Vec<TowerHorse>,
    drophorse: Option<DropHorse>,
    camera_height: f32,
}
impl MainState {
    const default_camera_height: f32 = 400.0;
    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        Ok(MainState{
            horsetower: Vec::new(),
            drophorse: None,
            camera_height: MainState::default_camera_height,
        })
    }
}
impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use ggez::graphics;
        graphics::clear(ctx, graphics::Color::WHITE);
        let groundbox = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, 800.0, 600.0].into(),
            graphics::Color::BLACK,
        )?;
        graphics::draw(
            ctx,
            &groundbox,
            (mint::Point2 { x: 0.0, y: self.camera_height },)
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self, _ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        _x: f32,
        _y: f32
    ) {
        use ggez::input::mouse::MouseButton;
        match button {
            MouseButton::Middle => {
                self.camera_height = MainState::default_camera_height;
            }
            _ => {}
        }
    }
    fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, _x: f32, y: f32) {
        self.camera_height += 50.0 * y;
        self.camera_height = self.camera_height.max(0.0);
    }
}
