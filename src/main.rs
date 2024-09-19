use color_eyre::Result;
use log::info;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    info!("Running");
    Ok(())
}
