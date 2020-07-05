use ggez::nalgebra;
use ggez::graphics::Rect;

use crate::{BLOC_LENGTH};
use crate::{board_to_world_coords};



type Vector2 = nalgebra::Vector2<f32>;

#[derive(Clone)]
pub enum MobType {
	Vert,
}

#[derive( Clone)]
pub struct Mob {
	pub alive : bool,
	pub tag : MobType,
	pub life : f32,
	pub rect : Rect,
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
			alive : true,
			tag : MobType::Vert,
			life : 100.0,
			rect : Rect::new(x+dif,y-dif,mob_size,mob_size),
			speed : 4.0,
			dir : Vector2::new(0.01,0.01),
			dist : 	Vector2::new(100.0,100.0),
		}
	}
	
	pub fn walk(self : &mut Self) {
		if self.life <= 0.0 {
			self.alive = false;
		}
		if self.alive {
			self.rect.x += self.dir.x * self.speed;
			self.rect.y += self.dir.y * self.speed;
			self.dist += self.dir * self.speed;
		}
	}
	
	pub fn update(self : &mut Self, dest : Vector2) {
		if self.dist.x.abs() + self.dist.y.abs() >= BLOC_LENGTH {
			self.dir = dest.normalize();
			self.dist = Vector2::new(0.0,0.0);
		}
	}
	
	pub fn pos(&self) -> Vector2 {
		Vector2::new(self.rect.x, self.rect.y)
	}
}