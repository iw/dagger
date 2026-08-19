use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = dagger_sdk::connect().await?;
    let version = client.query().version().await?;
    if version.trim().is_empty() {
        return Err("complete engine returned an empty version".into());
    }
    // The SDK's connection validator accepts conformance engines built from the
    // unmodified upstream tree (bare-commit metadata), so the release gate asserts
    // the fork iteration itself: a complete engine missing `+rust.<N>.` was not
    // built from this fork's workspace and must not ship.
    let expected_fork = std::env::var("RUST_SDK_EXPECTED_ENGINE_FORK")
        .map_err(|_| "RUST_SDK_EXPECTED_ENGINE_FORK is not set")?;
    let marker = format!("+{expected_fork}.");
    if !version.contains(&marker) {
        return Err(format!(
            "complete engine version {version:?} does not carry fork metadata {marker:?}"
        )
        .into());
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
    // line fails against any engine whose core predates that correction.
    client
        .container()
        .with_new_file("void canary", "/rust-sdk-void-canary")
        .export_image("rust-sdk-void-canary:verify")
        .await?;
    client.close().await?;
    println!("{version}");
    Ok(())
}
