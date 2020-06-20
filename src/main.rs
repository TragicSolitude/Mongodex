#[macro_use]
extern crate lazy_static;

mod error;
mod guardian;
mod connection;
mod subcommands;

use clap::Clap;

lazy_static! {
    static ref PROJECT_DIRS: directories::ProjectDirs =
        directories::ProjectDirs::from("us", "InTheVoid", "Mongodex")
            .expect("No project directory available on this platform");
}

/// CLI tool for managing multiple MongoDB databases across multiple servers with an
/// interface inspired by the NetworkManager CLI.
#[derive(Clap)]
#[clap(version = "1.0", author = "Noah Shuart <shuart.noah.s@gmail.com>")]
pub enum SubCommand {
    /// Manage saved database connections
    #[clap(name = "connection", alias = "c")]
    Connection(subcommands::ConnectionCommand),
    /// Migrate one database to another
    #[clap(name = "migrate", alias = "m")]
    Migrate(subcommands::MigrateOptions),
    /// Dump a database to the filesystem [unstable]
    #[clap(name = "dump", alias = "b")]
    Dump(subcommands::DumpOptions),
    /// Restore a database from a dump [unstable]
    #[clap(name = "restore", alias = "s")]
    Restore(subcommands::RestoreOptions)
}

fn main() {
    let opts = SubCommand::parse();
    let res = subcommands::run(&opts);

    if let Err(e) = res {
        eprintln!("Error: {}", e);

        // TODO Cast error enum to actual, usable exit codes
        std::process::exit(1);
    }
}