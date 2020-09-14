use serde::{Serialize, Deserialize};
use serde_json::*;
use serde_derive::*;

use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct TemplateMap {
    pub tiles: String,
    pub width: i32,
    pub height: i32,
}

pub fn load_from_file() -> TemplateMap {

    let json_file_path = Path::new("resources/maps.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized: TemplateMap = serde_json::from_reader(json_file).expect("error while reading json");

    deserialized
}


pub fn serialzie_test() {
    let template = TemplateMap {
        height: 9,
        width: 9,
        tiles: String::from("##########.......##.x...x.##.......##.o...o.##.......##.......##...@...##########")
    };

    let serialized = serde_json::to_string(&template).unwrap();

    //Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);


}