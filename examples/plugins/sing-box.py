import os
import subprocess
import sys


def sb_picker():
    for i in ["start", "restart", "stop"]:
        print(i)


def sb_runner(output: str):
    cmd = ["systemctl", "--user", output, "sing-box.service"]
    subprocess.call(["notify-send", "fzfmenu", " ".join(cmd)])
    subprocess.call(cmd)


def main():
    match sys.argv[1]:
        case "picker":
            sb_picker()
        case "runner":
            sb_runner(os.environ["FZFMENU_OUTPUT"])


if __name__ == "__main__":
    main()
