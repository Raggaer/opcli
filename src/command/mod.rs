use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::option;
use std::process;
use std::string;

pub mod get;
pub mod list;

#[derive(Debug)]
struct MissingCommandError(String);

#[derive(Debug)]
struct CommandExecuteError(String);

#[derive(Serialize, Deserialize, Debug)]
struct OpItem {
    uuid: String,
    vaultUuid: String,

    #[serde(default)]
    details: OpDetails,

    overview: OpOverview,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpDetails {
    fields: Vec<OpField>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpOverview {
    url: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpField {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpUser {
    uuid: String,
    firstName: String,
    lastName: String,
    email: String,
}

impl Default for OpDetails {
    fn default() -> Self {
        OpDetails { fields: vec![] }
    }
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

impl error::Error for MissingCommandError {}

impl error::Error for CommandExecuteError {}

pub fn execute_list_command(
    sub: option::Option<string::String>,
    search: option::Option<string::String>,
) {
    let r = match sub {
        Some(s) => match s.as_str() {
            "items" => crate::command::list::item::execute_list_item_command(search),
            "users" => crate::command::list::user::execute_list_user_command(),
            sub => {
                eprint!("Unkown subcommand '{}'", sub);
                Ok(())
            }
        },
        None => {
            eprint!("Missing subcommand flag (-s)");
            Ok(())
        }
    };
    if let Err(e) = r {
        eprint!("{}", e);
    }
}

pub fn execute_get_command(
    sub: option::Option<string::String>,
    item: option::Option<string::String>,
    fields: option::Option<string::String>,
    password_only: bool,
) {
    let r = match sub {
        Some(s) => match s.as_str() {
            "item" => {
                crate::command::get::item::execute_get_item_command(item, fields, password_only)
            }
            sub => {
                eprintln!("Unkown subcommand '{}'", sub);
                Ok(())
            }
        },
        None => {
            eprintln!("Missing subcommand flag (-s)");
            Ok(())
        }
    };
    if let Err(e) = r {
        eprintln!("{}", e);
    }
}

pub fn execute_command_stdout(
    command: &str,
    args: Vec<String>,
) -> Result<String, Box<dyn error::Error>> {
    let cmd_output = process::Command::new(command).args(args).output()?;
    let stdout = match std::str::from_utf8(&cmd_output.stdout) {
        Ok(s) => s,
        Err(err) => return Result::Err(Box::new(err)),
    };
    let stderr = match std::str::from_utf8(&cmd_output.stderr) {
        Ok(s) => s,
        Err(err) => return Result::Err(Box::new(err)),
    };

    // Handle output from stderr as an error
    if !stderr.is_empty() {
        return Result::Err(Box::new(crate::command::CommandExecuteError(
            stderr.to_string(),
        )));
    }
    return Ok(stdout.to_string());
}
