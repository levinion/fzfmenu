import subprocess
import os
import sys


def no_display_is_true(path: str):
    with open(path, "r") as f:
        for line in f.readlines():
            if line.startswith("NoDisplay=true"):
                return True
    return False


def get_name_by_path(path: str):
    with open(path, "r") as f:
        for line in f.readlines():
            if line.startswith("Name="):
                return line.removeprefix("Name=").strip()


def open_application_picker_by_path(path: str):
    output = subprocess.check_output(
        f"fd -a .desktop {path}", shell=True, text=True
    ).strip()
    for path in output.splitlines():
        if no_display_is_true(path):
            continue
        name = get_name_by_path(path)
        if name is not None:
            print(name + " " + path)


def open_application_picker():
    open_application_picker_by_path("/usr/share/applications/")
    open_application_picker_by_path(os.path.expanduser("~/.local/share/applications/"))
    open_application_picker_by_path(os.path.expanduser("~/Desktop/"))


def open_application_runner(output: str):
    desktop = output.split(" ")[-1]
    if os.path.exists(desktop):
        subprocess.call(f"dex {desktop}", shell=True)


def main():
    match sys.argv[1]:
        case "picker":
            open_application_picker()
        case "runner":
            open_application_runner(" ".join(sys.argv[2:]))


if __name__ == "__main__":
    main()
