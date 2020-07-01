use crate::blocs::{BlocType,Bloc};
use ron::de::from_str;
use ron::de::from_reader;
use ron::ser::to_string;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use ggez::nalgebra;


type Point2 = nalgebra::Point2<f32>;



fn bt_to_str(bloc: &Bloc) -> String {
    let tag =  match bloc.tag {
        BlocType::Orange => String::from("Or"),
        BlocType::Bleu => String::from("Bl"),
        BlocType::Gris => String::from("Gr"),
        BlocType::Noir => String::from("No"),
        BlocType::Rouge => String::from("Ro"),
        BlocType::Vert => String::from("Ve"),
    };
	let x = bloc.pos.x.to_string();
	let y = bloc.pos.y.to_string();
	format!("{} {} {}",tag,x,y)
}

fn str_to_bt(string: &str) -> Bloc {
	let vec : Vec<&str> = string.split(" ").collect();
    let tag = match vec[0] {
        "Or" => BlocType::Orange,
        "Bl" => BlocType::Bleu,
        "Gr" => BlocType::Gris,
        "No" => BlocType::Noir,
        "Ro" => BlocType::Rouge,
        "Ve" => BlocType::Vert,
        _ => BlocType::Noir,
    };
	let x : f32 = vec[1].parse().expect("Error parsing loading save");
	let y : f32 = vec[2].parse().expect("Error parsing loading save");
	let pos = Point2::new(x,y);
	
	Bloc{
		tag,
		pos,
	}
}

#[derive(Serialize, Deserialize)]
pub struct Save {
    pub board: Vec<Vec<Bloc>>,
}

pub fn save(save: &Save) -> Result<()> {
 

    let s = to_string(&save).expect("Serialization failed");

    let mut file = File::create("save.ron")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

pub fn load() -> Result<Save> {
    let mut file = File::open("save.ron")?;
    

    let save: Save = match from_reader(file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load save: {}", e);

            std::process::exit(1);
        }
    };
	
    Ok(save)
}
