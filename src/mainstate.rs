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
impl From<DropHorse> for TowerHorse {
    fn from(drophorse: DropHorse) -> Self {
        let x = drophorse.x();
        Self {
            kind: drophorse.kind,
            direction: drophorse.direction,
            x: x,
        }
    }
}

struct DropHorse {
    kind: HorseKind,
    direction: Direction,
    t: f32,
    y: f32,
    v: f32,
    dropping: bool,
    doomed: bool,
}
impl DropHorse {
    pub fn gen(_rng: &mut impl rand::Rng, y: f32) -> Self {
        Self {
            kind: HorseKind::Brown,
            direction: Direction::Left,
            t: 0.0,
            y: y,
            v: 0.0,
            dropping: false,
            doomed: false,
        }
    }
    pub fn x(&self) -> f32 {
        300.0
    }
}

pub struct MainState {
    rng: rand_pcg::Pcg64Mcg,
    horsetower: Vec<TowerHorse>,
    drophorse: Option<DropHorse>,
    t: f32,
    act: bool,
    camera_height: f32,
}
impl MainState {
    const DEFAULT_CAMERA_HEIGHT: f32 = 400.0;

    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let mut rng = rand_seeder::Seeder::from("uwu").make_rng::<rand_pcg::Pcg64Mcg>();
        let drophorse = DropHorse::gen(&mut rng, 300.0);
        Ok(MainState{
            rng: rng,
            horsetower: [
                TowerHorse { kind: HorseKind::Brown, direction: Direction::Left, x: 100.0 },
                TowerHorse { kind: HorseKind::Gray, direction: Direction::Right, x: 120.0 },
                TowerHorse { kind: HorseKind::Gold, direction: Direction::Left, x: 140.0 },
            ].into(),
            drophorse: Some(drophorse),
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
                let tower_height = 50.0 * self.horsetower.len() as f32;
                if !drophorse.doomed && drophorse.y < tower_height {
                    let intersecting = if let Some(tophorse) = self.horsetower.last() {
                        let a = drophorse.x();
                        let b = a + 50.0;
                        let c = tophorse.x;
                        let d = c + 50.0;
                        a <= d && c <= b
                    } else {
                        false
                    };
                    if intersecting {
                        
                    } else {
                        println!("doomed!");
                        drophorse.doomed = true;
                        drophorse.v += 10.0;
                    }
                }
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
                (mint::Point2 { x: d.x(), y: self.camera_height - (50.0 + d.y)},),
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
