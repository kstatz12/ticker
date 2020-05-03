extern crate yaml_rust;

use yaml_rust::{YamlLoader, Yaml};
use std::fs;

pub struct Config{
    pub token: String,
    pub symbols: Vec<String>,
    pub interval: u64
}

fn parse(docs: Vec<Yaml>) -> Config {
    let doc = &docs[0];

    let token = doc["finnhub"]["token"].as_str().expect("Could Not Find Token").to_string();
    let symbols = doc["finnhub"]["symbols"].as_vec().expect("Could Not Load Symbols");
    let interval = doc["finnhub"]["interval"].as_i64().unwrap();

    let mut vec = Vec::<String>::new();
    for s in symbols.iter() {
        vec.push(s.as_str().unwrap().to_string());
    }

    return Config {
        token: token,
        symbols: vec,
        interval: interval as u64
    }
}

impl Config {
    pub fn new(path: &str) -> Config {
        let file_contents = fs::read_to_string(path).expect("Could Not Open Config File");
        let docs = YamlLoader::load_from_str(&file_contents).expect("Could Not Load YAML");
        return parse(docs);
    }
}




