use crate::blocs::BlocType;
use ron::ser::to_string;
use ron::de::from_str;
use serde::Serialize;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn bt_to_str(bt: &BlocType) -> String {
    match bt {
        BlocType::Orange => String::from("Or"),
        BlocType::Bleu => String::from("Bl"),
        BlocType::Gris => String::from("Gr"),
        BlocType::Noir => String::from("No"),
    }
}

fn str_to_bt(string: &str) -> BlocType {
    match string {
        "Or" => BlocType::Orange,
        "Bl" => BlocType::Bleu,
        "Gr" => BlocType::Gris,
        "No" => BlocType::Noir,
        _ => BlocType::Noir,
    }
}

#[derive(Serialize,Deserialize)]
struct Save {
    board: Vec<Vec<String>>,
}

pub fn save(original_board: &Vec<Vec<BlocType>>) -> Result<()> {
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

pub fn load() -> Result<Vec<Vec<BlocType>>> {
	let mut file = File::open("save.ron")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
	
	let save : Save = from_str(&contents).unwrap();
	
	let mut board : Vec<Vec<BlocType> > = vec![];
	
	for subvec in save.board {
        let vec: Vec<BlocType> = subvec.iter().map(|x| str_to_bt(x)).collect();
        board.push(vec);
    }
	Ok(board)
}
