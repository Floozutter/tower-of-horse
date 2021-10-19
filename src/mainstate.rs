#[derive(Clone)]
enum Direction {
    Left,
    Right
}

#[derive(Clone)]
enum HorseKind {
    Brown,
    Gray,
    Gold,
    Book,
    Dark,
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
            x,
        }
    }
}

struct DropHorse {
    kind: HorseKind,
    direction: Direction,
    t: f32,
    y: f32,
    v: f32,
}
impl DropHorse {
    pub fn gen(rng: &mut impl rand::Rng, y: f32) -> Self {
        use rand::seq::SliceRandom;
        let kind = match rng.gen::<f32>() {
            x if x < 0.0 => HorseKind::Dark,
            x if x < 0.3 => HorseKind::Brown,
            x if x < 0.6 => HorseKind::Gray,
            x if x < 0.9 => HorseKind::Gold,
            x if x < 1.0 => HorseKind::Book,
            _ => HorseKind::Dark,
        };
        Self {
            kind,
            direction: [Direction::Left, Direction::Right].choose(rng).expect("no direction").clone(),
            t: rng.gen_range(0.0..2.0*std::f32::consts::PI),
            y,
            v: 0.0,
        }
    }
    pub fn x(&self) -> f32 {
        let pre = self.t.sin();
        let (x1, y1) = (-1.0, 50.0);
        let (x2, y2) = (1.0, 700.0);
        let m = (y2 - y1)/(x2 - x1);
        let b = y1 - m*x1;
        m*pre + b
    }
}
impl From<HeldHorse> for DropHorse {
    fn from(heldhorse: HeldHorse) -> Self {
        todo!()
    }
}

struct HeldHorse {
    kind: HorseKind,
    direction: Direction,
    t: f32,
}
impl HeldHorse {
    pub fn gen(rng: &mut impl rand::Rng, y: f32) -> Self {
        todo!()
    }
    pub fn x(&self) -> f32 {
        todo!()
    }
}

pub struct MainState {
    rng: rand_pcg::Pcg64Mcg,
    horsetower: Vec<TowerHorse>,
    heldhorse: DropHorse,
    nexthorse: DropHorse,
    dropped: std::collections::VecDeque<DropHorse>,
    doomed: std::collections::VecDeque<DropHorse>,
    act: bool,
    camera_height: f32,
}
impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> ggez::GameResult<MainState> {
        let mut rng = rand_seeder::Seeder::from("uwu").make_rng::<rand_pcg::Pcg64Mcg>();
        let heldhorse = DropHorse::gen(&mut rng, 300.0);
        let nexthorse = DropHorse::gen(&mut rng, 300.0);
        let mut ret = MainState{
            rng,
            horsetower: [
                TowerHorse { kind: HorseKind::Brown, direction: Direction::Left, x: 100.0 },
                TowerHorse { kind: HorseKind::Gray, direction: Direction::Right, x: 120.0 },
                TowerHorse { kind: HorseKind::Gold, direction: Direction::Left, x: 140.0 },
            ].into(),
            heldhorse,
            nexthorse,
            dropped: std::collections::VecDeque::new(),
            doomed: std::collections::VecDeque::new(),
            act: false,
            camera_height: 0.0,
        };
        ret.reset_spawns();
        ret.reset_camera();
        Ok(ret)
    }
    
    fn tower_height(&self) -> f32 {
        50.0 * self.horsetower.len() as f32
    }
    fn drophorse_spawn_height(&self) -> f32 {
        self.tower_height() + 250.0
    }
    fn default_camera_height(&self) -> f32 {
        self.drophorse_spawn_height() + 75.0
    }

    fn reset_spawns(&mut self) {
        self.heldhorse.y = self.drophorse_spawn_height();
        self.nexthorse.y = self.drophorse_spawn_height();
    }
    fn reset_camera(&mut self) {
        self.camera_height = self.default_camera_height();
    }
}

impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        // update heldhorse
        self.heldhorse.t += 0.01;
        // drop heldhorse?
        if self.act {
            self.act = false;
            // lol
            let h = self.drophorse_spawn_height();
            self.dropped.push_back(
                std::mem::replace(
                    &mut self.heldhorse,
                    std::mem::replace(
                        &mut self.nexthorse,
                        DropHorse::gen(&mut self.rng, h)
                    )
                )
            );
        }
        // handle dropped
        for h in &mut self.dropped {
            h.v -= 0.25;
            h.y += h.v;
        }
        while let Some(lowest) = self.dropped.front() {
            if lowest.y < self.tower_height() {
                let intersecting = self.horsetower.last().map_or(false, |tophorse| {
                    let a = lowest.x();
                    let b = a + 50.0;
                    let c = tophorse.x;
                    let d = c + 50.0;
                    a <= d && c <= b
                });
                let mut lowest = self.dropped.pop_front().expect("no front");
                if intersecting {
                    self.horsetower.push(lowest.into());
                    self.reset_spawns();
                    self.reset_camera();
                } else {
                    lowest.v += 10.0;
                    self.doomed.push_back(lowest);
                }
            } else {
                break;
            }
        }
        // handle doomed
        for h in &mut self.doomed {
            h.v -= 0.25;
            h.y += h.v;
        }
        while let Some(lowest) = self.dropped.front() {
            if lowest.y < -100.0 {
                self.dropped.pop_front();
            } else {
                break;
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
        // draw nexthorse
        let nextmesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, 50.0, 50.0].into(),
            [1.0, 0.0, 1.0, 0.5].into(),
        )?;
        graphics::draw(
            ctx,
            &nextmesh,
            (mint::Point2 { x: self.nexthorse.x(), y: self.camera_height - (50.0 + self.nexthorse.y) },),
        )?;
        // draw heldhorse
        let heldmesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, 50.0, 50.0].into(),
            graphics::Color::RED
        )?;
        graphics::draw(
            ctx,
            &heldmesh,
            (mint::Point2 { x: self.heldhorse.x(), y: self.camera_height - (50.0 + self.heldhorse.y) },),
        )?;
        // draw dropped
        let droppedmesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, 50.0, 50.0].into(),
            graphics::Color::GREEN
        )?;
        for h in &self.dropped {
            graphics::draw(
                ctx,
                &droppedmesh,
                (mint::Point2 { x: h.x(), y: self.camera_height - (50.0 + h.y) },),
            )?;
        }
        // draw doomed
        let doomedmesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, 50.0, 50.0].into(),
            graphics::Color::YELLOW
        )?;
        for h in &self.doomed {
            graphics::draw(
                ctx,
                &doomedmesh,
                (mint::Point2 { x: h.x(), y: self.camera_height - (50.0 + h.y) },),
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
        if button == ggez::input::mouse::MouseButton::Middle {
            self.reset_camera();
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut ggez::Context, _x: f32, y: f32) {
        self.camera_height += 50.0 * y;
        self.camera_height = self.camera_height.max(0.0);
    }
}
