use base64::Engine;

mod xs_lib;

/*
## Storage

content.add: id, hash  # need to be able give a type

content_type is:

- Text
- Link
- Stack
- Image

mime_type is:

- text/plain
- image/png

Potentially source

stack.add: id, id
stack.del: id, id
edit.add: id, id

xs_lib::store_put(&env, Some("clipboard".into()), None, line.clone())
xs_lib::store_put(&env, Some("stack".into()), None, data).unwrap();
xs_lib::store_put(&env, Some("stack".into()), Some("delete".into()), data).unwrap();
xs_lib::store_put(&env, Some("item".into()), None, item).unwrap();
xs_lib::store_put(&env, Some("link".into()), None, data).unwrap();
*/


fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                } else if types.contains_key("public.png") {
                    let content = types["public.png"].as_str().unwrap().as_bytes();
                }
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
