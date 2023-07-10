use serde::{Deserialize, Serialize};
use sled::Db;
use std::io::{self, BufRead};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Frame {
    pub id: scru128::Scru128Id,
    pub hash: ssri::Integrity,
}

fn main() {
    let db: Db = sled::open("my_db").unwrap();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        // Store the content from stdin as-is in cacache
        let h = cacache::write_hash_sync("./my-cache", line.as_bytes()).unwrap();
        println!("hash: {}", &h);

        // Store a single corresponding entry in sled
        let frame = Frame {
            id: scru128::new(),
            hash: h,
        };
        let encoded: Vec<u8> = bincode::serialize(&frame).unwrap();
        db.insert(frame.id.to_string(), encoded).unwrap();
    }

    // Read and print all records from sled
    for record in db.iter() {
        let record = record.unwrap();
        let decoded: Frame = bincode::deserialize(&record.1).unwrap();
        println!("{:?}", decoded);
    }
}
