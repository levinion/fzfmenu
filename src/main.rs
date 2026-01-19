mod app;
mod plugin;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::app::{App, AppOpt};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    list_plugins: bool,
    #[arg(short, long)]
    version: bool,
    #[arg(short, long)]
    query: Option<String>,
    #[arg(short, long)]
    terminal: Option<String>,
    #[arg(short)]
    options: Option<String>,
    #[arg(long)]
    no_reload: bool,
    #[arg(short)]
    #[clap(short = 'O')]
    fzf_options: Option<String>,
    #[clap(subcommand)]
    subcommand: Option<SubCommand>,
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(
        name = "_picker",
        hide = true,
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    Picker { args: Vec<String> },
    #[clap(
        name = "_runner",
        hide = true,
        trailing_var_arg = true,
        allow_hyphen_values = true
    )]
    Runner { args: Vec<String> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let app = App::new()?;
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
            SubCommand::Picker { args } => app.run_picker(args.join(" ")),
            SubCommand::Runner { args } => app.run_runner(args.join(" ")),
        },
        None => app.run(AppOpt {
            query: cli.query,
            terminal: cli.terminal,
            options: cli.options,
            fzf_options: cli.fzf_options,
            no_reload: cli.no_reload,
        }),
    }
}
