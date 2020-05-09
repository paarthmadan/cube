use std::time::Duration;
use serde::{Serialize, Deserialize};
use std::fs::File;
use super::app::App;


#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub times: Vec<Duration>,
}

impl Data {
    fn from_app(app: App) -> Self {
        Data { times: app.times }
    }
}

pub mod import {
    use super::*;
    use std::io::BufReader;

    type Result<T> = std::result::Result<T, ReadError>;
    pub struct ReadError;

    pub fn from_file() -> Result<Data> {
        let file = File::open("data.json").map_err(|_| ReadError)?;
        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader).map_err(|_| ReadError);
        data
    }
}

pub mod export {
    use super::*;

    type Result<T> = std::result::Result<T, WriteError>;
    pub struct WriteError;

    pub fn to_file(app: App) -> Result<()> {
        let data = Data::from_app(app);
        let file = File::create("data.json").map_err(|_| WriteError)?;

        serde_json::to_writer(&file, &data).map_err(|_| WriteError)
    }
}


