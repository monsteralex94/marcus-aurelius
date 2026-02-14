use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

use crate::level::boss_data::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub num_stages: usize,
    pub dialog: Vec<Vec<String>>,
    pub access_next: Vec<bool>,
    pub boss_data: Vec<Option<BossData>>,
}

impl LevelData {
    pub fn new(path: &str) -> Result<LevelData, Box<dyn Error>> {
        let data = fs::read_to_string(path)?;
        let level: LevelData = serde_json::from_str(&data)?;
        Ok(level)
    }
}

#[derive(Debug)]
pub struct LevelGroupData {
    pub directory: String,
    pub num_levels: usize,
    pub levels: Vec<LevelData>,
}

impl LevelGroupData {
    pub fn new(directory: &str) -> Result<LevelGroupData, Box<dyn Error>> {
        let data = fs::read_to_string(format!("{}/levels.json", directory))?;
        let level_names: Vec<String> = serde_json::from_str(&data)?;
        let mut levels: Vec<LevelData> = Vec::new();

        for level_name in &level_names {
            levels.push(LevelData::new(
                format!("{}/{}/level.json", directory, level_name).as_str(),
            )?);
        }

        Ok(LevelGroupData {
            directory: String::from(directory),
            num_levels: levels.len(),
            levels: levels,
        })
    }
}
