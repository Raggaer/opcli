use std::error;
use std::option;
use std::string;

pub fn execute_list_item_command(
    search: option::Option<string::String>,
) -> Result<(), Box<dyn error::Error>> {
    let stdout = match crate::command::execute_command_stdout(
        "op",
        vec!["list".to_string(), "items".to_string()],
    ) {
        Ok(out) => out,
        Err(e) => return Err(e),
    };
    println!("{}", stdout);
    Ok(())
}
