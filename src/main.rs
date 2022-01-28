mod coord;

use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    GameError,
    conf::{WindowSetup, WindowMode},
    event::{self, EventHandler},
    input::keyboard::{
        KeyCode,
        KeyMods,
    },
    graphics::{
        self,
        Rect,
        Color,
        DrawMode,
        DrawParam,
    },
};

type Point = coord::Coord;

const GOLDEN_RATIO: f32 = 1.618_034;
const INV_GOLDEN_RATIO: f32 = 1. / GOLDEN_RATIO;
const TWO_PI: f32 = 2. * std::f32::consts::PI;

enum State {
    Zooming,
    Rotating,
}

impl State {
    fn swap(&mut self) {
        *self = match self {
            State::Zooming => State::Rotating,
            State::Rotating => State::Zooming,
        }
    }
}

struct Game {
    conf: GameConf,
    style: Style,
    starting_size: f32,
    delta_theta: f32,
    state: State,
    paused: bool,
}

impl Game {
    fn new(_ctx: &mut Context, conf: GameConf, style: Style) -> Self {
        Self {
            conf,
            style,
            starting_size: 0.01,
            delta_theta: 0.,
            state: State::Zooming,
            paused: false,
        }
    }

    fn get_curve_points(&self, p: (Point, Point, Point), n_points: i32) -> Vec<Point> {
        let mut res = vec![];
        for i in 0..=n_points {
            let t = i as f32 / n_points as f32;
            res.push((p.0 * (1. - t) + p.1 * t) * (1. - t) + (p.1 * (1. - t) + p.2 * t) * t);
        }
        res
    }

    fn draw_zooming(&mut self, ctx: &mut Context) -> GameResult {
        let (mut prev, mut curr) = (0., self.starting_size);
        let mut mb = graphics::MeshBuilder::new();
        let mut pos = Point::new(0., 0.);
        for i in 0..30 {
            let square = Rect::new(pos.x, pos.y , curr, curr);
            let (dpos, points) = match i % 4 {
                0 => (
                    Point::new(curr, 0.),
                    (
                        Point::new(pos.x, pos.y + curr),
                        Point::new(pos.x, pos.y),
                        Point::new(pos.x + curr, pos.y),
                        )
                    ),
                1 => (
                    Point::new(-prev, curr),
                    (
                        Point::new(pos.x, pos.y),
                        Point::new(pos.x + curr, pos.y),
                        Point::new(pos.x + curr, pos.y + curr),
                        )
                    ),
                2 => (
                    Point::new(-curr - prev, -prev),
                    (
                        Point::new(pos.x + curr, pos.y),
                        Point::new(pos.x + curr, pos.y + curr),
                        Point::new(pos.x, pos.y + curr),
                        )
                    ),
                3 => (
                    Point::new(0., - curr - prev),
                    (
                        Point::new(pos.x + curr, pos.y + curr),
                        Point::new(pos.x, pos.y + curr),
                        Point::new(pos.x, pos.y),
                        )
                    ),
                _ => unreachable!(),
            };
            pos += dpos;
            mb.rectangle(
                DrawMode::fill(),
                square,
                self.style.get_color(i),
                )?
                .rectangle(
                    DrawMode::stroke(self.style.line_width),
                    square,
                    self.style.line_color,
                )?
                .line(
                    &self.get_curve_points(points, 100)[..],
                    self.style.line_width,
                    self.style.line_color,
                )?;
            prev += curr;
            std::mem::swap(&mut prev, &mut curr);
        }
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::new().dest(self.conf.window_center))
    }

    fn draw_rotating(&mut self, ctx: &mut Context) -> GameResult {
        let mut pos = Point::default();
        let mut rect = Rect::new(0., 0., 1000., 1000.);
        let mut theta = 0.;
        for i in 0..20{
            let param = DrawParam::default().dest(pos).rotation(theta);
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                rect,
                self.style.get_color(i),
            )?;
            graphics::draw(ctx, &mesh, param)?;
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(self.style.line_width),
                rect,
                self.style.line_color,
            )?;
            graphics::draw(ctx, &mesh, param)?;
            // add the rotation matrix multiplied by the size vector
            pos += (rect.w * theta.cos() - rect.h * theta.sin(), rect.w * theta.sin() + rect.h * theta.cos());
            rect.scale(INV_GOLDEN_RATIO, INV_GOLDEN_RATIO);
            theta += self.delta_theta;
        }
        Ok(())
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.paused {
            return Ok(());
        }
        match self.state {
            State::Zooming => {
                self.starting_size *= 1.05;
                if self.starting_size >= 3.322_971 {
                    self.starting_size = 0.01;
                }
            },
            State::Rotating => {
                self.delta_theta += 0.01;
                if self.delta_theta >= TWO_PI {
                    self.delta_theta = 0.;
                }
            },
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        match self.state {
            State::Zooming => self.draw_zooming(ctx)?,
            State::Rotating => self.draw_rotating(ctx)?,
        }
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Escape |
            KeyCode::Q => event::quit(ctx),
            KeyCode::Space |
            KeyCode::S => self.state.swap(),
            KeyCode::P => self.toggle_pause(),
            _ => (),
        }
    }
}

#[derive(Copy, Clone)]
struct GameConf {
    window_size: Point,
    window_center: Point,
}

impl GameConf {
    fn new(window_size: Point) -> Self {
        Self {
            window_size,
            window_center: window_size * 0.5,
        }
    }
}

#[derive(Clone)]
struct Style {
    line_width: f32,
    line_color: Color,
    main_colors: Vec<Color>,
}

impl Style {
    fn get_color(&self, index: usize) -> Color {
        self.main_colors[index % self.main_colors.len()]
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            line_width: 5.,
            line_color: Color::from_rgb_u32(0x555555),
            main_colors: vec![
                Color::from_rgb_u32(0xB00B69),
                Color::from_rgb_u32(0x042069),
                Color::from_rgb_u32(0xB4DA55),
                Color::from_rgb_u32(0x069420),
            ],
        }
    }
}

fn main() -> GameResult{
    let conf = GameConf::new(Point::new(2400., 1800.));
    let (mut ctx, event_loop) = ContextBuilder::new("Fibonacci Spiral", "KermitPurple")
        .window_setup(WindowSetup{
            title: String::from("Fibonacci Spiral"),
            ..Default::default()
        })
        .window_mode(WindowMode{
            width: conf.window_size.x,
            height: conf.window_size.y,
            ..Default::default()
        })
        .build()?;
    let game = Game::new(&mut ctx, conf, Default::default());
    graphics::set_window_position(&ctx, Point::new(20., 20.))?;
    event::run(ctx, event_loop, game)
}
