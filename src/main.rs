use ggez::event::{self, EventHandler};
use ggez::graphics::{DrawParam};
use ggez::input::{keyboard,mouse::MouseButton};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra;
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use math::round::floor;
use std::env;
use std::path;


mod blocs;
mod ingame;
mod save;
mod mobs;
mod bullets;
mod towers;

type Point2 = nalgebra::Point2<f32>;
type Vector2 = nalgebra::Vector2<f32>;

const BLOC_LENGTH: f32 = 60.0;

const SCREEN_HEIGHT: f32 = 900.;
const SCREEN_WIDTH: f32 = 1500.;

const CAMERA_SPEED: f32 = 3.0;

const CREATIVE_PANNEL_WIDTH: f32 = 250.0;

/// *************************************************************************************************
/// Loading every images, sounds, etc.
/// I may have to change it for an actual asset management system later on.
/// For now, it's just hard-coded.
/// *************************************************************************************************

struct Assets {
    bloc_orange: graphics::Image,
    bloc_bleu: graphics::Image,
    bloc_gris: graphics::Image,
    bloc_noir: graphics::Image,
    bloc_rouge: graphics::Image,
    mob_vert: graphics::Image,
	bullet_noir: graphics::Image,
	tower_cannon : graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let bloc_orange = graphics::Image::new(ctx, "/orange_60.png")?;
        let bloc_bleu = graphics::Image::new(ctx, "/bleu_60.png")?;
        let bloc_gris = graphics::Image::new(ctx, "/gris_60.png")?;
        let bloc_noir = graphics::Image::new(ctx, "/noir_60.png")?;
        let bloc_rouge = graphics::Image::new(ctx, "/rouge_60.png")?;
        let mob_vert = graphics::Image::new(ctx, "/vert_20.png")?;
        let bullet_noir = graphics::Image::new(ctx, "/noir_10.png")?;
		let tower_cannon = graphics::Image::new(ctx, "/cannon_60.png")?;
        Ok(Assets {
            bloc_orange,
            bloc_bleu,
            bloc_gris,
            bloc_noir,
            bloc_rouge,
            mob_vert,
			bullet_noir,
			tower_cannon,
        })
    }

    fn bloc_image(&mut self, bloctype: &blocs::BlocType) -> &mut graphics::Image {
        match bloctype {
            blocs::BlocType::Orange => &mut self.bloc_orange,
            blocs::BlocType::Bleu => &mut self.bloc_bleu,
            blocs::BlocType::Noir => &mut self.bloc_noir,
            blocs::BlocType::Gris => &mut self.bloc_gris,
            blocs::BlocType::Rouge => &mut self.bloc_rouge,
        }
    }
	
	fn mob_image(&mut self, mobtype: &mobs::MobType) -> &mut graphics::Image {
        match mobtype {
            mobs::MobType::Vert => &mut self.mob_vert,
        }
    }
	
	fn bullet_image(&mut self, bullettype: &bullets::BulletType) -> &mut graphics::Image {
        match bullettype {
            bullets::BulletType::CannonBall => &mut self.bullet_noir,
        }
    }
	
	fn tower_image(&mut self, towertype: &towers::TowerType) -> &mut graphics::Image {
        match towertype {
           towers::TowerType::Cannon => &mut self.tower_cannon,
        }
    }
}

/// **************************************************************************************************
/// Constructors functions for different game objects.
/// **************************************************************************************************

fn create_board_rect(x: u32, y: u32) -> Vec<Vec<blocs::Bloc>> {
    let mut vec = vec![];

    for i in 0..x {
        let mut foo = vec![];
        for j in 0..y {
            let bloc = blocs::Bloc::new_gris(j, i);
            foo.push(bloc);
        }
        vec.push(foo);
    }
    vec
}

/// **************************************************************************************************
/// A couple of utility functions.
/// **************************************************************************************************

/// Translates the world coordinate system, which
/// has Y pointing up and the origin at the center,
/// to the screen coordinate system, which has Y
/// pointing downward and the origin at the top-left,
fn world_to_screen_coords(point: Point2) -> Point2 {
    let x = point.x + SCREEN_WIDTH / 2.0;
    let y = SCREEN_HEIGHT / 2.0 - point.y;
    Point2::new(x, y)
}

fn screen_to_world_coords(point: Point2) -> Point2 {
    let x = point.x - SCREEN_WIDTH / 2.0;
    let y = SCREEN_HEIGHT / 2.0 - point.y;
    Point2::new(x, y)
}

fn screen_to_board_coords(x:f32, y:f32, origin:Vector2) -> (usize,usize) {
	let j: usize = floor(((x + origin.x) / BLOC_LENGTH).into(), 0) as usize;
    let i: usize = floor(((y + origin.y) / BLOC_LENGTH).into(), 0) as usize;
    (i,j)
}

fn world_to_board_coords(x:f32, y:f32) -> (usize,usize) {
	let j = floor(((x + SCREEN_WIDTH / 2.0) / BLOC_LENGTH).into(),0) as usize;
	let i = floor(((SCREEN_HEIGHT / 2.0 - y) / BLOC_LENGTH).into(),0) as usize;
	(i,j)
}

fn board_to_world_coords(i : usize, j : usize) -> (f32,f32) {
	let x = (j as f32) * BLOC_LENGTH - SCREEN_WIDTH / 2.0;
    let y = SCREEN_HEIGHT / 2.0 - (i as f32) * BLOC_LENGTH;
	(x,y)
}

fn vector_to_radian(vec:Vector2) -> f32 {
	 vec.y.atan2(vec.x)
}
/// **************************************************************************************************
/// A couple of drawing functions.
/// **************************************************************************************************

fn draw_board(ctx: &mut Context, mygame: &mut MyGame) -> GameResult {
    let assets = &mut mygame.assets;
    let board = &mygame.map.board;
    let origin = mygame.origin.clone();

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let bloc_pos = board[i][j].pos;
            let pos = world_to_screen_coords(bloc_pos) - origin;
            let image = assets.bloc_image(&board[i][j].tag);
            let draw_params = graphics::DrawParam::new().dest(pos);
            graphics::draw(ctx, image, draw_params).unwrap();
        }
    }
    Ok(())
}

fn draw_mobs(ctx: &mut Context, mygame: &mut MyGame) -> GameResult {
    let assets = &mut mygame.assets;
    let mobs = &mygame.mobs;
    let origin = mygame.origin.clone();

    for i in 0..mobs.len() {
		if mobs[i].alive {
			let mob_pos = mobs[i].rect;
			let pos = world_to_screen_coords(Point2::new(mob_pos.x,mob_pos.y)) - origin;
			let image = assets.mob_image(&mobs[i].tag);
			let draw_params = graphics::DrawParam::new().dest(pos);
			graphics::draw(ctx, image, draw_params).unwrap();   
		}
	}
    Ok(())
}

fn draw_bullets(ctx: &mut Context, mygame: &mut MyGame) -> GameResult {
    let assets = &mut mygame.assets;
    let bullets = &mygame.bullets;
    let origin = mygame.origin.clone();

    for i in 0..bullets.len() {
		if bullets[i].alive {
			let rect = bullets[i].rect;
			let pos = world_to_screen_coords(Point2::new(rect.x,rect.y)) - origin;
			let image = assets.bullet_image(&bullets[i].tag);
			let draw_params = graphics::DrawParam::new().dest(pos);
			graphics::draw(ctx, image, draw_params).unwrap();   
		}
	}
    Ok(())
}

fn draw_towers(ctx: &mut Context, mygame: &mut MyGame) -> GameResult {
    let assets = &mut mygame.assets;
	let origin = mygame.origin.clone();
	
	for g in 0..mygame.settings.board_height {
		for h in 0..mygame.settings.board_width {
			if let Some(tower) = &mut mygame.towers[g][h] {
				let rect = tower.rect;
				let pos = world_to_screen_coords(Point2::new(rect.x,rect.y)) - origin;
				let image = assets.tower_image(&tower.tag);
				let draw_params = graphics::DrawParam::new()
					.dest(pos)
					.rotation(tower.facing)
					.offset(Point2::new(0.5, 0.5));
				graphics::draw(ctx, image, draw_params).unwrap();   
			}
		}
	}
	
	Ok(())
}

fn draw_creative_pannel(ctx: &mut Context) -> GameResult {

    // Create and draw a filled rectangle mesh.
    let rect = graphics::Rect::new(
        SCREEN_WIDTH - CREATIVE_PANNEL_WIDTH,
        0.0,
        CREATIVE_PANNEL_WIDTH,
        SCREEN_HEIGHT,
    );
    let r1 = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        rect,
        graphics::Color::new(0.8, 0.8, 0.8, 1.0),
    )?;
    graphics::draw(ctx, &r1, DrawParam::default())?;
    Ok(())
}


/// **************************************************************************************************
/// A couple of enum and struct relative to the main game
/// **************************************************************************************************

#[derive(PartialEq)]
enum GameMode {
    Creative,
    Normal,
}

struct Settings {
    gamemode: GameMode,
    board_height: usize,
    board_width: usize,
}

struct MyGame {
    assets: Assets,
    pub map: save::Map,
    origin: Vector2,
    settings: Settings,
	mobs: Vec<mobs::Mob>,
	bullets: Vec<bullets::Bullet>,
	towers: Vec<Vec<Option<towers::Tower>>>,
}

fn load_board() -> save::Map {
    match save::load() {
        Ok(save) => save.map,
        Err(_) => save::Map{ 
			board : create_board_rect(14, 20),
		},
    }
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let assets = Assets::new(ctx)?;
        let map = load_board();
        // let board = create_board_rect(14,20);
        let origin = Vector2::new(0.0, 0.0);

        let gamemode = GameMode::Normal;
        let board_height = map.board.len();
        let board_width = map.board[0].len();
        let settings = Settings {
            gamemode,
            board_height,
            board_width,
        };
		
		let mobs: Vec<mobs::Mob> = vec![];
		let bullets:Vec<bullets::Bullet> = vec![];
		let mut towers : Vec<Vec<Option<towers::Tower>>> = vec![];
		let mut sub_towers : Vec<Option<towers::Tower>> = vec![];
		sub_towers.resize(board_width,None);
		towers.resize(board_height,sub_towers);

        let s = MyGame {
            // ...
            assets,
            map,
            origin,
            settings,
			mobs,
			bullets,
			towers,
        };

        Ok(s)
    }
	
	fn clear_dead_stuff(&mut self){
		self.mobs.retain(|x| x.alive);
		self.bullets.retain(|x| x.alive);
	}
	
	/// **************************************************************************************************
	/// Functions handeling actors.
	/// **************************************************************************************************

	fn handle_mobs(&mut self) {
		for mob in &mut self.mobs {
			if mob.alive {
				let (i,j) = world_to_board_coords(mob.rect.x, mob.rect.y);
				if let Some((k,l)) = self.map.board[i][j].parent{
						let m = i as f32 - k as f32;
						let n = l as f32 - j as f32;
						let dest = Vector2::new(n, m);
						mob.update(dest);
				}
				mob.walk();
				let (i,j) = world_to_board_coords(mob.rect.x+mob.rect.w/2.0, mob.rect.y-mob.rect.w/2.0);
				if self.map.board[i][j].tag == blocs::BlocType::Rouge {
					mob.alive = false;
				}
			}
		}
	}

	fn handle_bullets(&mut self) {
		for bullet in &mut self.bullets {
			bullet.walk();
		}
	}
	
	fn handle_towers(&mut self) {
		for i in 0..self.settings.board_height {
			for j in 0..self.settings.board_width {
				if let Some(tower) = &mut self.towers[i][j] {
					
					let mut mob_in_range = false;
					// handle rotation
					if &self.mobs.len() > &0 {
						let mut dist : f32 = tower.range;
						let mut pos = Vector2::new(0.0,0.0);
						for mob in &mut self.mobs {
							let dt = (tower.pos()-mob.pos()).norm();
	
							if dt < dist {
								mob_in_range = true;
								dist = dt;
								pos = mob.pos();
							}
						}
						let mut dir = (pos-tower.pos()).normalize();
						dir.y *= -1.0;
						tower.facing = vector_to_radian(dir);
					}
					
					tower.time_to_shoot -= 1.0/ 60.0;
					if tower.time_to_shoot < 0.0 && mob_in_range{
						tower.time_to_shoot = tower.time_between_shot;
						self.bullets.push(tower.shoot());
					}
				
				}
			}
		}
		
	}

	fn handle_collision(&mut self) {
		for mob in &mut self.mobs {
			for bullet in &mut self.bullets {
				if mob.rect.overlaps(&mut bullet.rect) {
					mob.life-= bullet.dmg;
					bullet.alive = false;
				}
			}
		}
	} 
	
	/// **************************************************************************************************
	/// Other functions
	/// **************************************************************************************************
	
	fn add_tower(&mut self, k:usize, l:usize) {
		let (i,j) = board_to_world_coords(k,l);
		let tower = towers::Tower::new(Vector2::new(i+30.0,j-30.0));
		self.towers[k][l] = Some(tower);
	}
}

/// **************************************************************************************************
/// The main func + event handler
/// **************************************************************************************************

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

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Up) {
                self.origin.y -= CAMERA_SPEED;
            }
            if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Down) {
                self.origin.y += CAMERA_SPEED;
            }
            if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Right) {
                self.origin.x += CAMERA_SPEED;
            }
            if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Left) {
                self.origin.x -= CAMERA_SPEED;
            }
			
			self.handle_mobs();
			
			self.handle_bullets();
			
			self.handle_towers();
			
			self.handle_collision();
			
			
			self.clear_dead_stuff();
			

            self.origin.x = self.origin.x.max(-1.0 * BLOC_LENGTH);
            self.origin.y = self.origin.y.max(-1.0 * BLOC_LENGTH);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.7, 0.7, 0.7, 1.0));

        draw_board(ctx, self)?;
		
		draw_mobs(ctx, self)?;
		
		draw_bullets(ctx, self)?;
		
		draw_towers(ctx, self)?;

        if self.settings.gamemode == GameMode::Creative {
            draw_creative_pannel(ctx)?;
        }
        graphics::present(ctx)?;

        // This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left
            && GameMode::Creative == self.settings.gamemode
            && x < SCREEN_WIDTH - CREATIVE_PANNEL_WIDTH
        {
            let (i,j) = screen_to_board_coords(x,y,self.origin);
            if i < self.settings.board_height && j < self.settings.board_width {
                self.map.board[i][j].tag = blocs::change_bloc_type(&self.map.board[i][j].tag);
            }
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            ggez::event::quit(ctx);
        } else if keycode == KeyCode::C {
            self.settings.gamemode = GameMode::Creative;
			
        } else if keycode == KeyCode::N {
            self.settings.gamemode = GameMode::Normal;
			
        } else if keycode == KeyCode::S {
			let map = self.map.clone();
            let save = save::Save {map};
            save::save(&save).expect("Failed to save");
			
        } else if keycode == KeyCode::L {
            self.map = load_board();
			
        } else if keycode == KeyCode::M {
            self.map.board = create_board_rect(14, 20);
			
        }else if keycode == KeyCode::P {
            ingame::find_path(&mut self.map.board).expect("Error finding Path");
			println!("{:?}",self.map.board[8][9].parent);
			
        }else if keycode == KeyCode::A {
			ingame::find_path(&mut self.map.board).expect("Error finding Path");
            self.mobs.push(mobs::Mob::new_vert(6,0));
			
        }else if keycode == KeyCode::T {
            self.add_tower(4,5);
			
        }else if keycode == KeyCode::D {
			let pos = board_to_world_coords(4,5);
            let bullet = bullets::Bullet::new_cb(Vector2::new(pos.0+25.0,pos.1-25.0));
			self.bullets.push(bullet); 
			
        }
    }
}
