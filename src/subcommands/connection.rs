use clap::Clap;
use crate::error::Error;
use crate::connection::Server;

#[derive(Clap)]
pub struct ConnectionCommand {
    #[clap(subcommand)]
    subcommand: SubCommand
}

#[derive(Clap)]
enum SubCommand {
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
struct ConnectionModifyArgs {
    #[clap()]
    name: String
}

impl ConnectionCommand {
    pub fn handle(&self) -> Result<(), Error> {
        match &self.subcommand {
            SubCommand::List =>
                list_connections(),
            SubCommand::Add(args) =>
                add_connection(args),
            SubCommand::Remove(args) =>
                remove_connection(args),
            SubCommand::Edit(args) =>
                edit_connection(args)
        }
    }
}

fn list_connections() -> Result<(), Error> {
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

fn add_connection(args: &ConnectionModifyArgs) -> Result<(), Error> {
    let info = Server::prompt_details()?;
    // TODO Validate connection info
    info.save(&args.name)?;
    println!("Successfully added \"{}\"", &args.name);

    Ok(())
}

fn remove_connection(args: &ConnectionModifyArgs) -> Result<(), Error> {
    // TODO Fix usage of owned string
    Server::remove_saved(&args.name)?;
    println!("Successfully removed \"{}\"", &args.name);

    Ok(())
}

fn edit_connection(_args: &ConnectionModifyArgs) -> Result<(), Error> {
    // let connection = ConnectionInfo::load_saved(&args.name)?;
    todo!();
}