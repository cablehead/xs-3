use serde::{Deserialize, Serialize};
use sled::Db;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Frame {
    pub id: scru128::Scru128Id,
    pub hash: ssri::Integrity,
}

pub struct Store {
    db: Db,
    cache_path: String,
}

impl Store {
    pub fn new(path: &str) -> Store {
        let db = sled::open(Path::new(path).join("index")).unwrap();
        let cache_path = Path::new(path).join("cas").to_str().unwrap().to_string();
        Store { db, cache_path }
    }

    pub fn put(&mut self, content: &[u8]) -> Frame {
        let h = cacache::write_hash_sync(&self.cache_path, content).unwrap();
        let frame = Frame {
            id: scru128::new(),
            hash: h,
        };
        let encoded: Vec<u8> = bincode::serialize(&frame).unwrap();
        self.db.insert(frame.id.to_string(), encoded).unwrap();
        frame
    }
}

fn main() {
    let mut store = Store::new("my_db");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let frame = store.put(line.as_bytes());
        println!("Stored frame: {:?}", frame);
    }

    // Read and print all records from sled
    for record in store.db.iter() {
        let record = record.unwrap();
        let decoded: Frame = bincode::deserialize(&record.1).unwrap();
        println!("{}", serde_json::to_string(&decoded).unwrap());
    }
}
