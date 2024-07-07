use flate2::write::GzEncoder;
use flate2::Compression;
use pprof::{protos::Message, ProfilerGuardBuilder};
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Deserialize)]
pub struct ProfileParams {
    pub seconds: Option<u64>,
}

pub async fn generate_profile(duration: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let guard = ProfilerGuardBuilder::default()
        .frequency(200)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()?;

    sleep(Duration::from_secs(duration)).await;

    let profile = guard.report().build()?.pprof()?;

    let mut body = Vec::new();
    let mut encoder = GzEncoder::new(&mut body, Compression::default());

    profile.write_to_writer(&mut encoder)?;
    encoder.finish()?;

    Ok(body)
}
