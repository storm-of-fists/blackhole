# https://github.com/dropbox/dropbox-sdk-python/blob/master/example/back-up-and-restore/backup-and-restore-example.py
# https://github.com/dropbox/dropbox-sdk-python/blob/master/example/updown.py

import base.python as base
import dropbox
from dropbox.files import CommitInfo, WriteMode, UploadSessionCursor, DeleteArg
from dropbox.exceptions import ApiError, AuthError
from pathlib import Path
import sys
import shutil
from progress.bar import IncrementalBar
import schedule
import time
import datetime


LOG = base.log.init("minecraft_backup_worker")

CREDS_PATH = Path("/credentials")

ACCESS_TOKEN_PATH = CREDS_PATH / "dropbox.access_token"
KEY_PATH = CREDS_PATH / "dropbox.key"
SECRET_PATH = CREDS_PATH / "dropbox.secret"
REFRESH_TOKEN_PATH = CREDS_PATH / "dropbox.refresh_token"


def access_token():
    return ACCESS_TOKEN_PATH.read_text().rstrip()


def app_key():
    return KEY_PATH.read_text().rstrip()


def app_secret():
    return SECRET_PATH.read_text().rstrip()


def refresh_token():
    return REFRESH_TOKEN_PATH.read_text().rstrip()


DATA_PATH = Path("/server_data")

SERVER_PATH = DATA_PATH / "test_dir"

ARCHIVE_TYPE = "tar"
ARCHIVE_NAME = "minecraft_server_backup"
ARCHIVE_FILE_FULL = f"{ARCHIVE_NAME}.{ARCHIVE_TYPE}"
ARCHIVE_PATH = DATA_PATH / ARCHIVE_FILE_FULL

DROPBOX_BACKUP_PATH = f"/{datetime.date.today()}_{ARCHIVE_NAME}.{ARCHIVE_TYPE}"

MAX_CHUNK_SIZE = int(1e8) # 100 MB 1e7

BACKUP_TIME_UTC = "09:00"
CHECK_INTERVAL = 60 # seconds
DELETE_BACKUP_TIME = datetime.timedelta(days=3)


def backup_archive(dbx):
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


def create_archive():
    LOG.info(f"Creating archive at {ARCHIVE_PATH} from {SERVER_PATH}")
    shutil.make_archive(BASE_PATH / ARCHIVE_NAME, ARCHIVE_TYPE, SERVER_PATH)


def delete_local_archive():
    LOG.info(f"Removing local archive at {ARCHIVE_PATH}")
    ARCHIVE_PATH.unlink()


def delete_old_files(dbx):
    LOG.info("Checking files to delete.")
    ids_to_delete = []
    things_in_root = dbx.files_list_folder("")

    for thing in things_in_root.entries:
        thing_id = thing.id
        date_created = thing.server_modified

        if datetime.datetime.now() - date_created > DELETE_BACKUP_TIME:
            LOG.info(f"Found file to delete: {thing.path_display}")
            ids_to_delete.append(DeleteArg(thing_id))

    if ids_to_delete:
        LOG.info(f"Deleting these ids: {ids_to_delete}")
        dbx.files_delete_batch(ids_to_delete)
    else:
        LOG.info("No files to delete.")


def perform_backup(dbx):
    LOG.info("Start backup.")

    dbx.refresh_access_token()

    create_archive()
    backup_archive(dbx)
    delete_local_archive()

    LOG.info("All done!")
    

if __name__ == "__main__":
    with dropbox.Dropbox(
        oauth2_access_token=access_token(),
        oauth2_refresh_token=refresh_token(),
        app_key=app_key(),
        app_secret=app_secret()
    ) as dbx:
        LOG.info("Adding to scheduler.")
        schedule.every().day.at(BACKUP_TIME_UTC).do(perform_backup, dbx=dbx)
        schedule.every().day.at(BACKUP_TIME_UTC).do(delete_old_files, dbx=dbx)

        LOG.info("Starting scheduler loop.")
        while True:
            schedule.run_pending()
            time.sleep(CHECK_INTERVAL)
