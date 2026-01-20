import os
import subprocess
import sys


def window_jump_picker():
    command = """
local workspaces = ura.api.get_workspaces()
assert(workspaces)
for _, ws in ipairs(workspaces) do
	local wins = ura.api.get_windows(ws)
	if wins then
		for k, win in ipairs(wins) do
			if win then
				local app_id = ura.api.get_window_app_id(win)
				local title = ura.api.get_window_title(win)
				if not ura.api.is_workspace_named(ws) then
					local workspace_index = ura.api.get_workspace_index(ws)
					if workspace_index then
						print(app_id, title, workspace_index, k-1)
					end
				else
					local workspace_name = ura.api.get_workspace_name(ws)
					if workspace_name then
						print(app_id, title, workspace_name, k-1)
					end
				end
			end
		end
	end
end
"""

    output = subprocess.check_output(
        f"ura shell -c '{command}'", shell=True, text=True
    ).strip()
    for line in output.splitlines():
        line = line.strip()
        if line.startswith("fzfmenu"):
            continue
        print(line)


def window_jump_runner(output: str):
    args = output.split(" ")
    workspace_id = args[-2]
    window_index = args[-1]
    command = ""
    if not workspace_id.isnumeric():
        command = f""" 
local ws = ura.api.get_named_workspace("{workspace_id}")
assert(ws)
local win = ura.api.get_window(ws, {window_index})
assert(win)
ura.api.activate_window(win)
"""
    else:
        command = f""" 
local output = ura.api.get_current_output()
assert(output)
local ws = ura.api.get_indexed_workspace(output, {workspace_id})
assert(ws)
local win = ura.api.get_window(ws, {window_index})
assert(win)
ura.api.activate_window(win)
"""
    subprocess.call(f"ura shell -c '{command}'", shell=True)


def main():
    match sys.argv[1]:
        case "picker":
            window_jump_picker()
        case "runner":
            window_jump_runner(os.environ["FZFMENU_OUTPUT"])


if __name__ == "__main__":
    main()
