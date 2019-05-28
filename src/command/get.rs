use std::error;
use std::option;
use std::process;
use std::string;

pub fn execute_command(
    sub: option::Option<string::String>,
    item: option::Option<string::String>,
    fields: option::Option<string::String>,
    password_only: bool,
) {
    match sub {
        Some(s) => match s.as_str() {
            "item" => {
                if let Err(e) = execute_get_item_command(item, fields, password_only) {
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

fn execute_get_item_command(
    item: option::Option<string::String>,
    fields: option::Option<string::String>,
    password_only: bool,
) -> Result<(), Box<dyn error::Error>> {
    // Check if item is set
    if item.is_none() {
        return Result::Err(Box::new(crate::command::MissingCommandError(
            "Missing item identifier flag (-i)".into(),
        )));
    }
    let item_id = item.unwrap();

    // Create op command
    let cmd_output = process::Command::new("op")
        .arg("get")
        .arg("item")
        .arg(item_id)
        .output()?;
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

    return parse_get_item_output(stdout, fields, password_only);
}

// Convert command stdout into human readable text
fn parse_get_item_output(
    output: &str,
    fields: option::Option<string::String>,
    password_only: bool,
) -> Result<(), Box<dyn error::Error>> {
    let item: crate::command::OpItem = serde_json::from_str(output)?;

    // Check if user only wants the password
    if password_only {
        crate::clipboard::write(item.get_field("password".to_string()))?;
    }

    match fields {
        Some(f) => {
            // Loop over the provided field list
            let field_list = f.split(",");
            for field in field_list {
                match field {
                    "uuid" => println!("UUID: {}", item.uuid),
                    "vaultUuid" => println!("Vault UUID: {}", item.vaultUuid),
                    "username" => println!("Username: {}", item.get_field("username".to_string())),
                    "password" => println!("Password: {}", item.get_field("password".to_string())),
                    _ => (),
                }
            }
        }
        None => (),
    }
    Ok(())
}
