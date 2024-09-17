use color_eyre::Result;
use log::info;

fn main() -> Result<()> {
    env_logger::init();
    info!("Running");
    Ok(())
}
