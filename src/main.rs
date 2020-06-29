use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra;
use std::env;
use ggez::timer;
use std::path;

mod blocs;

type Point2 = nalgebra::Point2<f32>;
type Vector2 = nalgebra::Vector2<f32>;

struct Assets {
    bloc_orange: graphics::Image,
    bloc_bleu: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let bloc_orange = graphics::Image::new(ctx, "/orange.png")?;
        let bloc_bleu = graphics::Image::new(ctx, "/bleu.png")?;

        Ok(Assets {
            bloc_orange,
            bloc_bleu,
        })
    }

    fn bloc_image(&mut self, bloc: &blocs::Bloc) -> &mut graphics::Image {
        match bloc.tag {
            blocs::BlocType::Orange => &mut self.bloc_orange,
            blocs::BlocType::Bleu => &mut self.bloc_bleu,
        }
    }
}



/// Translates the world coordinate system, which
/// has Y pointing up and the origin at the center,
/// to the screen coordinate system, which has Y
/// pointing downward and the origin at the top-left,
fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Point2) -> Point2 {
    let x = point.x + screen_width / 2.0;
    let y = screen_height - (point.y + screen_height / 2.0);
    Point2::new(x, y)
}

fn draw_bloc(
	assets: &mut Assets,
	ctx: &mut Context,
	bloc: &blocs::Bloc,
	world_coords: (f32, f32),
	) -> GameResult {
	
		let (screen_w, screen_h) = world_coords;
		let pos = world_to_screen_coords(screen_w, screen_h, bloc.pos);
		let image = assets.bloc_image(bloc);
		let draw_params = graphics::DrawParam::new()
			.dest(pos);
		graphics::draw(ctx, image, draw_params)
}

fn create_board() -> Vec<Vec<blocs::Bloc> > {
	let mut bleu: blocs::Bloc = blocs::Bloc::new_bleu();
	bleu.pos = Point2::new(40.0, 40.0);
	
	let orange: blocs::Bloc = blocs::Bloc::new_orange();
	
	vec![vec![bleu,orange]]
	
}

fn main() -> GameResult {
    // Make a Context.

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::renderer_info(ctx)?);
    let game = &mut MyGame::new(ctx).unwrap();
    event::run(ctx, events_loop, game)
}

struct MyGame {
    // Your state here...
    meshes: Vec<graphics::Mesh>,
    assets: Assets,
	board: Vec<Vec<blocs::Bloc>>,
	screen_width: f32,
    screen_height: f32,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let meshes = vec![build_mesh(ctx)?];
        let assets = Assets::new(ctx)?;
		let board = create_board();
		let (screen_width, screen_height) = graphics::drawable_size(ctx);
		
        let s = MyGame {
            // ...
            meshes,
            assets,
			board,
			screen_width,
			screen_height,
        };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.circle(
        DrawMode::fill(),
        Point2::new(600.0, 380.0),
        40.0,
        1.0,
        Color::new(1.0, 0.0, 1.0, 1.0),
    );

    mb.build(ctx)
}
impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
		
		let assets = &mut self.assets;
        let coords = (self.screen_width, self.screen_height);

        // Draw some pre-made meshes
        for m in &self.meshes {
            graphics::draw(ctx, m, DrawParam::new())?;
        }
		
		for line in &self.board {
			for bloc in line {
				draw_bloc(assets, ctx, bloc, coords)?;
			}
		}

        graphics::present(ctx)?;
		timer::yield_now();
		Ok(())
    }
}
