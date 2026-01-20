mod api;
mod app;
mod plugin;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::app::App;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    list_plugins: bool,
    #[arg(short, long)]
    version: bool,
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[clap(subcommand)]
    subcommand: Option<SubCommand>,
    #[arg(allow_hyphen_values = true, num_args = 0..)]
    extra_args: Vec<String>,
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(
        name = "_picker",
        hide = true,
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    Picker,
    #[clap(
        name = "_runner",
        hide = true,
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    Runner,
    #[clap(
        name = "_controller",
        hide = true,
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    Controller,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let app = App::new(cli.config)?;
    if cli.version {
        println!("fzfmenu {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if cli.list_plugins {
        for plugin in &app.plugins {
            println!("{}", plugin);
        }
        return Ok(());
    }
    match cli.subcommand {
        Some(subcommand) => match subcommand {
            SubCommand::Picker => app.run_picker(),
            SubCommand::Runner => app.run_runner(),
            SubCommand::Controller => app.run_controller(),
        },
        None => app.run(cli.extra_args),
    }
}
