use std::{
    env,
    fs::{File, remove_file},
    io::{BufRead, BufReader},
    os::unix::process::CommandExt,
    path::PathBuf,
    process::Command,
};

use anyhow::{Context, Result, bail};
use vipera::Configuration;

use crate::{
    api::{call_actions, change_border_label, reload},
    plugin::Plugin,
};

#[derive(serde::Deserialize)]
pub struct App {
    #[serde(default)]
    pub plugins: Vec<Plugin>,
}

impl vipera::Configuration for App {
    fn vipera() -> Result<vipera::Vipera> {
        let vipera = vipera::Vipera::new();
        if let Ok(path) = std::env::var("FZFMENU_CONFIG_PATH") {
            let path = PathBuf::from(path);
            let vipera = vipera
                .set_config_name(
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .context(format!("Invalid config path: {:?}", path))?,
                )
                .add_config_path(
                    path.parent()
                        .context(format!("Invalid config path: {:?}", path))?,
                );
            Ok(vipera)
        } else {
            let vipera = vipera
                .set_config_name("config.toml")
                .add_config_path("$HOME/.config/fzfmenu")
                .add_config_path("/etc/fzfmenu");
            Ok(vipera)
        }
    }
}

impl App {
    pub fn new(config: Option<PathBuf>) -> Result<Self> {
        if let Some(path) = config {
            unsafe {
                std::env::set_var("FZFMENU_CONFIG_PATH", path.to_str().unwrap());
            }
        }
        Self::read_in_config()
    }

    pub fn run(self, mut args: Vec<String>) -> Result<()> {
        let exe = std::env::current_exe()?.to_str().unwrap().to_owned();
        args.extend(
            [
                "--bind",
                &format!("start,change:transform({} _controller)", exe),
                "--bind",
                &format!("enter:become(FZF_SELECTED={{+f}} {} _runner)", exe),
            ]
            .map(|s| s.to_owned()),
        );
        let err = Command::new("fzf").args(args).exec();
        bail!(err);
    }

    pub fn run_picker(self) -> Result<()> {
        let query = &env::var("FZF_QUERY").unwrap();
        if let Some(plugin) = self.active_plugin(query) {
            plugin.run_picker(query)?;
        }
        Ok(())
    }

    pub fn run_runner(self) -> Result<()> {
        let query = &env::var("FZF_QUERY").unwrap();
        if let Some(plugin) = self.active_plugin(query) {
            let tempfile = PathBuf::from(env::var("FZF_SELECTED").unwrap());
            let file = File::open(&tempfile)?;
            let reader = BufReader::new(file);
            let selected_items = reader.lines().map_while(Result::ok);
            for selected in selected_items {
                plugin.run_runner(&selected)?;
            }
            // try cleaning the tempfile
            let _ = remove_file(tempfile);
        }
        Ok(())
    }

    pub fn run_controller(self) -> Result<()> {
        let query = &env::var("FZF_QUERY").unwrap();
        if let Some(plugin) = self.active_plugin(query) {
            let mut actions = vec![];
            match self.last_plugin() {
                Some(last_plugin) => {
                    if plugin.name != last_plugin.name {
                        actions.extend(last_plugin.on_leave(query)?);
                        actions.extend(plugin.on_enter(query)?);
                    }
                    if plugin.dynamic.unwrap_or(false) || plugin.name != last_plugin.name {
                        actions.extend(plugin.on_reload(query)?);
                        actions.push(change_border_label(format!(" {} ", plugin.name)));
                        actions.push(reload()?);
                    }
                }
                None => {
                    actions.extend(plugin.on_enter(query)?);
                    actions.extend(plugin.on_reload(query)?);
                    actions.push(change_border_label(format!(" {} ", plugin.name)));
                    actions.push(reload()?);
                }
            }
            if !actions.is_empty() {
                call_actions(actions);
            }
        }
        Ok(())
    }

    fn active_plugin(&self, query: impl AsRef<str>) -> Option<&Plugin> {
        self.plugins
            .iter()
            .filter(|plugin| query.as_ref().starts_with(&plugin.prefix))
            .max_by_key(|plugin| plugin.prefix.len())
    }

    fn last_plugin(&self) -> Option<&Plugin> {
        let border_label = env::var("FZF_BORDER_LABEL").unwrap();
        let last_plugin_name = border_label.trim();
        self.plugins.iter().find(|s| s.name == last_plugin_name)
    }
}
