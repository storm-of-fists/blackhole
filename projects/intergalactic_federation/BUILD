load("//projects/intergalactic_federation:character.bzl", "character")
load("//projects/intergalactic_federation:discussion.bzl", "dialogue", "discussion", "line")
load("//projects/intergalactic_federation:condition.bzl", "condition")
load("//tools/python:defs.bzl", "py_binary")

py_binary(
    name = "discussion_maker",
    srcs = ["discussion_maker.py"],
    deps = [
        "@external_py//cbor2",
    ],
)

py_binary(
    name = "discussion_haver",
    srcs = ["discussion_haver.py"],
    deps = [
        "@external_py//cbor2",
    ],
    data =[
        ":weird_discussion",
    ],

    args = [
        "--discussion_path weird_discussion.cbor"
    ]
)

### Phrases ###
line(
    name = "what_in_the_worlds",
    line = "What in the worlds!",
)

### Awkward hello discussion ###

character(
    name = "rafael_character",
    used_first_name = "Rafael",
    used_last_name = "Jender",
)

condition(
    name = "some_charm",
    statement = "charm >= 2",
)

# The start of the conversation.
dialogue(
    name = "hello_there",
    character = ":rafael_character.name",
    character_line = "hey whats up",
    next_dialogues = [
        ":awkward_reply",
        ":normal_reply",
    ],
    player_line = "hello there!",
)

# You dont stumble on your words.
dialogue(
    name = "normal_reply",
    character = ":rafael_character.name",
    character_line = "doin good man",
    conditions = [":some_charm"],
    next_dialogues = [":normal_goodbye"],
    player_line = "nothing much how are you",
)

dialogue(
    name = "normal_goodbye",
    character = ":rafael_character.name",
    character_line = "alright see ya man",
    player_line = "goodbye!",
)

# You completely fumble your words
dialogue(
    name = "awkward_reply",
    character = ":rafael_character.name",
    character_line = "whyd you say hello again?",
    next_dialogues = [":awkward_goodbye", ":more_awkward_reply"],
    player_line = "hello there!",
)

dialogue(
    name = "more_awkward_reply",
    character = ":rafael_character.name",
    character_line = "dude what?",
    next_dialogues = [":awkward_goodbye"],
    player_line = "ill fuck ya er uh!",
)

dialogue(
    name = "awkward_goodbye",
    character = ":rafael_character.name",
    character_line = "ok great, see ya",
    player_line = "im uh not sure",
)

# Finally, you put it as one of multiple starters for a discussion.
discussion(
    name = "weird_discussion",
    starter_dialogues = [
        ":hello_there",
    ],
)
