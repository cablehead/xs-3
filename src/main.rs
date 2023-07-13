use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::io::{self, Read};
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
        let cache_path = Path::new(path)
            .join("cas")
            .into_os_string()
            .into_string()
            .unwrap();
        Store { db, cache_path }
    }

    pub fn put(&mut self, content: &[u8]) -> Frame {
        let h = cacache::write_hash_sync(&self.cache_path, content).unwrap();
        let frame = Frame {
            id: scru128::new(),
            hash: h,
        };
        let encoded: Vec<u8> = bincode::serialize(&frame).unwrap();
        self.db.insert(frame.id.to_bytes(), encoded).unwrap();
        frame
    }
}

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(value_parser)]
    path: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "put")]
    Put(PutCommand),
    List,
    Cat(CatCommand),
}

#[derive(Parser, Debug)]
struct PutCommand {
    #[clap(short, long)]
    follow: bool,
}

#[derive(Parser, Debug)]
struct CatCommand {
    hash: String,
}

fn run_app(args: Args) {
    let mut store = Store::new(&args.path);

    match &args.command {
        Commands::Put(cmd) => {
            let stdin = io::stdin();
            let mut stdin = stdin.lock();
            let iterator: Box<dyn Iterator<Item = Vec<u8>>> = if cmd.follow {
                Box::new(stdin.bytes().map(|byte| vec![byte.unwrap()]))
            } else {
                let mut content = Vec::new();
                stdin.read_to_end(&mut content).unwrap();
                Box::new(std::iter::once(content))
            };

            for content in iterator {
                let frame = store.put(&content);
                println!("{}", serde_json::to_string(&frame).unwrap());
            }
        }
        Commands::List => {
            // Read and print all records from sled
            for record in store.db.iter() {
                let record = record.unwrap();
                let decoded: Frame = bincode::deserialize(&record.1).unwrap();
                println!("{}", serde_json::to_string(&decoded).unwrap());
            }
        }
        Commands::Cat(cmd) => {
            let hash: ssri::Integrity = cmd.hash.parse().unwrap();
            match cacache::read_hash_sync(&store.cache_path, &hash) {
                Ok(data) => print!("{}", String::from_utf8(data).unwrap()),
                Err(err) => eprintln!("Error reading file: {}", err),
            }
        }
    }
}

fn main() {
    let params = Args::parse();
    run_app(params);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_put_binary_data() {
        let data = vec![0u8, 1, 2, 3, 4, 5];
        let temp_dir = TempDir::new().unwrap();
        let mut store = Store::new(temp_dir.path().to_str().unwrap());
        let frame = store.put(&data);
        let read_data = cacache::read_hash_sync(&store.cache_path, &frame.hash).unwrap();
        assert_eq!(data, read_data);
    }

    #[test]
    fn test_put_string() {
        let data = "Hello, world!".as_bytes().to_vec();
        let temp_dir = TempDir::new().unwrap();
        let mut store = Store::new(temp_dir.path().to_str().unwrap());
        let frame = store.put(&data);
        let read_data = cacache::read_hash_sync(&store.cache_path, &frame.hash).unwrap();
        assert_eq!(data, read_data);
    }
}
