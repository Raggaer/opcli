use std::error;
use std::option;
use std::string;

pub fn execute_list_item_command(
    search: option::Option<string::String>,
) -> Result<(), Box<dyn error::Error>> {
    // Retrieve JSON
    let stdout = match crate::command::execute_command_stdout(
        "op",
        vec!["list".to_string(), "items".to_string()],
    ) {
        Ok(out) => out,
        Err(e) => return Err(e),
    };
    parse_list_item_output(stdout, search)
}

fn parse_list_item_output(
    output: String,
    search: option::Option<string::String>,
) -> Result<(), Box<dyn error::Error>> {
    let list: Vec<crate::command::OpItem> = serde_json::from_str(&output)?;
    match search {
        Some(query) => show_list_item(list, &query),
        None => show_list_item(list, ""),
    }
    Ok(())
}

fn show_list_item(mut list: Vec<crate::command::OpItem>, query: &str) {
    // If query is not empty modify the list with matched items
    if !query.is_empty() {
        list = list
            .into_iter()
            .filter(|item| item.overview.title.contains(query) && item.overview.url.contains(query))
            .collect();
    }
    for (i, item) in list.iter().enumerate() {
        println!("UUID: {}", item.uuid);
        println!("URL: {}", item.overview.url);
        println!("Title: {}", item.overview.title);
        if i < list.len() - 1 {
            println!("Vault UUID: {}\r\n", item.vaultUuid);
        } else {
            println!("Vault UUID: {}", item.vaultUuid);
        }
    }
}
