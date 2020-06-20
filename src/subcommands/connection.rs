use clap::Clap;
use crate::error::Error;
use crate::connection::Server;

#[derive(Clap)]
pub enum ConnectionCommand {
    /// List all saved connections
    #[clap(name = "list")]
    List,
    /// Add a new connection
    #[clap(name = "add")]
    Add(ConnectionModifyArgs),
    /// Remove a saved connection
    #[clap(name = "remove")]
    Remove(ConnectionModifyArgs),
    /// Edit a saved connection
    #[clap(name = "edit")]
    Edit(ConnectionModifyArgs)
}

#[derive(Clap)]
pub struct ConnectionModifyArgs {
    #[clap()]
    name: String
}

pub fn list() -> Result<(), Error> {
    println!("ALL CONNECTIONS");
    for pair in Server::list_saved() {
        let (key, value) = pair?;
        let keystr = std::str::from_utf8(&key)?;
        let connection_info =
            bincode::deserialize::<Server>(&value)?;
        println!("{}\t{:?}", keystr, connection_info);
    }

    Ok(())
}

pub fn add(args: &ConnectionModifyArgs) -> Result<(), Error> {
    let info = Server::prompt_details()?;
    // TODO Validate connection info
    info.save(&args.name)?;
    println!("Successfully added \"{}\"", &args.name);

    Ok(())
}

pub fn remove(args: &ConnectionModifyArgs) -> Result<(), Error> {
    // TODO Fix usage of owned string
    Server::remove_saved(&args.name)?;
    println!("Successfully removed \"{}\"", &args.name);

    Ok(())
}

pub fn edit(_args: &ConnectionModifyArgs) -> Result<(), Error> {
    // let connection = ConnectionInfo::load_saved(&args.name)?;
    todo!();
}