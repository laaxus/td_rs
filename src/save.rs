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
