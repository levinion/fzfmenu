use std::{
    fmt,
    io::{BufRead, BufReader},
    os::unix::process::CommandExt,
    process::Command,
};

use anyhow::{Result, bail};
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
        let arguments = arguments
            .strip_prefix(&self.prefix)
            .expect("invalid arguments");
        let mut child = Command::new("sh")
            .args(["-c", &self.picker.replace("{}", arguments)])
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line?;
            println!("{}{}", self.prefix, line);
        }
        child.wait()?;
        Ok(())
    }

    pub fn run_runner(&self, arguments: &str) -> Result<()> {
        let arguments = arguments
            .strip_prefix(&self.prefix)
            .expect("invalid arguments");
        let err = Command::new("sh")
            .args(["-c", &self.runner.replace("{}", arguments)])
            .exec();
        bail!(err);
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
