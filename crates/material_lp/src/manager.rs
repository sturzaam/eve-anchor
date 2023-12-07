// extern crate rustc_serialize;
extern crate bincode;

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::error::Error;
use crate::data::{find_system, get_system};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Alliance {
    pub name: String,
    pub corporations: Vec<Corporation>,
    pub outposts: Vec<Outpost>,
}

impl Alliance {
    pub fn new(name: String) -> Result<Self, Box<dyn Error>> {
        let alliance = Alliance {
            name,
            corporations: vec![],
            outposts: vec![],
        };
        Ok(alliance)
    }

    pub fn get_dir(&self) -> PathBuf {
        Path::new("./target").join(&self.name)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Corporation {
    pub name: String,
    pub alliance: Alliance,
    pub outposts: Vec<Outpost>,
}

impl Corporation {
    pub fn new(name: String, alliance: &Alliance) -> Result<Self, Box<dyn Error>> {
        let corporation = Corporation {
            name,
            alliance: alliance.clone(),
            outposts: vec![],
        };
        Ok(corporation)
    }

    pub fn get_dir(&self) -> PathBuf {
        self.alliance.get_dir().join(&self.name)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Member {
    pub name: String,
    pub corporation: Corporation,
    pub outposts: Vec<Outpost>,
}

impl Member {
    pub fn new(name: String, corporation: &Corporation) -> Result<Self, Box<dyn Error>> {
        let member = Member {
            name,
            corporation: corporation.clone(),
            outposts: vec![],
        };
        Ok(member)
    }

    pub fn get_dir(&self) -> PathBuf {
        self.corporation.get_dir().join(&self.name)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Manager {
    Alliance(Alliance),
    Corporation(Corporation),
    Member(Member),
}

impl Manager {
    fn get_dir(&self) -> PathBuf {
        match self {
            Manager::Alliance(alliance) => alliance.get_dir(),
            Manager::Corporation(corporation) => corporation.get_dir(),
            Manager::Member(member) => member.get_dir(),
        }
    }

    pub fn get_outposts(&self) -> &Vec<Outpost> {
        match self {
            Manager::Alliance(alliance) => &alliance.outposts,
            Manager::Corporation(corporation) => &corporation.outposts,
            Manager::Member(member) => &member.outposts,
        }
    }

    fn get_outposts_mut(&mut self) -> &mut Vec<Outpost> {
        match self {
            Manager::Alliance(alliance) => &mut alliance.outposts,
            Manager::Corporation(corporation) => &mut corporation.outposts,
            Manager::Member(member) => &mut member.outposts,
        }
    }

    pub fn add_outpost(
        &mut self,
        outpost_name: &str,
        outpost_system: &str,
        capsuleer_name: &str,
        available_planets: i32,
        available_arrays: i32
    ) -> Result<(), Box<dyn Error>> {
        if self.get_outposts().iter().any(|o| o.name == outpost_name) {
            return Err("An outpost with the same name already exists for this member.".into());
        }
        let capsuleer = Capsuleer::new(capsuleer_name.to_string(),5,5,5);
        let outpost = Outpost::new(outpost_name.to_string(),capsuleer,outpost_system.to_string(), available_planets, available_arrays)?;
        self.get_outposts_mut().push(outpost);
        self.save_data()?;
        Ok(())
    }

    pub fn load_data(&self) -> Result<Manager, Box<dyn Error>> {
        let manager_dir = self.get_dir();
        let file_path = manager_dir.join("data.bin");
    
        if file_path.exists() {
            let mut file = File::open(&file_path)?;
            let mut manager_data = Vec::new();
            file.read_to_end(&mut manager_data)?;
            Ok(bincode::deserialize(&manager_data)?)
        } else {
            fs::create_dir_all(&manager_dir)?;
            return Err(format!("A manager data does not exist so it was created: {}.", manager_dir.display()).into());
        }
    }

    fn save_data(&self) -> Result<(), Box<dyn Error>> {
        let manager_dir = self.get_dir();
        fs::create_dir_all(&manager_dir)?;
        let file_path = manager_dir.join("data.bin");
        let mut file = File::create(&file_path)?;
        let encoded = bincode::serialize(self)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn delete_outpost(&mut self, outpost_name: &str) -> Result<(), Box<dyn Error>> {
        if !self.get_outposts().iter().any(|o| o.name == outpost_name) {
            return Err(format!("An outpost named {} does not exist for this manager.", outpost_name).into());
        }
        self.get_outposts_mut().retain(|o| o.name != outpost_name);
        self.save_data()?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Capsuleer {
    pub name: String,
    pub planetology: i64,
    pub advanced_planetology: i64,
    pub expert_planetology: i64,
}

impl Capsuleer {
    pub fn new(
        name: String,
        planetology: i64,
        advanced_planetology: i64,
        expert_planetology: i64
    ) -> Self {
        Capsuleer {
            name, 
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
    pub constellation_id: i64,
    pub available_planets: i32,
    pub available_arrays: i32
}

impl Outpost {
    pub fn new(
        name: String,
        capsuleer: Capsuleer,
        system_name: String,
        available_planets: i32,
        available_arrays: i32,
    ) -> Result<Self, Box<dyn Error>> {
        let system_id = find_system(&system_name).unwrap();
        let system = get_system(*system_id).unwrap();

        let outpost = Outpost {
            name: name.clone(),
            capsuleer: capsuleer.clone(),
            system_id: *system_id,
            constellation_id: *system.constellation,
            available_planets: available_planets,
            available_arrays: available_arrays,
        };

        Ok(outpost)
    }
}