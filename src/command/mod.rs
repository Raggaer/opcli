use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::option;
use std::string;

pub mod get;

#[derive(Debug)]
struct MissingCommandError(String);

#[derive(Debug)]
struct CommandExecuteError(String);

#[derive(Debug)]
struct CommandJSONError(String);

#[derive(Serialize, Deserialize)]
struct OpItem {
    uuid: String,
    vaultUuid: String,
    details: OpDetails,
}

#[derive(Serialize, Deserialize)]
struct OpDetails {
    fields: Vec<OpField>,
}

#[derive(Serialize, Deserialize)]
struct OpField {
    name: String,
    value: String,
}

impl OpItem {
    pub fn get_field(&self, name: string::String) -> string::String {
        for field in &self.details.fields {
            if field.name == name {
                return field.value.clone();
            }
        }
        "".to_string()
    }
}

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

pub fn execute_command(
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
