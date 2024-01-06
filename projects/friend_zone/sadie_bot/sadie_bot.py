import discord
import re
import asyncio
import pathlib

from tools.python import sentry, log

LOG = log.init(name="sadie_bot")
sentry.init()

intents = discord.Intents.default()
intents.message_content = True

CLIENT = discord.Client(intents=intents)
DISCORD_TOKEN = pathlib.Path("/var/sadie_bot/token.txt").read_text()


@CLIENT.event
async def on_ready():
    LOG.debug(f"Logged on as {CLIENT.user}!")


@CLIENT.event
async def on_message(message):
    if "https://twitter.com" in message.content:
        await fix_and_repost_twitter_links(message)


async def fix_and_repost_twitter_links(message):
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


async def start_client():
    await CLIENT.start(DISCORD_TOKEN)


if __name__ == "__main__":
    asyncio.run(start_client())
