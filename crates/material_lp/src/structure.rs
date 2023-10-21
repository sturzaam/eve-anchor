// extern crate rustc_serialize;
extern crate bincode;

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::error::Error;
use crate::data::{find_system, get_system};


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Corporation {
    pub name: String,
    pub alliance: Alliance,
    pub outposts: Vec<Outpost>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Alliance {
    pub name: String,
    pub corporations: Vec<Corporation>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Capsuleer {
    pub name: String,
    pub corporation: Corporation,
    pub planetology: i64,
    pub advanced_planetology: i64,
    pub expert_planetology: i64,
}

impl Capsuleer {
    pub fn new(
        name: String,
        corporation: Corporation,
        planetology: i64,
        advanced_planetology: i64,
        expert_planetology: i64
    ) -> Self {
        Capsuleer {
            name,
            corporation, 
            planetology: std::cmp::max(std::cmp::min(planetology, 5), 0),
            advanced_planetology: std::cmp::max(std::cmp::min(advanced_planetology, 5), 0),
            expert_planetology: std::cmp::max(std::cmp::min(expert_planetology, 5), 0),
        }
    }

    pub fn planetology(&self) -> i64 {
        std::cmp::min(self.planetology, 5)
    }

    pub fn advanced_planetology(&self) -> i64 {
        std::cmp::min(self.advanced_planetology, 5)
    }
}


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Outpost {
    pub name: String,
    pub capsuleer: Capsuleer,
    pub system_id: i64,
    pub constellation_id: i64
}

impl Outpost {
    pub fn new(name: String, capsuleer: Capsuleer, system_name: String, key: String) -> Result<Self, Box<dyn Error>> {
        let system_id = find_system(&system_name).unwrap();
        let system = get_system(*system_id).unwrap();

        let outpost = Outpost {
            name: name.clone(),
            capsuleer: capsuleer.clone(),
            system_id: *system_id,
            constellation_id: *system.constellation,
        };

        let file_name = format!("{}.bin", key);
        let mut corporation = Self::load(&file_name)?;
        Self::update(&outpost, &mut corporation)?;
        Self::save(&file_name, &corporation)?;

        Ok(outpost)
    }

    fn load(file_name: &str) -> Result<Corporation, Box<dyn Error>> {
        let outpost_dir = Self::get_outpost_dir()?;
        let file_path = outpost_dir.join(file_name);

        if file_path.exists() {
            let mut file = File::open(&file_path)?;
            let mut corporation_data = Vec::new();
            file.read_to_end(&mut corporation_data)?;
            Ok(bincode::deserialize(&corporation_data)?)
        } else {
            fs::create_dir_all(&outpost_dir)?;
            Ok(Capsuleer::default().corporation)
        }
    }

    fn update(outpost: &Outpost, corporation: &mut Corporation) -> Result<(), Box<dyn Error>> {
        if let Some(index) = corporation.outposts.iter().position(|decoded| decoded.capsuleer.name == outpost.capsuleer.name) {
            corporation.outposts.remove(index);
        }
        
        corporation.outposts.push(outpost.clone());

        Ok(())
    }

    fn save(file_name: &str, corporation: &Corporation) -> Result<(), Box<dyn Error>> {
        let outpost_dir = Self::get_outpost_dir()?;
        let file_path = outpost_dir.join(file_name);

        let encoded = bincode::serialize(&corporation)?;
        let mut file = File::create(&file_path)?;
        file.write_all(&encoded)?;

        Ok(())
    }

    fn get_outpost_dir() -> Result<PathBuf, Box<dyn Error>> {
        let out_dir = "./target".to_string();
        let out_path = PathBuf::from(out_dir);
        let outpost_dir = out_path.join("outpost");
        Ok(outpost_dir)
    }
}