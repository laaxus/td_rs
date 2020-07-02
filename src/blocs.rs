use ggez::nalgebra;

use crate::{BLOC_LENGTH, SCREEN_HEIGHT, SCREEN_WIDTH};
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
    Vert,
}

pub fn change_bloc_type(bt: &BlocType) -> BlocType {
    match bt {
        BlocType::Orange => BlocType::Bleu,
        BlocType::Bleu => BlocType::Gris,
        BlocType::Gris => BlocType::Noir,
        BlocType::Noir => BlocType::Rouge,
        BlocType::Rouge => BlocType::Orange,
        BlocType::Vert => BlocType::Vert,
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
	tag : MobType,
	life : u32,
	pos : Point2,
	pos_in_block : Vector2,
	dir : Vector2, 
} 

impl Mob {
	pub fn new_vert(i : f32, j : f32) -> Mob {
		let x = (i as f32) * BLOC_LENGTH - SCREEN_WIDTH / 2.0;
        let y = SCREEN_HEIGHT / 2.0 - (j as f32) * BLOC_LENGTH;
		Mob{
			tag : MobType::Vert,
			life : 100,
			pos : Point2::new(x,y),
			pos_in_block : Vector2::new(0.0,0.0),
			dir : Vector2::new(0.0,0.0), 
		}
	}
}
