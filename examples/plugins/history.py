import os
import subprocess
import sys

shell = os.environ["SHELL"]


def history_picker(input):
    atuin_init_cmd = ""
    if "zsh" in shell:
        atuin_init_cmd = 'eval "$(atuin init zsh)"'
    elif "bash" in shell:
        atuin_init_cmd = 'eval "$(atuin init bash)"'
    elif "fish" in shell:
        atuin_init_cmd = "atuin init fish | source"

    output = (
        subprocess.check_output(
            [
                "zsh",
                "-c",
                f'{atuin_init_cmd} && atuin search "{input}" --cmd-only',
            ]
        )
        .strip()
        .decode()
    )

    for line in set(output.splitlines()):
        print(line)


def history_runner(output: str):
    subprocess.call(f"setsid {output} > /dev/null 2>&1", shell=True)


def main():
    args = sys.argv[2]
    match sys.argv[1]:
        case "picker":
            history_picker(args)
        case "runner":
            history_runner(args)


if __name__ == "__main__":
    main()
