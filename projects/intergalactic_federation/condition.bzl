"""Provider and rules for conditions in game."""

ConditionInfo = provider(
    "A condition that must be met for an event to occur.",
    fields = {
        "statement": "condition statement",
        "any": "any of the conditions in here can be met.",
        "all": "all of the conditions in here must be met.",
    },
)

def _condition_impl(ctx):
    return [ConditionInfo(
        statement = ctx.attr.statement,
        any = ctx.attr.any,
        all = ctx.attr.all,
    )]

condition = rule(
    implementation = _condition_impl,
    attrs = {
        "statement": attr.string(mandatory = True),
        "any": attr.label_list(default = []),
        "all": attr.label_list(default = []),
    },
)
