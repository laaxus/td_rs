use ggez::nalgebra;

use crate::{BLOC_LENGTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::{board_to_world_coords};
use serde::Deserialize;
use serde::Serialize;

type Point2 = nalgebra::Point2<f32>;
type Vector2 = nalgebra::Vector2<f32>;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum BlocType {
    Orange,
    Bleu,
    Gris,
    Noir,
    Rouge,
}

pub fn change_bloc_type(bt: &BlocType) -> BlocType {
    match bt {
        BlocType::Orange => BlocType::Bleu,
        BlocType::Bleu => BlocType::Gris,
        BlocType::Gris => BlocType::Noir,
        BlocType::Noir => BlocType::Rouge,
        BlocType::Rouge => BlocType::Orange,
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bloc {
    pub tag: BlocType,
    pub pos: Point2,
    pub parent: Option<(usize, usize)>,
}

impl Bloc {
    pub fn new_orange() -> Bloc {
        Bloc {
            tag: BlocType::Orange,
            pos: Point2::origin(),
            parent: None,
        }
    }

    pub fn new_bleu() -> Bloc {
        Bloc {
            tag: BlocType::Bleu,
            pos: Point2::origin(),
            parent: None,
        }
    }

    pub fn new_gris(i: u32, j: u32) -> Bloc {
        Bloc {
            tag: BlocType::Gris,
            pos: Point2::new(
                (i as f32) * BLOC_LENGTH - SCREEN_WIDTH / 2.0,
                SCREEN_HEIGHT / 2.0 - (j as f32) * BLOC_LENGTH,
            ),
            parent: None,
        }
    }

    pub fn new_noir() -> Bloc {
        Bloc {
            tag: BlocType::Noir,
            pos: Point2::origin(),
            parent: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MobType {
	Vert,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mob {
	pub tag : MobType,
	pub life : u32,
	pub starting_bloc : (usize,usize),
	pub mob_size : f32,
	pub pos : Vector2,
    speed : f32,
	pub dir : Vector2, 
	pub dist : Vector2,
} 

impl Mob {
	pub fn new_vert(i : usize, j : usize) -> Mob {
		let (x,y) = board_to_world_coords(i,j);
        
		let mob_size = 20.0;
		let dif = (BLOC_LENGTH - mob_size)/2.0;
		Mob{
			tag : MobType::Vert,
			life : 100,
			starting_bloc : (i,j),
			mob_size,
			pos : Vector2::new(x + dif,y - dif),
			speed : 4.0,
			dir : Vector2::new(0.01,0.01),
			dist : 	Vector2::new(100.0,100.0),
		}
	}
	
	pub fn walk(self : &mut Self) {
		self.pos += self.dir * self.speed;
		self.dist += self.dir * self.speed;
	}
	
	pub fn update(self : &mut Self, dest : Vector2) {
		if self.dist.x.abs() + self.dist.y.abs() >= BLOC_LENGTH {
			println!("{:?}",dest);
			self.dir = dest.normalize();
			self.dist = Vector2::new(0.0,0.0);
		}
	}
}
