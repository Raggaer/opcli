use std::error;
use std::option;
use std::string;

pub fn execute_list_user_command() -> Result<(), Box<dyn error::Error>> {
    // Retrieve JSON
    let stdout = match crate::command::execute_command_stdout(
        "op",
        vec!["list".to_string(), "users".to_string()],
    ) {
        Ok(out) => out,
        Err(e) => return Err(e),
    };
    parse_list_user_output(stdout)
}

fn parse_list_user_output(output: String) -> Result<(), Box<dyn error::Error>> {
    let list: Vec<crate::command::OpUser> = serde_json::from_str(&output)?;
    for (i, user) in list.iter().enumerate() {
        println!("UUID: {}", user.uuid);
        println!("First Name: {}", user.firstName);
        if i < list.len() - 1 {
            println!("Email: {}\r\n", user.email);
        } else {
            println!("Email: {}", user.email);
        }
    }
    Ok(())
}
