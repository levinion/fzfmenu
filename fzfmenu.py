import os
import subprocess
import sys
import i3ipc

i3 = i3ipc.Connection()
shell = os.environ["SHELL"]
terminal = "alacritty"


def main():
    argv = sys.argv
    if len(argv) == 1:
        call_fzf()
    else:
        if argv[1] == "picker":
            run_plugins_picker(" ".join(argv[2:]))
        elif argv[1] == "run":
            run_plugins(" ".join(argv[2:]))


def call_fzf():
    path = os.path.realpath(__file__)
    subprocess.call(
        [
            shell,
            "-c",
            f"{terminal} \
            --class fzfmenu \
            -e {shell} -c \
            \"fzf \
            --bind 'start,change:reload:python {path} picker {{q}}' \
            --bind 'enter:become(nohup python {path} run {{}} > /dev/null 2>&1 &)'\" \
            ",
        ]
    )


def run_plugins(output: str):
    if output.startswith("kl "):
        killer_runner(output)
    elif output.startswith("wd "):
        window_jump_runner(output)
    elif output.startswith("hs "):
        history_runner(output)
    else:
        open_application_runner(output)


def open_application_runner(output: str):
    desktop = output.split(" ")[-1]
    if os.path.exists(desktop):
        subprocess.call([shell, "-c", f"dex {desktop}"])


def window_jump_runner(output: str):
    id = output.split(" ")[-1]
    i3.command(f'[con_id="{id}"] focus')


def run_plugins_picker(input: str):
    if input.startswith("wd "):
        window_jump_picker(input)
    elif input.startswith("kl "):
        killer_picker(input)
    elif input.startswith("hs "):
        history_picker(input)
    else:
        open_application_picker(input)


def window_jump_picker(_):
    tree = i3.get_tree()
    walk_tree(tree)


def walk_tree(tree: i3ipc.Con):
    for node in tree.leaves():
        walk_tree(node)
    if tree.window_title is not None:
        print("wd " + str(tree.window_title) + " " + str(tree.ipc_data["id"]))


def open_application_picker_by_path(path: str):
    output = (
        subprocess.check_output([shell, "-c", f"fd -a .desktop {path}"])
        .strip()
        .decode()
    )
    for line in output.splitlines():
        name = get_name_by_path(line)
        if name is not None:
            print(name + " " + line)


def open_application_picker(_):
    open_application_picker_by_path("/usr/share/applications/")
    open_application_picker_by_path(os.path.expanduser("~/Desktop/"))


def get_name_by_path(path: str):
    with open(path, "r") as f:
        for line in f.readlines():
            if line.startswith("Name="):
                return line.removeprefix("Name=").strip()


def killer_picker(input: str):
    input = input.removeprefix("kl ")
    if len(input) == 0:
        return
    output = (
        subprocess.check_output([shell, "-c", f"pgrep -fa {input}"]).strip().decode()
    )
    path = os.path.realpath(__file__)
    for line in output.splitlines():
        if path in line:
            continue
        print("kl " + line)


def killer_runner(output: str):
    pid = output.removeprefix("kl ").split(" ")[0]
    subprocess.call([shell, "-c", f"kill -9 {pid}"])


def history_picker(_):
    output = subprocess.check_output([shell, "-i", "-c", "history"]).strip().decode()
    for line in output.splitlines():
        print("hs " + line)


def history_runner(output: str):
    cmd = output.removeprefix("hs ")
    subprocess.call([shell, "-c", f"nohup {cmd} > /dev/null 2>&1 &"])


if __name__ == "__main__":
    main()
