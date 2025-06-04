"""
TODO: generate typescript bindings (or js) https://fastapi.tiangolo.com/advanced/generate-clients/#generate-a-typescript-client-with-custom-operation-ids
TODO: impl auth
TODO: impl storage via google drive (any and all files)
TODO: explain how cyclic work can happen (cycling between phases and approvals) or via new work tickets.
TODO: regular checks on the database?
TODO: db connection pool

Figma: https://www.figma.com/design/kAac1fjfE2cHELzKygSZux/Good-Work?node-id=0-1&t=4QHaMIQKjYGHmqfi-1

1.  Relationships are expressed as hashtags in the workpad body.
2.  Work phase menu shows ticket creation and who made it and the next phase the tickets in, also allows setting user/group permissions to see/view/edit the work.
3.  Workpad can be split into tabs for easier organizing.
4.  Comments/discussion/log area. (shows all interactions with this work), allows diffing and reverting to old ticket changes. This log contains comments as well, and you can show or hide all actions or just the comments. Each action shows a pass/fail icon. Actuons can also be copied and applied to other works. Or whole groups or actions.
5.  At the bottom of the page is the search/interaction bar and create new ticket button. Both can be accessed via a shortcut.
6.  Tickets can define action buttons for creating other tickets.
7.  Adding phases/approvers/etc are all action based and can be seen in the log.

Okay, let's consolidate all the features and concepts we've discussed for your dead-simple work tracking tool. I'll try to group them logically.

I. Core Ticket Features:

Ticket Entity:

Title: Prominent and clear.

Phase/Status Menu: Tracks lifecycle (e.g., Open, In Progress, Review, Closed).

Creator & Creation Time: Automatically logged.

Assignees: Users responsible for the work.

Approvers: Users who need to approve certain stages/transitions.

Workpad Area:

Rich text editing.

Embed videos, text.

Embed files as links (stored in blob storage with unique IDs).

Reference embedded files from other tickets.

Discussion/Comments Area: For conversations related to the ticket.

Hashtags (#tag): In title or workpad for relating issues/topics.

Mentions (@user): In title or workpad to tag people.

Target Due Date Area: Simple date field.

Work History Log:

Tracks all interactions/changes (title edit, body edit, comments, status changes, assignee changes, etc.).

Diffing capabilities for changes.

Ability to "slide through" history to see the ticket evolve.

Unique ID: For each ticket.

II. User Interface & Navigation (Global):

Root View / Home Base:

Initially conceived as a list of recently modified tickets + prominent search bar.

Evolved into: "Username's Work" Ticket: Each user's primary view is a special, personal ticket.

Title: "Jane Doe's Work" (pre-filled, non-editable).

No standard phases/assignees/due date.

Workpad is fully customizable by the user to create their dashboard (embedding list views, Gantt views, notes, links to key tickets).

Serves as onboarding/tutorial page with pre-filled helpful content.

Global Search Bar:

Powerful, always accessible.

Searches: Titles, workpad content, filenames, comments, hashtags, @mentions, ticket IDs, usernames, status names.

Search Results Page: With snippets, metadata, sorting, and filtering options (by status, assignee, creator, hashtag, date range).

Simple search syntax (AND default, OR, quotes, NOT).

Persistent Header:

Global Search Bar.

"Create New Ticket" button.

Link back to "My Work" ticket.

Notifications Icon (badge count, list of relevant events).

User Profile/Settings access.

Ticket List View (Can be global or embedded):

Displays multiple tickets (title, status, assignee, due date).

Sorting options.

Quick filters (Assigned to Me, Created by Me, By Status, By Hashtag).

Gantt View (Can be global or embedded):

Visualizes tickets on a timeline based on start/due dates.

Timescale controls (day/week/month).

"Today" marker.

Relations View (Can be global or embedded):

Shows how tickets relate via hashtags (node graph or grouped list).

Interactive: click to navigate/filter.

III. Advanced Ticket & System Functionality:

Tickets as Containers:

No formal "project" entity; any ticket can act as a container by embedding views of other tickets (e.g., a list of sub-tasks).

Navigation: Breadcrumbs, "Parent Ticket" link.

Embedding Views within Workpad:

Users can embed live, interactive List, Gantt, or Relations views within any ticket's workpad.

Insertion via slash commands (/list) or toolbar.

Mini-UI for configuring the scope/filters of embedded views.

Permissions & Sharing (Per Ticket):

Accessed via status menu (or dedicated "Share" button).

Settings for user/group: Read, Write/Edit, Share.

Modal/panel UI to manage access.

Action Copying/Applying (Automation Foundation):

Copy a specific change/action from a ticket's Work History Log.

Apply (paste) that action to another ticket or multiple selected tickets.

Clear success/failure feedback.

(Future consideration: Trigger-based automation - IF X THEN DO Y).

"Project-Specific" Ticket Creation (from Container Tickets):

Buttons in a container ticket's workpad to "Create New Task for this Project."

Pre-populates new ticket fields (title prefix, hashtags, workpad template) based on the container.

Enhanced Phases Tooling (for Regulated Processes):

Phase Sets / Lifecycle Templates: Define named sequences of phases (e.g., "Bug Lifecycle").

Approvers Tied to Phase Transitions: Specific users/groups must approve before moving out of certain phases.

Required Checks/Fields for Phase Transition: Simple checklists or content requirements before changing phase.

Locking Fields Based on Phase: e.g., read-only after "Closed."

Kanban View (Embeddable): Columns as phases, drag-and-drop for phase changes.

"Snapshot" Action: Create a locked/PDF version of a ticket at a key milestone.

Simplified Custom Fields (via Hashtag Prefixes):

Conventions like priority:high, severity:critical.

Search understands prefixes; UI could suggest values.

Reporting via "Report Tickets":

Users create tickets whose workpads contain embedded, filtered list views to serve as reports.

IV. User Management (Potentially separate system, but essential):

User Accounts: Creation, invitation, authentication (SSO, email/pass).

User Profiles: Name, email, avatar.

Groups: Creation and management of user groups for simplified sharing.

System Admin Role(s): For managing users, groups, global settings.

V. Technical & Other Considerations:

Blob Storage: For all embedded files.

APIs: For potential integrations (inbound/outbound).

Webhooks: Simple outbound integration mechanism.

Performance: For embedded views and fast search.

Responsive Web UI: For access on different devices.

No explicit "Project" or "Folder" entities: Organization by search, hashtags, and ticket-as-container.

This list covers the core ideas and the elaborations we've discussed. The strength lies in the simplicity of the core ticket object and the power derived from its history, flexible workpad, and the ability to relate and embed tickets within each other, all navigated primarily by a robust search and personalized "My Work" tickets.

Dependencies between tickets, can be labelled or just related, then able to view the tickets

Ticket: title, description, document links (with viewers), labels? Idk, i think the relations handle the label part. Phase. Can have ticket templates. Multiple assigners, multiple approvers. Edits can be seen in the version history. Comments display on side so ticket context is never lost.

All work audited and captured in a work log. Can set archive limits for items.

Table view and dependency view.

Work tickets are not linked to the data source itself. On purpose. Githubs issues are tied to repositories but thats not how theyre used.

Organizations can have parent organizations. User accounts and groups are simple, lightweight.

Users
Organization -> project -> work

Work should have pre approvals and verification

But Id like to take a week to try and build a work tracking tool for myself.

Simple, json blobs in a postgres db.

Storyboards for putting files and notes. Dont use folders.

Forums and hashtags for posting about and getting answers to things or sending requests. Access each as a file.

Use hashtags for everything, forum posts, storyboards, individual files

Most transactions are logged in a huge "soft ordered" record store. Events are just inserted as fast as possible, but carry timestamps about when the request is ingested.

Messaging, voice chat, storyboards (like websites, but pageless, you just index them as needed), file storage, and work tracking.

- messaging
- forum style posting
- hashtags for tagging, request routing, tagging other users
- Users and groups, read/write/link /download permissions
- storyboards
- work tracking in gantt charts or table views
- custom work phases, approvals at each phase, documents to upload etc
- work types are defined by their phases
- audit log
- each item has a uuid, you link different things to that uuid
- indexing of common work in some cache
- tooltips and right click for help galore. Every item has a reference to documentation and video examples

work integrates the storyboard into the ticket. Conversations occur on the ticket, latest messages first.

Files put onto one storyboard are totally agnostic to it and others can use it elsewhere (its just a link to the file in blob storage)

Opening a file is as simple as opening a view right there on the page.

Could maybe be a built in tab system, like vscode.

Tomorrow, lets try to get the frontend up and the backend putting records into the database.

The next, maybe fastapi in a container.

Nginx running and serving the page at a secure url.

Then come up with a model for the ticket.

Have those blobs getting stored in postgres.

Things on a work:
Title
Phase info (phase name, assignees, approvers, other requirements)
Page-like board for any and all content you arZe showing off. Embed spreadsheets, videos, text, etc, on the page.
Discussion/chat/comments scroll window with search and filtering.
Created and edited dates are all tracked, all transactions/edits to a work are tracked. Only date is a target end date.
Relationships (more complicated, for data manager people and search stuff)
"""

from contextlib import asynccontextmanager
from fastapi import FastAPI, HTTPException, status
from models import *
from database import (
    initialize_database_pool,
    close_database_pool,
    get_database_connection,
)
from logger import logger


@asynccontextmanager
async def lifespan(app: FastAPI):
    # Load the ML model
    await initialize_database_pool()
    yield
    # Clean up the ML models and release the resources
    await close_database_pool()


app = FastAPI(lifespan=lifespan, redoc_url=None)


@app.get("/")
def root():
    return {"Hello": "World"}


#


@app.post("/create_work", status_code=status.HTTP_201_CREATED)
async def create_work(create_work: CreateWork):
    logger.info(create_work)


@app.get("/diagnostic/api", status_code=status.HTTP_200_OK)
async def diagnostic_api():
    """
    Returns healthcheck and other runtime info about the API.
    """
    return


@app.get("/diagnostic/database", status_code=status.HTTP_200_OK)
async def diagnostic_database():
    """
    Returns healthcheck and other runtime info about the database.
    """
    try:
        # Establish a connection
        conn = await asyncpg.connect(DATABASE_URL)  # type: ignore

        # Execute a simple query (e.g., SELECT 1) to verify connection
        # This is a lightweight way to check if the database is reachable and responsive
        await conn.execute("SELECT 1")

        # Close the connection
        await conn.close()

        return {"status": "ok", "database": "connected"}
    except Exception as e:
        # If any error occurs during connection or query, the database is considered unhealthy
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail=f"Database connection failed: {e}",
        )
