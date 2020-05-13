use std::io;
use dialoguer::Select;

pub fn prompt_db(items: &[String]) -> Result<&str, io::Error> {
    let db = Select::new()
        .with_prompt("Select a database")
        .default(0)
        .items(items)
        .interact()?;
    
    Ok(&items[db])
}