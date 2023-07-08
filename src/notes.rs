
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

/*
Then, you can use the `Mime` type in your struct:

```rust
use ssri::Integrity;
use std::collections::HashMap;
use mime::Mime;

pub struct MyStruct {
    pub integrity_map: HashMap<String, Integrity>,
    pub mime_type: Mime,
}
```

You can then create `Mime` instances using the `mime::TEXT_PLAIN` and `mime::IMAGE_PNG` constants, or by parsing a string:

```rust
let text_plain: Mime = "text/plain".parse().unwrap();
let image_png: Mime = "image/png".parse().unwrap();
```
*/

/*
 *

 If you have more than one struct type to store, you can use an enum to wrap your different struct types. This way, you can serialize and deserialize the enum, and thus indirectly your structs. Here's an example:

First, define your structs and the enum:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct1 {
    field1: u32,
    field2: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct2 {
    field3: f64,
    field4: bool,
}

#[derive(Serialize, Deserialize, Debug)]
enum MyEnum {
    Struct1(MyStruct1),
    Struct2(MyStruct2),
}
```

Now you can serialize your structs to bytes, store them in Sled, then retrieve them and deserialize them back to the structs:

```rust
use sled::{Db, Config};
use bincode::{serialize, deserialize};

fn main() -> sled::Result<()> {
    let config = Config::new().temporary(true);
    let db: Db = config.open()?;

    let my_struct1 = MyStruct1 {
        field1: 42,
        field2: "Hello, world!".to_string(),
    };

    let my_struct2 = MyStruct2 {
        field3: 3.14,
        field4: true,
    };

    // Serialize the structs to bytes
    let encoded1: Vec<u8> = serialize(&MyEnum::Struct1(my_struct1)).unwrap();
    let encoded2: Vec<u8> = serialize(&MyEnum::Struct2(my_struct2)).unwrap();

    // Store the bytes in Sled
    db.insert("key1", encoded1)?;
    db.insert("key2", encoded2)?;

    // Retrieve the bytes
    let maybe_bytes1 = db.get("key1")?;
    let maybe_bytes2 = db.get("key2")?;

    if let Some(bytes) = maybe_bytes1 {
        // Deserialize the bytes back to the struct
        let decoded: MyEnum = deserialize(&bytes).unwrap();
        println!("{:?}", decoded);
    }

    if let Some(bytes) = maybe_bytes2 {
        // Deserialize the bytes back to the struct
        let decoded: MyEnum = deserialize(&bytes).unwrap();
        println!("{:?}", decoded);
    }

    Ok(())
}
*/

