use std::error;
use std::fmt;
use std::option;
use std::string;

pub mod get;
pub mod list;

#[derive(Debug)]
struct MissingCommandError(String);

#[derive(Debug)]
struct CommandExecuteError(String);

#[derive(Debug)]
struct CommandJSONError(String);

impl fmt::Display for MissingCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for CommandExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for CommandJSONError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for MissingCommandError {}

impl error::Error for CommandExecuteError {}

impl error::Error for CommandJSONError {}

pub fn execute_list_command(
    sub: option::Option<string::String>,
    search: option::Option<string::String>,
) {
    match sub {
        Some(s) => match s.as_str() {
            "item" => {
                if let Err(e) = crate::command::list::item::execute_list_item_command(search) {
                    eprintln!("{}", e);
                }
            }
            sub => eprintln!("Unkown subcommand '{}'", sub),
        },
        None => {
            eprintln!("Missing subcommand flag (-s)");
            return;
        }
    };
}

pub fn execute_get_command(
    sub: option::Option<string::String>,
    item: option::Option<string::String>,
    fields: option::Option<string::String>,
    password_only: bool,
) {
    match sub {
        Some(s) => match s.as_str() {
            "item" => {
                if let Err(e) =
                    crate::command::get::item::execute_get_item_command(item, fields, password_only)
                {
                    eprintln!("{}", e);
                }
            }
            sub => eprintln!("Unkown subcommand '{}'", sub),
        },
        None => {
            eprintln!("Missing subcommand flag (-s)");
            return;
        }
    };
}
