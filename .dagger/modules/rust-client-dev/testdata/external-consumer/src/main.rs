use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = dagger_sdk::connect().await?;
    let version = client.query().version().await?;
    if version.trim().is_empty() {
        return Err("complete engine returned an empty version".into());
    }
    // One Void-typed field per verification: the wire encoding of a custom
    // scalar is invisible in schema bytes, so engine/SDK agreement on it is
    // provable only here, against the composed engine. Strict null decoding
    // makes any deviation fail this step.
    client.query().engine().local_cache().prune().await?;
    client.close().await?;
    println!("{version}");
    Ok(())
}
