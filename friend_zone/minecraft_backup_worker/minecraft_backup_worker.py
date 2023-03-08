# https://github.com/dropbox/dropbox-sdk-python/blob/master/example/back-up-and-restore/backup-and-restore-example.py
# https://github.com/dropbox/dropbox-sdk-python/blob/master/example/updown.py

from base.python import log
import dropbox
from dropbox.files import CommitInfo, WriteMode, UploadSessionCursor, DeleteArg
from pathlib import Path
import shutil
import schedule
import time
import datetime


LOG = log.init("minecraft_backup_worker")

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
TMP_PATH = Path("/tmp")

ARCHIVE_TYPE = "tar"
ARCHIVE_NAME = "minecraft_server_backup"
ARCHIVE_FILE_FULL = f"{ARCHIVE_NAME}.{ARCHIVE_TYPE}"
ARCHIVE_PATH = TMP_PATH / ARCHIVE_FILE_FULL

MAX_CHUNK_SIZE = int(1e8)  # 100 MB 1e8

BACKUP_TIME_UTC = "11:00"  # 3am pst
CHECK_INTERVAL = 60  # seconds
DELETE_BACKUP_TIME = datetime.timedelta(days=3)


def backup_archive(dbx):
    try:
        dropbox_backup_path = f"/{datetime.datetime.now()}_{ARCHIVE_NAME}.{ARCHIVE_TYPE}"
        LOG.info(f"Uploading {ARCHIVE_FILE_FULL} to Dropbox as {dropbox_backup_path} ...")

        total_file_size = ARCHIVE_PATH.stat().st_size
        LOG.info(f"File size to be uploaded: {total_file_size}")
        
        with open(ARCHIVE_PATH, 'rb') as f:
            offset = 0
            read_chunk = f.read(MAX_CHUNK_SIZE)
            LOG.info("Beginning upload session.")
            upload_session = dbx.files_upload_session_start(read_chunk)
            offset += len(read_chunk)

            while offset < total_file_size:
                LOG.info(f"Upload status: {((offset / total_file_size) * 100):.2f} %")
                read_chunk = f.read(MAX_CHUNK_SIZE)
                dbx.files_upload_session_append_v2(
                    read_chunk,
                    UploadSessionCursor(session_id=upload_session.session_id, offset=offset),
                )
                offset += len(read_chunk)

            dbx.files_upload_session_finish(
                read_chunk,
                UploadSessionCursor(session_id=upload_session.session_id, offset=offset),
                CommitInfo(path=dropbox_backup_path, mode=WriteMode('overwrite'))
            )

            LOG.info("Upload session finished.")
    except Exception as err:
        LOG.error(err)


def create_archive():
    try:
        LOG.info(f"Creating archive at {ARCHIVE_PATH} from {DATA_PATH}")
        shutil.make_archive(f"{TMP_PATH}/{ARCHIVE_NAME}", ARCHIVE_TYPE, DATA_PATH)
        LOG.info(f"Done creating archive.")
    except Exception as err:
        LOG.error(err)


def delete_local_archive():
    try:
        LOG.info(f"Removing local archive at {ARCHIVE_PATH}")
        ARCHIVE_PATH.unlink()
    except Exception as err:
        LOG.error(err)


def delete_old_files(dbx):
    try:
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
            LOG.info("Done deleting files.")
        else:
            LOG.info("No files to delete.")
    except Exception as err:
        LOG.error(err)


def perform_backup(dbx):
    try:
        LOG.info("Start backup.")

        create_archive()
        backup_archive(dbx)
        delete_local_archive()

        LOG.info("All done!")
    except Exception as err:
        LOG.error(err)


def refresh_dropbox_token():
    try:
        LOG.info("Refresh dropbox token.")
        dbx.refresh_access_token()
    except Exception as err:
        LOG.error(err)
    

if __name__ == "__main__":
    with dropbox.Dropbox(
        oauth2_access_token=access_token(),
        oauth2_refresh_token=refresh_token(),
        app_key=app_key(),
        app_secret=app_secret()
    ) as dbx:
        LOG.info("Adding to scheduler.")
        schedule.every().day.at(BACKUP_TIME_UTC).do(refresh_dropbox_token)
        schedule.every().day.at(BACKUP_TIME_UTC).do(perform_backup, dbx=dbx)
        schedule.every().day.at(BACKUP_TIME_UTC).do(delete_old_files, dbx=dbx)

        LOG.info("Starting scheduler loop.")
        while True:
            schedule.run_pending()
            time.sleep(CHECK_INTERVAL)
