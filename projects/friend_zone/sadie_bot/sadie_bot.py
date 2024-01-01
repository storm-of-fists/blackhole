import discord
import re
import asyncio
import pathlib

intents = discord.Intents.default()
intents.message_content = True

CLIENT = discord.Client(intents=intents)
TOKEN = pathlib.Path("/var/sadie_bot/token.txt").read_text()


@CLIENT.event
async def on_ready():
    print(f"Logged on as {CLIENT.user}!")


@CLIENT.event
async def on_message(message):
    if "https://twitter.com" in message.content:
        await fix_and_repost_twitter_links(message)


async def fix_and_repost_twitter_links(message):
    twitter_links = re.findall(r"https://twitter.com(.*)[ ,\n]*", message.content)
    urls = {f"https://vxtwitter.com{link}" for link in twitter_links}
    await asyncio.gather(*(message.reply(url) for url in urls))


async def start_client():
    await CLIENT.start(TOKEN)


if __name__ == "__main__":
    asyncio.run(start_client())
