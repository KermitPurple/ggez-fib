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
    conf: GameConf
}

impl Game {
    fn new(_ctx: &mut Context, conf: GameConf) -> Self {
        Self {
            conf,
        }
    }

    fn draw_fib(&mut self, ctx: &mut Context) -> GameResult {
        let (mut prev, mut curr) = (0., 30.);
        let mut mb = graphics::MeshBuilder::new();
        let mut pos = Point::new(0., 0.);
        for i in 0..20 {
            println!("{}", curr);
            let square = Rect::new(pos.x, pos.y , curr, curr);
            let dpos = match i % 4 {
                0 => Point::new(curr, 0.),
                1 => Point::new(-prev, curr),
                2 => Point::new(-curr - prev, -prev),
                3 => Point::new(0., - curr - prev),
                _ => unreachable!(),
            };
            pos += dpos;
            mb.rectangle(
                DrawMode::fill(),
                square,
                Color::from_rgb(100, 200, 100),
                )?
                .rectangle(
                    DrawMode::stroke(5.),
                    square,
                    Color::from_rgb(100, 100, 100),
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
    let game = Game::new(&mut ctx, conf);
    graphics::set_window_position(&ctx, Point::new(20., 20.))?;
    event::run(ctx, event_loop, game)
}
