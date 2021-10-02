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
    x: f32,
}
struct DropHorse {
    kind: HorseKind,
    direction: Direction,
    x: f32,
    y: f32,
    v: f32,
    dropping: bool,
    doomed: bool,
}

pub struct MainState {
    horsetower: Vec<TowerHorse>,
    drophorse: Option<DropHorse>,
    t: f32,
    act: bool,
    camera_height: f32,
}
impl MainState {
    const DEFAULT_CAMERA_HEIGHT: f32 = 400.0;
    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        Ok(MainState{
            horsetower: [
                TowerHorse { kind: HorseKind::Brown, direction: Direction::Left, x: 100.0 },
                TowerHorse { kind: HorseKind::Gray, direction: Direction::Right, x: 120.0 },
                TowerHorse { kind: HorseKind::Gold, direction: Direction::Left, x: 140.0 },
            ].into(),
            drophorse: Some(DropHorse { kind: HorseKind::Book, direction : Direction::Right, x: 400.0, y: 300.0, v: 0.0, dropping: false, doomed: false }),
            t: 0.0,
            act: false,
            camera_height: MainState::DEFAULT_CAMERA_HEIGHT,
        })
    }
}
impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if let Some(drophorse) = &mut self.drophorse {
            if !drophorse.dropping && self.act {
                drophorse.dropping = true;
                self.act = false;
            }
            if drophorse.dropping {
                drophorse.y += drophorse.v;
                drophorse.v -= 0.25;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use ggez::graphics;
        // clear screen
        graphics::clear(ctx, graphics::Color::WHITE);
        // draw ground
        let groundbox = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, 800.0, 600.0].into(),
            graphics::Color::BLACK
        )?;
        graphics::draw(
            ctx,
            &groundbox,
            (mint::Point2 { x: 0.0, y: self.camera_height },)
        )?;
        // draw horsetower
        for (i, th) in self.horsetower.iter().enumerate() {
            let r = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                [0.0, 0.0, 50.0, 50.0].into(),
                graphics::Color::BLUE
            )?;
            graphics::draw(
                ctx,
                &r,
                (mint::Point2 { x: th.x, y: self.camera_height - 50.0*((i+1) as f32) },),
            )?;
        }
        // draw drophorse
        if let Some(d) = &self.drophorse {
            let dropmesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                [0.0, 0.0, 50.0, 50.0].into(),
                graphics::Color::RED
            )?;
            graphics::draw(
                ctx,
                &dropmesh,
                (mint::Point2 { x: d.x, y: self.camera_height - (50.0 + d.y)},),
            )?;
        }
        // show screen
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        _keymodss: ggez::event::KeyMods,
        _repeat: bool
    ) {
        match keycode {
            ggez::event::KeyCode::Escape => {
                ggez::event::quit(ctx);
            },
            ggez::event::KeyCode::Space => {
                self.act = true;
            },
            _ => {},
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        _x: f32,
        _y: f32
    ) {
        use ggez::input::mouse::MouseButton;
        match button {
            MouseButton::Middle => {
                self.camera_height = MainState::DEFAULT_CAMERA_HEIGHT;
            },
            _ => {},
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, _x: f32, y: f32) {
        self.camera_height += 50.0 * y;
        self.camera_height = self.camera_height.max(0.0);
    }
}
