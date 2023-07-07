mod xs_lib;

fn main() -> cacache::Result<()> {
    let path = "/Users/andy/Library/Application Support/stream.cross.stacks/stream";
    let env = xs_lib::store_open(&path)?;
    let frames = xs_lib::store_cat(&env, None)?;

    println!("FRAMES: {:?}", frames);

    let h = cacache::write_hash_sync("./my-cache", b"my-data")?;
    println!("hash: {}", &h);
    let data = cacache::read_hash_sync("./my-cache", &h)?;
    assert_eq!(data, b"my-data");
    Ok(())
}
