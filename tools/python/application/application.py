from tools.python.sentry import sentry

# Create sentry before anything.
sentry.init()

# Non custom libs.
import asyncio

# Custom libs.
import tools.python.log.log as logging


class Application:
    def __init__(self):
        self.logging = logging
        self.sentry = sentry

        self.entry = lambda: self.log("Forgot to set an entry point.")

    def set_async_entry(self, entry):
        self.entry = lambda: asyncio.run(entry())

        return self

    def set_entry(self, entry):
        self.entry = entry

        return self

    def set_name(self, name):
        self.name = name
        return self

    def run(self):
        # Start logging right before app start.
        self.log, self.log_handlers, self.log_formatter = logging.init(name=self.name)

        try:
            self.entry()
        except KeyboardInterrupt:
            self.log.debug("Caught keyboard interrupt, shutting down.")


app = Application()
