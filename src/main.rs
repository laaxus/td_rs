use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra;
use std::env;
use ggez::timer;
use std::path;
use ggez::input::keyboard;
mod blocs;


type Point2 = nalgebra::Point2<f32>;
type Vector2 = nalgebra::Vector2<f32>;

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 600.;

const WORLD_WIDTH: f32 = 1200.;
const WORLD_HEIGHT: f32 = 1200.;

const CAMERA_SPEED: f32 = 3.0;

/// *************************************************************************************************
/// Loading every images, sounds, etc.
/// I may have to change it for an actual asset management system later on.
/// For now, it's just hard-coded.
/// *************************************************************************************************

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

/// **************************************************************************************************
/// Constructors functions for different game objects.
/// **************************************************************************************************

fn create_board() -> Vec<Vec<blocs::Bloc> > {
	let mut vec = vec![];
	
	let mut foo = vec![];
	for i in 0..15 {
		let mut bleu: blocs::Bloc = blocs::Bloc::new_bleu();
		bleu.pos = Point2::new(-SCREEN_WIDTH/2.0 + 40.0*(i as f32), SCREEN_HEIGHT/2.0);
		foo.push(bleu);
	}
	vec.push(foo);
	
	let mut foo2 = vec![];
	for i in 0..15 {
		let mut orange: blocs::Bloc = blocs::Bloc::new_orange();
		orange.pos = Point2::new(-SCREEN_WIDTH/2.0 + 40.0*(i as f32), SCREEN_HEIGHT/2.0-40.0);
		foo2.push(orange);
	}
	vec.push(foo2);

	vec
}

/// **************************************************************************************************
/// A couple of utility functions.
/// **************************************************************************************************

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
	origin: Point2,
	) -> GameResult {
	
		let (screen_w, screen_h) = world_coords;
		let pos = world_to_screen_coords(screen_w, screen_h, subs_p2(bloc.pos,origin));
		let image = assets.bloc_image(bloc);
		let draw_params = graphics::DrawParam::new()
			.dest(pos);
		graphics::draw(ctx, image, draw_params)
}

fn adds_p2(a : Point2, b : Point2) -> Point2 {
	Point2::new(a.x + b.x, a.y + b.y)
}

fn subs_p2(a : Point2, b : Point2) -> Point2 {
	Point2::new(a.x - b.x, a.y - b.y)
}



fn main() -> GameResult {
 
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb
	.window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
	.build()
	.unwrap();

    println!("{}", graphics::renderer_info(ctx)?);
    let game = &mut MyGame::new(ctx).unwrap();
    event::run(ctx, events_loop, game)
}

struct MyGame {
    // Your state here...
    assets: Assets,
	board: Vec<Vec<blocs::Bloc>>,
	screen_width: f32,
    screen_height: f32,
	origin : Point2,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        // Load/create resources such as images here.
        let assets = Assets::new(ctx)?;
		let board = create_board();
		let (screen_width, screen_height) = graphics::drawable_size(ctx);
		let origin = Point2::new(0.0,0.0);
		let zoom = 1.0;
		
        let s = MyGame {
            // ...
            assets,
			board,
			screen_width,
			screen_height,
			origin,			
        };

        Ok(s)
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        
		 const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
		
			if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Up) {
            self.origin.y += CAMERA_SPEED;
			}
			if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Down) {
				self.origin.y -= CAMERA_SPEED;
			}
			if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Right) {
				self.origin.x += CAMERA_SPEED;
			}
			if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Left) {
				self.origin.x -= CAMERA_SPEED;
			}
			
            
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
		
		let assets = &mut self.assets;
        let coords = (self.screen_width, self.screen_height);
		
		for line in &self.board {
			for bloc in line {
				draw_bloc(assets, ctx, bloc, coords, self.origin)?;
			}
		}

        graphics::present(ctx)?;
		// This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
		timer::yield_now();
		Ok(())
    }
}
