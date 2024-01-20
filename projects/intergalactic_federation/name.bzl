"""Different types of providers and rules for names of things."""

# TODO: decoy name!

CharacterNameInfo = provider(
    "A character provider",
    fields = {
        "given_first_name": "string",
        "given_last_name": "string",
        "used_first_name": "string",
        "used_last_name": "string",
        # "nicknames_that_replace_whole_name": "list of strings",
        # "nicknames_that_are_in_middle_of_name": "list of strings"
        # "special_nicknames": "special names corresponding with targets",
    },
)

def _character_name_impl(ctx):
    return [CharacterNameInfo(
        used_first_name = ctx.attr.used_first_name,
        used_last_name = ctx.attr.used_first_name,
        given_first_name = ctx.attr.given_first_name,
        given_last_name = ctx.attr.given_last_name,
    )]

_character_name = rule(
    implementation = _character_name_impl,
    attrs = {
        "given_first_name": attr.string(),
        "given_last_name": attr.string(),
        "used_first_name": attr.string(),
        "used_last_name": attr.string(),
    },
)

def character_name(name, **kwargs):
    _character_name(name = name, **kwargs)
