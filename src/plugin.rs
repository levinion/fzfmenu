use std::{fmt, io::BufRead, process::Command};

use anyhow::Result;
use colored::Colorize;

#[derive(serde::Deserialize)]
pub struct Plugin {
    pub name: String,
    pub description: Option<String>,
    pub prefix: String,
    pub picker: String,
    pub runner: String,
}

impl Plugin {
    pub fn run_picker(&self, arguments: &str) -> Result<()> {
        if let Some(arguments) = arguments.strip_prefix(&self.prefix) {
            let output = Command::new("sh")
                .args(["-c", &self.picker.replace("{}", arguments)])
                .output()?;
            if output.status.success() {
                let lines = output.stdout.lines().collect::<Vec<_>>();
                for line in lines {
                    let line = line?;
                    println!("{}{}", self.prefix, line);
                }
            }
        }
        Ok(())
    }

    pub fn run_runner(&self, arguments: &str) -> Result<()> {
        if let Some(arguments) = arguments.strip_prefix(&self.prefix) {
            Command::new("sh")
                .args(["-c", &self.runner.replace("{}", arguments)])
                .spawn()?;
        }
        Ok(())
    }
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_line = format!(
            "{:<20} {:<20}",
            self.name.bold().blue(),
            format!("`{}`", self.prefix).green()
        );
        if let Some(desc) = &self.description {
            write!(f, "{}{}", name_line, desc.truecolor(150, 150, 150))
        } else {
            write!(f, "{}", name_line)
        }
    }
}
