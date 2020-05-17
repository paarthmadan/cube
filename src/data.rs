use super::app::App;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::time::Duration;

const DIR: &str = ".cube";
const FILE: &str = "data.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub times: Vec<Duration>,
}

impl Data {
    fn from(app: App) -> Self {
        Data { times: app.times }
    }
}

pub mod import {
    use super::*;
    use std::io::BufReader;

    type Result<T> = std::result::Result<T, ReadError>;
    pub struct ReadError;

    pub fn from_file() -> Result<Data> {
        let home = home_dir().ok_or(ReadError)?;

        let file = File::open(home.join(DIR).join(FILE)).map_err(|_| ReadError)?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(|_| ReadError)
    }
}

pub mod export {
    use super::*;

    type Result<T> = std::result::Result<T, WriteError>;
    pub struct WriteError;

    pub fn to_file(app: App) -> Result<()> {
        let home = home_dir().ok_or(WriteError)?;
        create_dir_all(home.join(DIR)).map_err(|_| WriteError)?;

        let data = Data::from(app);
        let file = File::create(home.join(DIR).join(FILE)).map_err(|_| WriteError)?;

        serde_json::to_writer(file, &data).map_err(|_| WriteError)
    }
}
