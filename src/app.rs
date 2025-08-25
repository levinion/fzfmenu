use std::{fs::read_to_string, path::PathBuf, process::Command};

use anyhow::{Result, anyhow};

use crate::plugin::Plugin;

#[derive(serde::Deserialize)]
pub struct App {
    pub terminal: String,
    pub arguments: Option<Vec<String>>,
    pub plugins: Vec<Plugin>,
}

impl App {
    pub fn new() -> Result<Self> {
        let path = {
            let mut result = None;
            if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
                let path = PathBuf::from(xdg_config_home).join("fzfmenu/config.toml");
                if path.is_file() {
                    result = Some(path);
                }
            }
            if let Ok(home) = std::env::var("HOME") {
                let path = PathBuf::from(home).join(".config/fzfmenu/config.toml");
                if path.is_file() {
                    result = Some(path);
                }
            }
            result
        };
        match path {
            Some(path) => {
                let content = read_to_string(path)?;
                let app: App = toml::from_str(&content)?;
                Ok(app)
            }
            None => Err(anyhow!("Config file not found")),
        }
    }

    pub fn run(self) -> Result<()> {
        let mut arguments = self.arguments.unwrap_or_default();
        arguments.extend(
            [
                "-e",
                "sh",
                "-c",
                "fzf --bind 'start,change:reload:fzfmenu picker {q}' --bind 'enter:execute(setsid fzfmenu runner {} > /dev/null 2>&1)+abort'",
            ]
            .into_iter()
            .map(|str| str.to_string()),
        );
        Command::new(self.terminal)
            .args(arguments)
            .spawn()?
            .wait()?;
        Ok(())
    }

    pub fn run_picker(self, arguments: String) -> Result<()> {
        let plugins = self
            .plugins
            .into_iter()
            .filter(|plugin| arguments.starts_with(&plugin.prefix))
            .max_by_key(|plugin| plugin.prefix.len());
        if let Some(plugin) = plugins {
            plugin.run_picker(&arguments)?;
        }
        Ok(())
    }

    pub fn run_runner(self, arguments: String) -> Result<()> {
        let plugins = self
            .plugins
            .into_iter()
            .filter(|plugin| arguments.starts_with(&plugin.prefix))
            .max_by_key(|plugin| plugin.prefix.len());
        if let Some(plugin) = plugins {
            plugin.run_runner(&arguments)?;
        }
        Ok(())
    }
}
