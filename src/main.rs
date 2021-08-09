mod coord;

use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    GameError,
    conf::{WindowSetup, WindowMode},
    event::{self, EventHandler},
    graphics::{
        self,
        mint,
        Font,
        Text,
        Rect,
        Color,
        DrawMode,
        DrawParam,
    },
};

type Point = coord::Coord;

struct Game {
    conf: GameConf,
    style: Style,
}

impl Game {
    fn new(_ctx: &mut Context, conf: GameConf, style: Style) -> Self {
        Self {
            conf,
            style,
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

    fn draw_fib(&mut self, ctx: &mut Context) -> GameResult {
        let (mut prev, mut curr) = (0., 1.);
        let mut mb = graphics::MeshBuilder::new();
        let mut pos = Point::new(0., 0.);
        for i in 0..20 {
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
                    DrawMode::stroke(5.),
                    square,
                    self.style.line_color,
                )?
                .line(
                    &self.get_curve_points(points, 100)[..],
                    5.,
                    self.style.line_color,
                )?;

            prev += curr;
            std::mem::swap(&mut prev, &mut curr);
        }
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::new().dest(self.conf.window_center))
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.draw_fib(ctx)?;
        graphics::present(ctx)
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
            line_color: Color::from_rgb_u32(0x555555),
            main_colors: vec![
                Color::from_rgb_u32(0xB00B69),
                Color::from_rgb_u32(0x042069),
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
