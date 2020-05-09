use std::time::Duration;
use serde::{Serialize, Deserialize};
use super::App;
use std::fs::File;
use std::io::BufReader;
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub times: Vec<Duration>,
}

impl Data {
    fn from_app(app: App) -> Self {
        Data { times: app.times }
    }
}

pub fn read_from_file() -> Result<Data, Error> {
    let file = File::open("data.json").unwrap();
    let reader = BufReader::new(file);

    let data = serde_json::from_reader(reader);
    data
}

pub fn write_to_file(app: App) -> Result<(), Error> {
    let data = Data::from_app(app);
    serde_json::to_writer(&File::create("data.json").unwrap(), &data)
}
