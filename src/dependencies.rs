use algonaut::algod::{v2::Algod, AlgodBuilder};
use anyhow::Result;

use crate::provider::Provider;

pub struct Dependencies {
    pub provider: Provider,
}

pub fn init_dependencies() -> Result<Dependencies> {
    Ok(Dependencies {
        provider: provider(algod()?),
    })
}

fn algod() -> Result<Algod> {
    let algod = AlgodBuilder::new()
        // If running from a device and algod is on local computer, replace localhost with computer's IP
        // You might have to configure algod to run explicitly on that IP (see node's config.json/algod.net)
        .bind("http://localhost:53630")
        .auth("44d70009a00561fe340b2584a9f2adc6fec6a16322554d44f56bef9e682844b9")
        .build_v2()?;
    Ok(algod)
}

fn provider(algod: Algod) -> Provider {
    Provider::new(algod)
}
