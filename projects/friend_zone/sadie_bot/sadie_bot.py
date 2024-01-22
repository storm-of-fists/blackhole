import discord
import re
import asyncio
import pathlib
import random

from tools.python.application.application import app


intents = discord.Intents.default()
intents.message_content = True

CLIENT = discord.Client(intents=intents)
DISCORD_TOKEN = pathlib.Path("/var/sadie_bot/token.txt").read_text()


@CLIENT.event
async def on_ready():
    app.log.info(f"Logged on as {CLIENT.user}!")


async def sadie_reply(message):
    roll = random.random()

    # Make sure to always go from lowest probability to highest.
    if roll > 0.995:
        # Sometimes Sadie learns to speak.
        sadie_message = await message.reply(
            "No one will ever believe you that you heard a dog talk. Fuck you."
        )
        # We wanna freak em.
        await sadie_message.delete(delay=5.0)
    elif roll > 0.97:
        await message.reply(random.choice(("rauf", "arf", "roof", "*whining*")))


async def check_and_handle_twitter(message):
    if "https://twitter.com" in message.content:
        # Need a sleep here because embeds take a second to show up for twitter.
        await asyncio.sleep(1)

        if not message.embeds:
            await fix_and_repost_twitter_embed(message)


@CLIENT.event
async def on_message(message):
    await check_and_handle_twitter(message)
    await sadie_reply(message)


async def fix_and_repost_twitter_embed(message):
    twitter_links = re.findall(r"https://twitter.com(.*)[ ,\n]*", message.content)
    urls = {f"rauf! https://vxtwitter.com{link}" for link in twitter_links}
    await asyncio.gather(
        *(
            message.reply(
                url,
                silent=True,
                allowed_mentions=discord.AllowedMentions(replied_user=False),
            )
            for url in urls
        )
    )

    if random.random() > 0.8:
        await austin_says_thanks(message)


async def austin_says_thanks(message):
    await message.reply(
        "This is Austin Lindell, thanks Sadie!",
        silent=True,
        allowed_mentions=discord.AllowedMentions(replied_user=False),
    )


async def start_client():
    # TODO, use multiple handlers here. should be possible idk why not.
    # silly discord py lib
    discord.utils.setup_logging(
        handler=app.log_handlers[1],
        formatter=app.log_formatter,
        level=app.logging.INFO,
        root=False,
    )
    await CLIENT.start(DISCORD_TOKEN)


app.set_name("sadie_bot").set_async_entry(start_client).run()
