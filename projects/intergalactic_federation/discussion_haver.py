import cbor2
import argparse
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("--discussion_path", type=Path)
args = parser.parse_args()

loaded_discussions = cbor2.load(args.discussion_path.open(mode="rb"))


def _enumerated_input(option_names, option_details={}, instruction=""):
    if instruction:
        print(instruction)

    option_names = list(option_names)

    for option_num, option_name in enumerate(option_names):
        if option_details:
            print(f"{option_num}) {option_details[option_name]}")
        else:
            print(f"{option_num}) {option_name}")

    return option_names[int(_input())]


def _input(instruction=""):
    if instruction:
        print(instruction)

    return input("> ")


def _choose_discussion(discussions):
    discussion_choice = _enumerated_input(
        discussions.keys(),
        instruction="Choose the discussion to have:\n",
    )
    chosen_discussion = discussions[discussion_choice]
    print(f"Beginning discussion '{discussion_choice}'\n")

    return chosen_discussion


def _run_dialogue(next_dialogue_keys, dialogues):
    if not next_dialogue_keys:
        return None

    next_dialogue_key = _enumerated_input(
        next_dialogue_keys,
        option_details={
            dialogue_key: f"{dialogues[dialogue_key]['player_line']['line']}"
            for dialogue_key in next_dialogue_keys
        },
    )

    chosen_dialogue = dialogues[next_dialogue_key]
    player_line = chosen_dialogue["player_line"]
    character_line = chosen_dialogue["character_line"]

    print(f"You say: '{player_line['line']}'\n")
    print(f"{chosen_dialogue['character_key']} says: '{character_line['line']}'\n")

    return chosen_dialogue


while True:
    discussion = _choose_discussion(loaded_discussions)

    starter_keys = discussion["starter_dialogue_keys"]
    dialogues = discussion["dialogues"]

    next_dialogue = _run_dialogue(starter_keys, dialogues)

    while next_dialogue:
        next_dialogue = _run_dialogue(next_dialogue["next_dialogue_keys"], dialogues)
