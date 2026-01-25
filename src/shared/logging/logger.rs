// Logger initialization

use anyhow::Result;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initialize logging with privacy-aware configuration
pub fn init() -> Result<()> {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .try_init()?;

    log::info!("Logging initialized");
    Ok(())
}
