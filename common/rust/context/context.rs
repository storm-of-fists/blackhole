use arrayvec::ArrayString;
use clap::{arg, Arg, ArgMatches, Command};
use raw_pointer::RawPointer;
use uuid::Uuid;

pub type ApplicationName = ArrayString<64>;

pub struct Context {
    uuid: Uuid,
    command: Command,
}

impl Context {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        let command = Command::new("context")
            .arg(arg!(--app_name <APP_NAME> "set the app name"))
            .arg(arg!(--uuid_override [UUID_OVERRIDE] "Specify the uuid that should be used by the application."));

        Self { uuid, command }
    }

    pub fn add_arg(mut self, arg: Arg) -> Self {
        self.command = self.command.arg(arg);
        return self;
    }

    pub fn parse_args(&mut self) -> ArgMatches {
        let parsed_args = self.command.get_matches_mut();

        if let Some(uuid_override) = parsed_args.get_one::<String>("uuid_override") {
            self.uuid =
                Uuid::try_parse(uuid_override).expect("UUID override was improperly formatted!");
        }

        return parsed_args;
    }

    pub fn uuid(&self) -> &Uuid {
        return &self.uuid;
    }
}

pub type ContextPtr = RawPointer<Context>;

#[macro_export]
macro_rules! init_context {
    () => {{
        use context::Context;

        let mut context = Context::new();
        raw_pointer::create_raw_ptr!(context)
    }};
}
