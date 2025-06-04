from pydantic import BaseModel
from datetime import datetime


class GoodWorkModel(BaseModel):
    """
    Base class for all Good Work requests.

    Empty right now, but this is used for top level design reference.

    - Dont put timestamps in client payloads. Use the server for timestamps.
    """


# TODO: this isnt needed for auth typically?
class LoggedInRequest(GoodWorkModel):
    user: str
    token: str


class CreateWork(LoggedInRequest):
    """
    Creates a Work. All it needs is a requester and a name for the work.

    Additional settings can prevent duplicate Work names.
    """

    # The name of the Work.
    name: str


class AddLog(LoggedInRequest):
    detail: str


class Assignee(GoodWorkModel):
    user: str


class Approver(GoodWorkModel):
    user: str


class AddPhase(LoggedInRequest):
    name: str
    assignees: list[Assignee]
    approvers: list[Approver]
    # requirements: list[str]


class WorkTab:
    name: str
    description: str


class WorkSpace:
    tabs: list[WorkTab]


class Permission:
    name: str

class RequiredApproval():
    permission: Permission
    default_user: str | None

class Phase:
    name: str
    started_at: datetime
    assignees: list[str]
    approvers: list[str]
    required_approvals: list[Permission]


class Work:
    name: str
    workspace: list[WorkTab]
    phases: list[Phase]


class ConnectionType:
    TIE = 0
    NEEDS = 1
    STOPS = 2


class Connection:
    label: str
    type: ConnectionType


# connection types
# tie:"This Work"
# needs:"Some Work Name"
# stops:"Another Work"

# work table, connections table, updates table, phases table?
# work: uuid, name, workspace_uuid, phase_list_uuid, updates_log
# TODO: i think at least workspace files need to be separate.
# files: uuid, name, filename, download_link, user_group_permissions, creation_date? edit_date? edit_log? maybe edit_log stays on the work tickets. maybe the file edit log is really just a combo of the edits in work tickets.
#
#
# users: id serial primary key, username, email, created_at
# work: id, title, phase_id, workspace_id
# phases:
