# https://github.com/dropbox/dropbox-sdk-python/blob/master/example/back-up-and-restore/backup-and-restore-example.py
# https://github.com/dropbox/dropbox-sdk-python/blob/master/example/updown.py

import base.python as base
import dropbox
from pathlib import Path

LOG = base.log.init("minecraft_backup_worker")

TOKEN_PATH = Path("friend_zone/minecraft/backup_worker/access.token")
TOKEN = TOKEN_PATH.read_text()

dbx = dropbox.Dropbox(TOKEN)

user_act = dbx.users_get_current_account()

LOG.info(user_act)
