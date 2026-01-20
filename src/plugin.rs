use std::{
    fmt,
    io::{BufRead, BufReader},
    os::unix::process::CommandExt,
    process::Command,
};

use anyhow::{Result, bail};
use colored::Colorize;
use itertools::Itertools;

#[derive(serde::Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
    pub description: Option<String>,
    pub prefix: String,
    pub picker: String,
    pub runner: String,
    pub dynamic: Option<bool>, // default to false
    #[serde(default)]
    pub on_enter: Vec<String>,
    #[serde(default)]
    pub on_reload: Vec<String>,
    #[serde(default)]
    pub on_leave: Vec<String>,
    pub on_enter_script: Option<String>,
    pub on_reload_script: Option<String>,
    pub on_leave_script: Option<String>,
}

impl Plugin {
    pub fn run_picker(&self, query: impl AsRef<str>) -> Result<()> {
        let arguments = query
            .as_ref()
            .strip_prefix(&self.prefix)
            .expect("invalid arguments");
        let mut child = Command::new("sh")
            .env("FZFMENU_INPUT", arguments)
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

    pub fn run_runner(&self, selected: impl AsRef<str>) -> Result<()> {
        let arguments = selected.as_ref().strip_prefix(&self.prefix).unwrap();
        let err = Command::new("sh")
            .env("FZFMENU_OUTPUT", arguments)
            .args(["-c", &self.runner.replace("{}", arguments)])
            .exec();
        bail!(err);
    }

    pub fn on_enter(&self, query: impl AsRef<str>) -> Result<Vec<String>> {
        if let Some(script) = self.on_enter_script.clone() {
            let arguments = query
                .as_ref()
                .strip_prefix(&self.prefix)
                .expect("invalid arguments");
            let mut child = Command::new("sh")
                .env("FZFMENU_INPUT", arguments)
                .args(["-c", &script.replace("{}", arguments)])
                .stdout(std::process::Stdio::piped())
                .spawn()?;
            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);
            return Ok(reader.lines().map_while(Result::ok).collect_vec());
        }
        Ok(self.on_enter.clone())
    }

    pub fn on_reload(&self, query: impl AsRef<str>) -> Result<Vec<String>> {
        if let Some(script) = self.on_reload_script.clone() {
            let arguments = query
                .as_ref()
                .strip_prefix(&self.prefix)
                .expect("invalid arguments");
            let mut child = Command::new("sh")
                .env("FZFMENU_INPUT", arguments)
                .args(["-c", &script.replace("{}", arguments)])
                .stdout(std::process::Stdio::piped())
                .spawn()?;
            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);
            return Ok(reader.lines().map_while(Result::ok).collect_vec());
        }
        Ok(self.on_reload.clone())
    }

    pub fn on_leave(&self, query: impl AsRef<str>) -> Result<Vec<String>> {
        if let Some(script) = self.on_leave_script.clone() {
            let arguments = query
                .as_ref()
                .strip_prefix(&self.prefix)
                .expect("invalid arguments");
            let mut child = Command::new("sh")
                .env("FZFMENU_INPUT", arguments)
                .args(["-c", &script.replace("{}", arguments)])
                .stdout(std::process::Stdio::piped())
                .spawn()?;
            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);
            return Ok(reader.lines().map_while(Result::ok).collect_vec());
        }
        Ok(self.on_leave.clone())
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
