import cbor2
import argparse
from pathlib import Path

DIR_PATH = Path(__file__).parent.resolve()

parser = argparse.ArgumentParser()
parser.add_argument("--discussion_path", type=Path)
args = parser.parse_args()

discussion = cbor2.load((DIR_PATH / args.discussion_path).open(mode="rb"))[
    "weird_discussion"
]

starter_keys = discussion["starter_dialogue_keys"]
dialogues = discussion["dialogues"]


def _input(string):
    print(string)
    return input("> ")


def _run_dialogue(next_dialogue_keys, dialogues):
    enumeration_dict = {}
    if not next_dialogue_keys:
        return None

    for i, dialogue_key in enumerate(next_dialogue_keys):
        enumeration_dict[i] = dialogue_key
        print(f"{i}) '{dialogues[dialogue_key]['player_line']['line']}'")

    chosen_dialogue = dialogues[enumeration_dict[int(_input(''))]]

    print(f"\n{chosen_dialogue['character_key']} says '{chosen_dialogue['character_line']['line']}'\n")

    return chosen_dialogue

print(
    """
Start 'weird_discussion'.
"""
)

next_dialogue = _run_dialogue(starter_keys, dialogues)

while next_dialogue:
    next_dialogue = _run_dialogue(next_dialogue["next_dialogue_keys"], dialogues)
