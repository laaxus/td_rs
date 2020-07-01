use crate::blocs::{BlocType,Bloc};
use ron::de::from_str;
use ron::ser::to_string;
use serde::Deserialize;
use serde::Serialize;
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
struct Save {
    board: Vec<Vec<String>>,
}

pub fn save(original_board: &Vec<Vec<Bloc>>) -> Result<()> {
    let mut board: Vec<Vec<String>> = vec![];
    for subvec in original_board {
        let vec: Vec<String> = subvec.iter().map(|x| bt_to_str(x)).collect();
        board.push(vec);
    }

    let data = Save { board };

    let s = to_string(&data).expect("Serialization failed");

    let mut file = File::create("save.ron")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

pub fn load() -> Result<Vec<Vec<Bloc>>> {
    let mut file = File::open("save.ron")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let save: Save = from_str(&contents).unwrap();

    let mut board: Vec<Vec<Bloc>> = vec![];

    for subvec in save.board {
        let vec: Vec<Bloc> = subvec.iter().map(|x| str_to_bt(x)).collect();
        board.push(vec);
    }
    Ok(board)
}
