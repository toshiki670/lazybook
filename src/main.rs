use anyhow::Result;

mod asset_management;
mod shared;

fn main() -> Result<()> {
    // Initialize shared infrastructure
    shared::logging::init()?;

    // Launch Asset Management context
    asset_management::presentation::run()?;

    // Future: Context selection menu
    Ok(())
}
