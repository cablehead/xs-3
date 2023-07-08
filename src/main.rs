use base64::Engine;

mod xs_lib;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MyStruct1 {
    field1: u32,
    field2: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct MyStruct2 {
    field3: f64,
    field4: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum MyEnum {
    Struct1(MyStruct1),
    Struct2(MyStruct2),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if false {
        let db: sled::Db = sled::open("my_db").unwrap();
        for record in db.iter() {
            let record = record.unwrap();
            let decoded: MyEnum = bincode::deserialize(&record.1).unwrap();
            println!("{:?}", decoded);
        }
        return Ok(());
    }

    let db: sled::Db = sled::open("my_db").unwrap();
    let path =
        std::path::Path::new("/Users/andy/Library/Application Support/stream.cross.stacks/stream");
    let env = xs_lib::store_open(&path)?;
    let frames = xs_lib::store_cat(&env, None)?;

    for frame in &frames {
        println!("FRAME: {:?}", frame.topic);

        match &frame.topic {
            Some(topic) if topic == "clipboard" => {
                let clipped: serde_json::Value = serde_json::from_str(&frame.data)?;
                let types = clipped["types"].as_object().unwrap();
                let source = clipped["source"].as_str().unwrap();
                println!("{}", source);

                if types.contains_key("public.utf8-plain-text") {
                    let content = types["public.utf8-plain-text"].as_str().unwrap();
                    let bytes = base64::engine::general_purpose::STANDARD.decode(content)?;

                    let h = cacache::write_hash_sync("./my-cache", bytes)?;
                    println!("hash: {}", &h);

                    let my_struct1 = MyStruct1 {
                        field1: 42,
                        field2: "Hello, world!".to_string(),
                    };
                    let encoded1: Vec<u8> =
                        bincode::serialize(&MyEnum::Struct1(my_struct1)).unwrap();
                    db.insert("key1", encoded1)?;
                }
                /*
                     else if types.contains_key("public.png") {
                     let content = types["public.png"].as_str().unwrap().as_bytes();
                 }
                */
            }

            _ => (),
        }
    }

    let h = cacache::write_hash_sync("./my-cache", b"my-data")?;
    println!("hash: {}", &h);
    let data = cacache::read_hash_sync("./my-cache", &h)?;
    assert_eq!(data, b"my-data");
    Ok(())
}
