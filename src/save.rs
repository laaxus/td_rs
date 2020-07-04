use crate::blocs::Bloc;
use ron::de::from_reader;
use ron::ser::to_string;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, prelude::*, Result};
use std::fs::File;




#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
	pub board: Vec<Vec<Bloc>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Save {
    pub map: Map,
}

pub fn save(save: &Save) -> Result<()> {
    let s = to_string(&save).expect("Serialization failed");

    let mut file = File::create("save.ron")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

pub fn load() -> Result<Save> {
    let file = File::open("save.ron")?;

	let custom_error = Error::new(ErrorKind::Other, "Error from reader loading save");
    match from_reader(file) {
        Ok(x) => Ok(x),
        Err(_) => Err(custom_error),
    }

    
}
