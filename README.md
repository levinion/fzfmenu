# fzfmenu

fzfmenu is an application launcher inspired by [using_fzf_as_a_dmenurofi_replacement](https://www.reddit.com/r/commandline/comments/jatyek/using_fzf_as_a_dmenurofi_replacement/). It harnesses the power of fzf’s fuzzy search to offer a unified tool. It's written in rust and pluggable. 

![](/assets/fzfmenu.png)

## Dependencies

- **terminal** (any terminal emulator supports `-e` flag to run command)
- **fzf** (the core dependency)

## Build

To build, make sure `cargo` is in your path.

```shell
git clone https://github.com/levinion/fzfmenu
cd fzfmenu
cargo install --path .
```

There's also an AUR package for ArchLinux users:

```
$AUR_HELPER -S fzfmenu
```

## Configuration

To keep dependencies minimal, `fzfmenu` does not ship with any default configurations or plugins. Therefore, it requires a custom setup before it can be used.

`fzfmenu` is configured using a toml file, typically located at ~/.config/fzfmenu/config.toml. This file defines needed settings and a list of plugins.

Here is a heavily commented example to help you get started:

```toml
#
# Fzfmenu Configuration File
#
# This file is used to define fzfmenu's main program settings and plugin list.
#

# --- Main Settings ---

# `terminal` (string): 
# Specifies the name or path of a terminal emulator. This program will be used
# to launch the fzfmenu interface.
# Examples: "foot", "kitty", "alacritty", "gnome-terminal", etc.
# Ensure that the terminal supports the `-e` option to run a command.
terminal = "foot"

# `arguments` (array of strings):
# Provides additional arguments to be passed to the terminal emulator.
# For example, the `-a` argument is often used to set the application's app_id.
# Other terminal emulator, like `alacritty`, may prefer a `--class` flag to set the window's class.
# Window properties like app_id or class, are useful in tiling window managers for changing a window's default layout and size.
arguments = ["-a", "fzfmenu"]

# `fzf_arguments` (array of strings):
# Provides arguments to be passed directly to the `fzf` command.
# Use this field to customize `fzf`'s appearance and behavior, such as
# its layout, border style, or colors.
# For a full list of options, see the fzf man page or documentation.
fzf_arguments = ["--border=rounded", "--layout=rounded"]

# --- Plugin Configuration ---

# `[[plugins]]` (table array):
# This is a TOML table array used to define each plugin.
# Each `[[plugins]]` block represents a separate fzfmenu plugin.

[[plugins]]
# `name` (string):
# The name of the plugin, used for display in the fzfmenu interface.
# It should be a short, easily recognizable name, like "app_launcher".
name = "app_launcher"

# `description` (optional string):
# A brief description of the plugin, providing extra information about its purpose.
# This can be displayed as help text in the fzfmenu interface.
description = "Launch applications based on your desktop environment."

# `prefix` (string):
# The command prefix that triggers this plugin. Users type this prefix at the fzf prompt
# to activate the plugin.
# If the prefix is an empty string (""), this plugin is considered the default.
prefix = ""

# `picker` (string):
# The command executed when the plugin is activated to generate a list of candidates.
# Fzfmenu runs this command and uses its standard output as fzf's input.
# The `{}` is a placeholder that will be replaced by the remaining text the user typed
# at the fzf prompt.
picker = "python ~/.config/fzfmenu/plugins/app_launcher.py picker {}"

# `runner` (string):
# The command executed when a user selects an item from the list.
# Fzfmenu runs this command.
# The `{}` placeholder will be replaced by the result the user selected.
runner = "python ~/.config/fzfmenu/plugins/app_launcher.py runner {}"

# more plugins

[[plugins]]
name = "killer"
description = "Find and terminate running processes."
prefix = "kl "
picker = "python ~/.config/fzfmenu/plugins/killer.py picker {}"
runner = "python ~/.config/fzfmenu/plugins/killer.py runner {}"


[[plugins]]
name = "jumper"
description = "Jump to any windows (only works on Ura WM)."
prefix = "wd "
picker = "python ~/.config/fzfmenu/plugins/jumper.py picker {}"
runner = "python ~/.config/fzfmenu/plugins/jumper.py runner {}"


[[plugins]]
name = "history"
description = "Search and reuse past shell commands."
prefix = "hs "
picker = "python ~/.config/fzfmenu/plugins/history.py picker {}"
runner = "python ~/.config/fzfmenu/plugins/history.py runner {}"
```

A simpler version of configuration file can be found in [examples/config.toml](examples/config.toml).

You can also find examples for plugins in the [examples/plugins](examples/plugins) directory.

## More

For more detailed implementation insights, you can refer to this blog post: [The Implementation Approach of fzfmenu](https://blog.maruka.top/posts/Linux/fzfmenu%E5%AE%9E%E7%8E%B0%E6%80%9D%E8%B7%AF/). The post explains the inside-implementation of the previous Python version.

