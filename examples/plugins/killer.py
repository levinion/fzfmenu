import os
import subprocess
import sys


def killer_picker(input: str):
    if len(input) == 0:
        return
    output = subprocess.check_output(f"pgrep -fa {input}", shell=True).strip().decode()
    for line in output.splitlines():
        if "fzfmenu" in line:
            continue
        print(line)


def killer_runner(output: str):
    pid = output.split(" ")[0]
    subprocess.call(["kill", "-9", pid])


def main():
    match sys.argv[1]:
        case "picker":
            killer_picker(os.environ["FZFMENU_INPUT"])
        case "runner":
            killer_runner(os.environ["FZFMENU_OUTPUT"])


if __name__ == "__main__":
    main()
