use ggez::nalgebra;

type Point2 = nalgebra::Point2<f32>;
type Vector2 = nalgebra::Vector2<f32>;

pub enum BlocType {
    Orange,
    Bleu,
	Gris,
	Noir,
}

pub struct Bloc {
    pub tag: BlocType,
    pub pos: Point2,
}

impl Bloc {
    pub fn new_orange() -> Bloc {
        Bloc {
            tag: BlocType::Orange,
            pos: Point2::origin(),
        }
    }

    pub fn new_bleu() -> Bloc {
        Bloc {
            tag: BlocType::Bleu,
            pos: Point2::origin(),
        }
    }
	
	pub fn new_gris() -> Bloc {
        Bloc {
            tag: BlocType::Gris,
            pos: Point2::origin(),
        }
    }
	
	pub fn new_noir() -> Bloc {
        Bloc {
            tag: BlocType::Noir,
            pos: Point2::origin(),
        }
    }
}