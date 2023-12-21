load("//projects/intergalactic_federation/characters:character.bzl", "character")
load("//projects/intergalactic_federation:discussion.bzl", "dialogue", "player_line")

character(
    name = "rafael_character",
    used_first_name = "Rafael",
    used_last_name = "Jender",
)

player_line(
    name = "hello_there",
    line = "hello there!",
)

player_line(
    name = "hey_whats_up",
    line = "hey whats up!",
)

player_line(
    name = "oh_thats_embarassing",
    line = "hey whats up!",
)

dialogue(
    name = "hello_there_response",
    initial_player_line = ":hello_there",
    line = "hey hows it going",
    character = ":rafael_character.name",
    player_responses = [":hey_whats_up", ":hello_there"],
)

# dialogue(
#     name = "second_hello_response",
#     initial_player_line = ":hello_there_response",
#     line = "what whyd you say hello again?",
#     character = ":rafael_character",
#     player_responses = [":oh_thats_embarassing"],
# )
