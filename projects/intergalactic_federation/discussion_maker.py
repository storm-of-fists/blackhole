import cbor2
import argparse
from pathlib import Path

parser = argparse.ArgumentParser()
# Common
parser.add_argument("--conditions", type=Path, action="append", default=[])
parser.add_argument("--output_path", type=Path)

# Lines file
parser.add_argument("--discussion_part_type", type=str)
parser.add_argument("--line", type=str)

# Dialogue File
parser.add_argument("--dialogue_key", type=str)
parser.add_argument("--player_line_path", type=Path)
parser.add_argument("--character_line_path", type=Path)
parser.add_argument("--character_key", type=str)
parser.add_argument("--next_dialogue_paths", type=Path, action="append", default=[])

# Discussion File

parser.add_argument("--starter_dialogue_paths", type=Path, action="append")
parser.add_argument("--discussion_key", type=str)

args = parser.parse_args()

put_in_file = {}

if args.discussion_part_type == "line":
    put_in_file = {
        "line": args.line,
        "conditions": args.conditions,
    }

elif args.discussion_part_type == "dialogue":
    player_line = cbor2.load(args.player_line_path.open(mode="rb"))
    character_line = cbor2.load(args.character_line_path.open(mode="rb"))
    next_dialogue_keys = []

    for next_dialogue_path in args.next_dialogue_paths:
        next_dialogue = cbor2.load(next_dialogue_path.open(mode="rb"))
        put_in_file.update(next_dialogue)
        next_dialogue_keys.append(next_dialogue_path.stem)

    put_in_file[args.dialogue_key] = {
        "player_line": player_line,
        "character_line": character_line,
        "character_key": args.character_key,
        "next_dialogue_keys": next_dialogue_keys,
        "conditions": args.conditions,
    }

elif args.discussion_part_type == "discussion":
    put_in_file = {
        "dialogues": {},
        "starter_dialogue_keys": [],
        "conditions": args.conditions,
    }

    for dialogue_path in args.starter_dialogue_paths:
        dialogue_key = dialogue_path.stem
        dialogue = cbor2.load(dialogue_path.open(mode="rb"))
        put_in_file["dialogues"] = dialogue
        put_in_file["starter_dialogue_keys"].append(dialogue_key)

    put_in_file = {args.discussion_key: put_in_file}

print(put_in_file)

cbor2.dump(put_in_file, args.output_path.open(mode="wb"))
