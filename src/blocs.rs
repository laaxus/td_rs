use ggez::nalgebra;

use crate::{BLOC_LENGTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use serde::Deserialize;
use serde::Serialize;

type Point2 = nalgebra::Point2<f32>;


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

}




