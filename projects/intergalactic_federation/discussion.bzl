"""Targets related to the discussions characters and players can have."""

load("//projects/intergalactic_federation/characters:name.bzl", "CharacterNameInfo")

DiscussionInfo = provider(
    "All the information relevant to a single discussion.",
    fields = {
        "name": "the name of this discussion. should be a unique string.",
        "dialogue_chains": "list of the dialogue chains possible for this discussion.",
        "state_conditions": "list of the game state conditions that must be met for this discussion to be available.",
        "story_conditions": "list of the story conditions that must be met for this discussion to be available.",
    },
)

DialogueChainInfo = provider(
    "A discussion provider",
    fields = {
        "lines_in_order": "List of the player or character lines",
    },
)

PlayerLineInfo = provider(
    "A line from the player to a character.",
    fields = {
        "line": "the line the player says.",
        "state_conditions": "list of the game state conditions that must be met for this discussion to be available.",
        "story_conditions": "list of the story conditions that must be met for this discussion to be available.",
    },
)

def _player_line_impl(ctx):
    return [PlayerLineInfo(
        line = ctx.attr.line,
        state_conditions = ctx.attr.state_conditions,
        story_conditions = ctx.attr.story_conditions,
    )]

player_line = rule(
    implementation = _player_line_impl,
    attrs = {
        "line": attr.string(),
        "state_conditions": attr.label_list(default = []),
        "story_conditions": attr.label_list(default = []),
    },
)

DialogueInfo = provider(
    "A line from a character and allowed player responses.",
    fields = {
        "line": "the line the character says.",
        "player_responses": "the list of responses the player can reply with.",
        "initial_player_line": "the player line this is in response to. optional.",
        "character": "the character saying the line.",
        "state_conditions": "list of the game state conditions that must be met for this discussion to be available.",
        "story_conditions": "list of the story conditions that must be met for this discussion to be available.",
    },
)

def _dialogue_impl(ctx):
    return [DialogueInfo(
        line = ctx.attr.line,
        player_responses = ctx.attr.player_responses,
        initial_player_line = ctx.attr.initial_player_line,
        state_conditions = ctx.attr.state_conditions,
        character = ctx.attr.character,
        story_conditions = ctx.attr.story_conditions,
    )]

dialogue = rule(
    implementation = _dialogue_impl,
    attrs = {
        "line": attr.string(),
        "player_responses": attr.label_list(providers = [PlayerLineInfo]),
        "initial_player_line": attr.label(providers = [PlayerLineInfo]),
        "character": attr.label(providers = [CharacterNameInfo]),
        "state_conditions": attr.label_list(default = []),
        "story_conditions": attr.label_list(default = []),
    },
)
