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
    // Bare (non-nullable) Void is the discriminating canary: prune above travels the
    // engine's Nullable[Void] path, which was JSON null even on pre-correction
    // engines, but a bare core.Void result was Go's default `{}` until the engine
    // carried its explicit null marshaller. Strict decoding refuses `{}`, so this
    // line fails against any engine whose core predates that correction. Workspace
    // export is that bare path and no-ops cleanly on an empty overlay; the verify
    // runner initializes the local Git workspace it requires.
    client.query().current_workspace().export().await?;
    client.close().await?;
    println!("{version}");
    Ok(())
}
