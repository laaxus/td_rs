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
	pub pos_in_block : Vector2,
	pub max_speed : f32,
	pub max_acc : f32,
	pub speed : Vector2, 
	pub acc : Vector2,
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
			pos_in_block : Vector2::new(0.0,0.0),
			max_speed : 2.0,
			max_acc : 0.092,
			speed : Vector2::new(0.01,0.01), 
			acc : Vector2::new(0.01,0.01), 
		}
	}
	
	pub fn update(self : &mut Self, dest : Vector2){
		let dir = (dest - (self.pos + Vector2::new(self.mob_size/2.0,-self.mob_size/2.0))).normalize();
		self.acc = (self.acc + (dir-self.speed.normalize())*0.5);
		if self.acc.norm() > self.max_acc {
			self.acc = self.acc.normalize()*self.max_acc;
		}
		self.speed += self.acc;
		if self.speed.norm() > self.max_speed {
			self.speed = self.speed.normalize()*self.max_speed;
		}
		self.pos += self.speed ; 
	}
	
}
