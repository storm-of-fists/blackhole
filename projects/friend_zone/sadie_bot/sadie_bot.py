import discord
import re
import asyncio
import pathlib
import random

# SENTRY
from tools.python.sentry import sentry
sentry.init()

# LOGGING
import tools.python.log.log as log
LOG, HANDLERS, FORMATTER = log.init(name="sadie_bot")


intents = discord.Intents.default()
intents.message_content = True

CLIENT = discord.Client(intents=intents)
DISCORD_TOKEN = pathlib.Path("/var/sadie_bot/token.txt").read_text()


@CLIENT.event
async def on_ready():
    LOG.info(f"Logged on as {CLIENT.user}!")


@CLIENT.event
async def on_message(message):
    if "https://twitter.com" in message.content:
        await fix_and_repost_twitter_links(message)


async def sadie_reply(message=None):
    _barks = ("rauf", "arf", "roof", "*whining*")

    await message.reply(random.choice(_barks))


async def fix_and_repost_twitter_links(message):
    if not message.embeds:
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
    else:
        if random.random() > 0.8:
            await sadie_reply(message)


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
        handler=HANDLERS[1], formatter=FORMATTER, level=log.INFO, root=False
    )
    await CLIENT.start(DISCORD_TOKEN)


if __name__ == "__main__":
    try:
        asyncio.run(start_client())
    except KeyboardInterrupt:
        LOG.debug("Caught keyboard interrupt, shutting down.")
