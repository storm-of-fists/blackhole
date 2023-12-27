"""Targets related to the discussions characters and players can have."""

load("//projects/intergalactic_federation:name.bzl", "CharacterNameInfo")
load("//projects/intergalactic_federation:condition.bzl", "ConditionInfo")

LineInfo = provider(
    "A line that a character says.",
    fields = {
        "line": "the line the player says.",
        "conditions": "list of the conditions that must be met for this line to be available.",
        "file": "cbor file for this line",
    },
)

def _line_impl(ctx):
    line_file = ctx.actions.declare_file("{}.cbor".format(ctx.attr.name))

    args = ctx.actions.args()
    args.add("--discussion_part_type", "line")
    args.add("--line", ctx.attr.line)
    args.add("--output_path", line_file)
    # args.add_all("--conditions", ctx.attr.conditions)

    ctx.actions.run(
        arguments = [args],
        outputs = [line_file],
        executable = ctx.executable._discussion_maker,
    )

    return [
        DefaultInfo(files = depset([line_file])),
        LineInfo(
            line = ctx.attr.line,
            conditions = ctx.attr.conditions,
            file = line_file,
        ),
    ]

line = rule(
    implementation = _line_impl,
    attrs = {
        "line": attr.string(mandatory = True),
        "conditions": attr.label_list(default = [], providers = [ConditionInfo]),
        # The script that makes the discussion file.
        "_discussion_maker": attr.label(
            default = Label("//projects/intergalactic_federation:discussion_maker"),
            executable = True,
            cfg = "exec",
        ),
    },
)

DialogueInfo = provider(
    "A line from a character and allowed player responses.",
    fields = {
        "player_line": "the line the character says.",
        "character_line": "character line",
        "character": "the character saying the line.",
        "conditions": "list of the conditions that must be met for this line to be available.",
        "next_dialogues": "optional next dialogue",
        "file": "cbor file for this info",
    },
)

def _dialogue_impl(ctx):
    dialogue_file = ctx.actions.declare_file("{}.cbor".format(ctx.attr.name))
    next_dialogue_paths = [dialogue[DialogueInfo].file for dialogue in ctx.attr.next_dialogues]

    args = ctx.actions.args()
    args.add("--discussion_part_type", "dialogue")
    args.add("--dialogue_key", ctx.attr.name)
    args.add("--player_line_path", ctx.attr.player_line[LineInfo].file)
    args.add("--character_line_path", ctx.attr.character_line[LineInfo].file)
    args.add("--character_key", "jimbo")  # TODO
    args.add_all(next_dialogue_paths, before_each = "--next_dialogue_paths")
    args.add("--output_path", dialogue_file)

    ctx.actions.run(
        arguments = [args],
        outputs = [dialogue_file],
        inputs = next_dialogue_paths + [ctx.attr.player_line[LineInfo].file, ctx.attr.character_line[LineInfo].file],
        executable = ctx.executable._discussion_maker,
    )

    return [
        DefaultInfo(files = depset([dialogue_file])),
        DialogueInfo(
            player_line = ctx.attr.player_line,
            character_line = ctx.attr.character_line,
            conditions = ctx.attr.conditions,
            character = ctx.attr.character,
            next_dialogues = ctx.attr.next_dialogues,
            file = dialogue_file,
        ),
    ]

_dialogue = rule(
    implementation = _dialogue_impl,
    attrs = {
        # Player line always comes first. then the character says something. then it goes to the next dialogue.
        "player_line": attr.label(
            providers = [LineInfo],
        ),
        # The line the character will say after the player. Will be the first thing said if there is no player line.
        "character_line": attr.label(
            providers = [LineInfo],
            mandatory = True,
        ),
        # The next dialogues will show each player line. When the player selects a line, it moves to that dialogue.
        "next_dialogues": attr.label_list(
            providers = [DialogueInfo],
        ),
        # The character saying the character_line.
        "character": attr.label(
            providers = [CharacterNameInfo],
            mandatory = True,
        ),
        # The conditions that allow this dialogue to be used by the player. Greyed out otherwise.
        "conditions": attr.label_list(
            default = [],
            providers = [ConditionInfo],
        ),
        # The script that makes the discussion file.
        "_discussion_maker": attr.label(
            default = Label("//projects/intergalactic_federation:discussion_maker"),
            executable = True,
            cfg = "exec",
        ),
    },
)

def dialogue(name, character_line, **kwargs):
    """
    nice

    Args:
        name (_type_): _description_
        player_line (_type_): _description_
        character_line (_type_): _description_
    """
    player_line_target = kwargs.pop("player_line", default = None)

    if type(player_line_target) == "string":
        player_line = player_line_target
        player_line_target = "{}.player_line".format(name)

        line(
            name = player_line_target,
            line = player_line,
        )

    character_line_target = character_line

    if type(character_line_target) == "string":
        character_line_target = "{}.character_line".format(name)

        line(
            name = character_line_target,
            line = character_line,
        )

    if player_line_target:
        kwargs["player_line"] = player_line_target

    _dialogue(
        name = name,
        character_line = character_line_target,
        **kwargs
    )

DiscussionInfo = provider(
    "All the information relevant to a single discussion.",
    fields = {
        "starter_dialogues": "list of the dialogue chains possible for this discussion.",
        "conditions": "list of the conditions that must be met for this line to be available.",
        "file": "the file",
    },
)

def _discussion_impl(ctx):
    discussion_file = ctx.actions.declare_file("{}.cbor".format(ctx.attr.name))
    starter_dialogues = [dialogue[DialogueInfo].file for dialogue in ctx.attr.starter_dialogues]

    args = ctx.actions.args()
    args.add("--discussion_part_type", "discussion")
    args.add_all(starter_dialogues, before_each = "--starter_dialogue_paths")
    args.add("--discussion_key", ctx.attr.name)
    args.add("--output_path", discussion_file)

    ctx.actions.run(
        arguments = [args],
        outputs = [discussion_file],
        inputs = starter_dialogues,
        executable = ctx.executable._discussion_maker,
    )

    return [
        DefaultInfo(
            files = depset([discussion_file]),
            runfiles = ctx.runfiles([discussion_file]),
        ),
        DiscussionInfo(
            starter_dialogues = ctx.attr.starter_dialogues,
            conditions = ctx.attr.conditions,
            file = discussion_file,
        ),
    ]

_discussion = rule(
    implementation = _discussion_impl,
    attrs = {
        "starter_dialogues": attr.label_list(providers = [DialogueInfo], mandatory = True),
        "conditions": attr.label_list(default = [], providers = [ConditionInfo]),
        # The script that makes the discussion file.
        "_discussion_maker": attr.label(
            default = Label("//projects/intergalactic_federation:discussion_maker"),
            executable = True,
            cfg = "exec",
        ),
    },
)

def _have_discussion_impl(ctx):
    file_to_run = ctx.attr._discussion_haver
    discussion_file = ctx.attr.discussion[DiscussionInfo].file

    run_file = ctx.actions.declare_file("{}".format(ctx.attr.name))
    run_file_contents = "./" + file_to_run.files_to_run.executable.short_path + " --discussion_path " + discussion_file.short_path

    ctx.actions.write(
        content = run_file_contents,
        is_executable = True,
        output = run_file,
    )

    # merge runfiles
    runfiles = file_to_run.default_runfiles.merge(ctx.runfiles([discussion_file, run_file]))

    return DefaultInfo(
        executable = run_file,
        runfiles = runfiles,
    )

_have_discussion = rule(
    implementation = _have_discussion_impl,
    executable = True,
    attrs = {
        "discussion": attr.label(providers = [DiscussionInfo], mandatory = True),
        "_discussion_haver": attr.label(
            default = Label("//projects/intergalactic_federation:discussion_haver"),
            executable = True,
            cfg = "exec",
        ),
    },
)

def discussion(name, **kwargs):
    _discussion(
        name = name,
        **kwargs
    )

    _have_discussion(
        name = "{}.run".format(name),
        discussion = name,
        tags = ["manual"],
    )
