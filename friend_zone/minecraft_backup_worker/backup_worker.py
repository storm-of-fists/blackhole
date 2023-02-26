# https://github.com/dropbox/dropbox-sdk-python/blob/master/example/back-up-and-restore/backup-and-restore-example.py
# https://github.com/dropbox/dropbox-sdk-python/blob/master/example/updown.py

import base.python as base
import dropbox
from dropbox.files import CommitInfo, WriteMode, UploadSessionCursor
from dropbox.exceptions import ApiError, AuthError
from pathlib import Path
import sys
import shutil
from progress.bar import IncrementalBar


LOG = base.log.init("minecraft_backup_worker")

TOKEN = Path("/etc/minecraft_backup_worker/dropbox_api.token").read_text().rstrip()

BASE_PATH = Path("/home/connor/code/blackhole/friend_zone")

SERVER_PATH = BASE_PATH / "minecraft_server"

ARCHIVE_TYPE = "tar"
ARCHIVE_NAME = "minecraft_server_backup"
ARCHIVE_FILE_FULL = f"{ARCHIVE_NAME}.{ARCHIVE_TYPE}"
ARCHIVE_PATH = BASE_PATH / ARCHIVE_FILE_FULL

DROPBOX_BACKUP_PATH = f"/{ARCHIVE_NAME}.{ARCHIVE_TYPE}"

MAX_CHUNK_SIZE = int(1e8) # 100 MB 1e7

if __name__ == "__main__":
    with dropbox.Dropbox(TOKEN) as dbx:
        # Check that the access token is valid
        try:
            dbx.users_get_current_account()
        except AuthError:
            LOG.error("ERROR: Invalid access token; try re-generating an "
                "access token from the app console on the web.")

        LOG.info(f"Creating archive at {ARCHIVE_PATH} from {SERVER_PATH}")
        # shutil.make_archive(BASE_PATH / ARCHIVE_NAME, ARCHIVE_TYPE, SERVER_PATH)

        LOG.info(f"Uploading {ARCHIVE_FILE_FULL} to Dropbox as {DROPBOX_BACKUP_PATH} ...")

        total_file_size = ARCHIVE_PATH.stat().st_size
        LOG.info(f"File size to be uploaded: {total_file_size}")
        
        with open(ARCHIVE_PATH, 'rb') as f:
            progress_bar = IncrementalBar('Upload Status', 
                                      max=total_file_size/MAX_CHUNK_SIZE,
                                      suffix='%(percent)d%%')

            offset = 0
            read_chunk = f.read(MAX_CHUNK_SIZE)
            upload_session = dbx.files_upload_session_start(read_chunk)
            offset += len(read_chunk)

            while offset < total_file_size:
                read_chunk = f.read(MAX_CHUNK_SIZE)
                dbx.files_upload_session_append_v2(
                    read_chunk,
                    UploadSessionCursor(session_id=upload_session.session_id, offset=offset),
                )
                offset += len(read_chunk)

                progress_bar.next()

            dbx.files_upload_session_finish(
                read_chunk,
                UploadSessionCursor(session_id=upload_session.session_id, offset=offset),
                CommitInfo(path=DROPBOX_BACKUP_PATH, mode=WriteMode('overwrite'))
            )

        LOG.info(f"Removing local archive at {ARCHIVE_PATH}")
        ARCHIVE_PATH.unlink()

        LOG.info("All done!")
    

