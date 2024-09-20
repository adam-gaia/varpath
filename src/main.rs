use clap::{Parser, Subcommand};
use color_eyre::Result;
use log::info;

#[derive(Debug, Subcommand)]
enum Commands {
    Todo,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    // TODO
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let args = Cli::parse();
    match &args.command {
        Some(command) => {
            match command {
                Commands::Todo => {
                    // TODO
                    info!("TODO: Command::Todo");
                }
            }
        }
        None => {
            // TODO
            info!("TODO: no subcommannd provided");
        }
    }

    Ok(())
}
