import subprocess
import sys


def window_jump_picker():
    command = " ".join(
        [
            "local workspaces = ura.ws.list()",
            "for _, workspace in ipairs(workspaces) do",
            "for _, toplevel in ipairs(workspace.windows) do",
            "if workspace.name then",
            "print(toplevel.app_id, toplevel.title, workspace.name, toplevel.index)",
            "else",
            "print(toplevel.app_id, toplevel.title, workspace.index, toplevel.index)",
            "end",
            "end",
            "end",
        ]
    )
    output = subprocess.check_output(
        f"uracil -c '{command}'", shell=True, text=True
    ).strip()
    for line in output.splitlines():
        line = line.strip()
        print(line)


def window_jump_runner(output: str):
    args = output.split(" ")
    workspace_id = args[-2]
    window_index = args[-1]
    if not workspace_id.isnumeric():
        workspace_id = f'"{workspace_id}"'
    command = f"ura.win.activate({workspace_id},{window_index})"
    subprocess.call(f"uracil -c '{command}'", shell=True)


def main():
    args = " ".join(sys.argv[2:])
    match sys.argv[1]:
        case "picker":
            window_jump_picker()
        case "runner":
            window_jump_runner(args)


if __name__ == "__main__":
    main()
