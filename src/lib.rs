use algonaut::core::Address;
use anyhow::{anyhow, Result};
use dependencies::{init_dependencies, Dependencies};
use once_cell::sync::OnceCell;
use provider::AccountViewData;
use tokio::runtime;

#[cfg(target_os = "ios")]
mod ffi_ios;

mod dependencies;
mod provider;

pub static DEPS: OnceCell<Dependencies> = OnceCell::new();

pub fn bootstrap() -> Result<()> {
    DEPS.set(init_dependencies()?)
        .map_err(|_| anyhow!("Couldn't set dependencies"))
}

fn dependencies() -> Result<&'static Dependencies> {
    DEPS.get().ok_or(anyhow!(
        "Dependencies not initialized. Did the app call bootstrap?"
    ))
}

pub fn get_infos(address: &Address) -> Result<AccountViewData> {
    let deps = dependencies()?;

    let rt = runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    // Convert async to blocking, for simpler FFI/JNI.
    // Concurrency is responsibility of apps.
    rt.block_on(deps.provider.get_infos(address))
}
