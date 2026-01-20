import os
import subprocess
import sys
import json


def sink_picker():
    output = subprocess.check_output("pw-dump").strip().decode()
    data = json.loads(output)
    for obj in data:
        try:
            if obj["info"]["props"]["media.class"] != "Audio/Sink":
                continue
            id = obj["id"]
            name = obj["info"]["props"]["node.description"]
            print(id, name)
        finally:
            pass


def sink_runner(output: str):
    output_ = output.split(" ")
    id = output_[0]
    name = " ".join(output_[1:])
    subprocess.call(["notify-send", f"Active: `{name}`"])
    subprocess.call(["wpctl", "set-default", id])


def main():
    match sys.argv[1]:
        case "picker":
            sink_picker()
        case "runner":
            sink_runner(os.environ["FZFMENU_OUTPUT"])


if __name__ == "__main__":
    main()
