load("//projects/intergalactic_federation:name.bzl", "character_name")

# CharacterInfo = provider(
#     "A character provider",
#     fields = {

#     },
# )

# def _character_impl(ctx):
#     return [CharacterInfo(name = ctx.attr.character_name)]

# _character = rule(
#     implementation = _character_impl,
#     attrs = {
#         "character_name": attr.string(),
#     },
# )

def character(name, **kwargs):
    character_name(name = "{}.name".format(name), **kwargs)
