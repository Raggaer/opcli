use std::error;
use std::option;
use std::process;
use std::string;

pub fn execute_get_item_command(
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

    let stdout = match crate::command::execute_command_stdout(
        "op",
        vec!["get".to_string(), "item".to_string(), item_id],
    ) {
        Ok(out) => out,
        Err(e) => return Err(e),
    };

    return parse_get_item_output(stdout, fields, password_only);
}

// Convert command stdout into human readable text
fn parse_get_item_output(
    output: String,
    fields: option::Option<string::String>,
    password_only: bool,
) -> Result<(), Box<dyn error::Error>> {
    let item: crate::command::get::OpItem = serde_json::from_str(&output)?;

    // Check if user only wants the password
    if password_only {
        crate::clipboard::write(item.get_field("password".to_string()))?;
    }

    match fields {
        Some(f) => {
            // Loop over the provided field list
            let field_list = f.split(",");
            for field in field_list {
                match field.trim() {
                    "uuid" => println!("UUID: {}", item.uuid),
                    "url" => println!("URL: {}", item.overview.url),
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
