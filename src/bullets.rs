use ggez::nalgebra;
use ggez::graphics::Rect;

type Vector2 = nalgebra::Vector2<f32>;

#[derive(Clone)]
pub enum BulletType {
	CannonBall,
}

#[derive(Clone)]
pub struct Bullet {
	pub alive: bool,
	pub tag: BulletType,
	pub dmg : f32,
	pub rect: Rect,
	pub speed: f32,
	pub dir: Vector2,
}

impl Bullet {
	pub fn new_cb(pos : Vector2) -> Bullet {
		let alive = true;
		let tag = BulletType::CannonBall;
		let rect = Rect::new(pos.x,pos.y,10.0,10.0);
		let dir = Vector2::new(1.0,0.0);
		let speed = 20.0;
		let dmg = 100.0;
		Bullet {
			alive,
			tag,
			dmg,
			rect,
			speed,
			dir,
		}
	}
	
	pub fn walk(self : &mut Self) {
		if self.alive {
			self.rect.x += self.dir.x * self.speed;
			self.rect.y += self.dir.y * self.speed;
		}
	}
	
	pub fn pos(&self) -> Vector2 {
		Vector2::new(self.rect.x, self.rect.y)
	}
}