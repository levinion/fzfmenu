use std::{fs::read_to_string, os::unix::process::CommandExt, path::PathBuf, process::Command};

use anyhow::{Result, anyhow, bail};
use shell_quote::{Bash, QuoteRefExt};

use crate::plugin::Plugin;

#[derive(serde::Deserialize)]
pub struct App {
    pub terminal: String,
    pub arguments: Option<Vec<String>>,
    pub fzf_arguments: Option<Vec<String>>,
    pub plugins: Vec<Plugin>,
}

pub struct AppOpt {
    pub query: Option<String>,
    pub terminal: Option<String>,
    pub options: Option<String>,
    pub fzf_options: Option<String>,
    pub no_reload: bool,
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

    pub fn run(self, opt: AppOpt) -> Result<()> {
        let terminal = opt.terminal.unwrap_or(self.terminal);
        let mut arguments = match opt.options {
            Some(options) => options
                .split_whitespace()
                .map(|opt| opt.quoted(Bash))
                .collect::<Vec<String>>(),
            None => self.arguments.unwrap_or_default(),
        };
        let fzf_arguments = match opt.fzf_options {
            Some(options) => options
                .split_whitespace()
                .map(|opt| opt.quoted(Bash))
                .collect::<Vec<String>>(),
            None => self.fzf_arguments.unwrap_or_default(),
        }
        .join(" ");
        let exe = std::env::current_exe()?.to_string_lossy().to_string();
        let query = match opt.query {
            Some(query) => {
                let query: String = query.quoted(Bash);
                "--query ".to_owned() + &query
            }
            None => "".to_owned(),
        };
        let action = if opt.no_reload {
            "start"
        } else {
            "start,change"
        };
        let fzf_cmd = format!(
            "fzf {0} {1} --bind '{3}:reload:{2} picker {{q}}' --bind 'enter:execute-silent(nohup {2} runner {{}} >/dev/null 2>&1)+accept'",
            &fzf_arguments, query, &exe, action
        );
        arguments.extend(
            ["-e", "sh", "-c", &fzf_cmd]
                .into_iter()
                .map(|str| str.to_string()),
        );
        let err = Command::new(terminal).args(arguments).exec();
        bail!(err);
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
