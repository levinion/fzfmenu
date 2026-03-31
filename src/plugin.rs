use std::{
    fmt,
    io::{BufRead, BufReader},
    os::unix::process::CommandExt,
    process::Command,
};

use anyhow::Result;
use colored::Colorize;
use itertools::Itertools;

#[derive(serde::Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
    pub description: Option<String>,
    pub prefix: String,
    pub picker: String,
    pub runner: Option<String>,
    pub dynamic: Option<bool>,    // default to false
    pub background: Option<bool>, // default to false
    #[serde(default)]
    pub on_enter: Vec<String>,
    #[serde(default)]
    pub on_reload: Vec<String>,
    #[serde(default)]
    pub on_leave: Vec<String>,
    pub on_enter_script: Option<String>,
    pub on_reload_script: Option<String>,
    pub on_leave_script: Option<String>,
    // helper
    pub preview: Option<String>,
    pub multiple: Option<bool>,
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
        let mut cmd = Command::new("sh");
        cmd.env("FZFMENU_OUTPUT", arguments).args([
            "-c",
            &self.runner.as_ref().unwrap().replace("{}", arguments),
        ]);
        if self.background.unwrap_or(false) {
            cmd.process_group(0);
        }
        cmd.spawn()?.wait()?;
        Ok(())
    }

    pub fn on_enter(&self, query: impl AsRef<str>) -> Result<Vec<String>> {
        let mut on_enter = if let Some(script) = self.on_enter_script.clone() {
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
            reader.lines().map_while(Result::ok).collect_vec()
        } else {
            vec![]
        };
        on_enter.extend(self.on_enter.clone());
        // helper
        if let Some(preview) = self.preview.clone() {
            on_enter.push(format!("change-preview({})", preview));
            on_enter.push("show-preview".to_owned());
        }
        if self.multiple.unwrap_or(false) {
            on_enter.push("change-multi".to_owned());
        }
        Ok(on_enter)
    }

    pub fn on_reload(&self, query: impl AsRef<str>) -> Result<Vec<String>> {
        let mut on_reload = if let Some(script) = self.on_reload_script.clone() {
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
            reader.lines().map_while(Result::ok).collect_vec()
        } else {
            vec![]
        };
        on_reload.extend(self.on_reload.clone());
        Ok(on_reload)
    }

    pub fn on_leave(&self, query: impl AsRef<str>) -> Result<Vec<String>> {
        let mut on_leave = if let Some(script) = self.on_leave_script.clone() {
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
            reader.lines().map_while(Result::ok).collect_vec()
        } else {
            vec![]
        };
        on_leave.extend(self.on_leave.clone());
        // helper
        if self.preview.is_some() {
            on_leave.push("hide-preview".to_owned());
        }
        if self.multiple.unwrap_or(false) {
            on_leave.push("change-multi(0)".to_owned());
        }
        Ok(on_leave)
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
