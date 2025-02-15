# ffmenu

ffmenu is an application launcher inspired by [using_fzf_as_a_dmenurofi_replacement](https://www.reddit.com/r/commandline/comments/jatyek/using_fzf_as_a_dmenurofi_replacement/). It harnesses the power of fzf’s fuzzy search to offer a unified tool for i3-wm users, enabling quick application launching, efficient window switching (via window-jump and i3ipc), and process management (search/kill).

## Dependencies

- **i3-wm** (with the window-jump plugin and custom window styling)
- **i3ipc**
- **alacritty**
- **fish** (could edit the code by yourself to use other shell like bash or zsh)
- **dex**
- **fzf** (the core dependency)

## Installation & Configuration

1. **Install Dependencies**  
   Ensure all the required dependencies are installed and that your i3-wm is correctly configured.

2. **Clone the Repository**
   
   ```bash
   git clone https://github.com/levinion/fzfmenu
   ```

3. **Then run the code**

	```python
    cd fzfmenu
	python fzfmenu.py
	```

## Configure i3 Window Rules

Add the following rule to your i3 configuration to display ffmenu in a floating window with fixed dimensions and centered:

```
for_window [class="fzfmenu"] floating enable, resize set height 400, resize set width 1200, move position center, focus
```

## Customize

Adjust ffmenu’s behavior and appearance as needed. For more detailed implementation insights, refer to this blog post: <https://blog.maruka.top/posts/Linux/ffmenu%E5%AE%9E%E7%8E%B0%E6%80%9D%E8%B7%AF/>.
