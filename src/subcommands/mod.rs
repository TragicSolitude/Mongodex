mod connection;
mod dump;
mod restore;
mod migrate;

use crate::error::Error;
use crate::SubCommand;
pub use connection::ConnectionCommand;
pub use dump::DumpOptions;
pub use restore::RestoreOptions;
pub use migrate::MigrateOptions;

pub fn run(subcommand: &SubCommand) -> Result<(), Error> {
    match subcommand {
        SubCommand::Connection(command) => match command {
            ConnectionCommand::List => connection::list(),
            ConnectionCommand::Add(args) => connection::add(args),
            ConnectionCommand::Remove(args) => connection::remove(args),
            ConnectionCommand::Edit(args) => connection::edit(args),
            ConnectionCommand::ListDatabases(args) => connection::list_databases(args)
        },
        SubCommand::Dump(options) => dump::run(options),
        SubCommand::Restore(options) => restore::run(options),
        SubCommand::Migrate(options) => migrate::run(options)
    }
}