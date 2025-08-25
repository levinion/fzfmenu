mod app;
mod plugin;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::app::App;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    list_plugins: bool,
    #[clap(subcommand)]
    subcommand: Option<SubCommand>,
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(hide = true, trailing_var_arg = true, allow_hyphen_values = true)]
    Picker { args: Vec<String> },
    #[clap(hide = true, trailing_var_arg = true, allow_hyphen_values = true)]
    Runner { args: Vec<String> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let app = App::new()?;
    if cli.list_plugins {
        for plugin in &app.plugins {
            println!("{}", plugin);
        }
        return Ok(());
    }
    match cli.subcommand {
        Some(subcommand) => match subcommand {
            SubCommand::Picker { args } => app.run_picker(args.join(" ")),
            SubCommand::Runner { args } => app.run_runner(args.join(" ")),
        },

        None => app.run(),
    }
}
