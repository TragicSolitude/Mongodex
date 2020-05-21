#[macro_use]
extern crate lazy_static;

mod error;
mod guardian;
mod connection_target;
mod connection_info;
mod subcommands;

use clap::Clap;

/// CLI tool for managing multiple MongoDB databases across multiple servers with an
/// interface inspired by the NetworkManager CLI.
#[derive(Clap)]
#[clap(version = "1.0", author = "Noah Shuart <shuart.noah.s@gmail.com>")]
struct CliOptions {
    #[clap(subcommand)]
    command: SubCommand
}

#[derive(Clap)]
enum SubCommand {
    /// Manage saved database connections
    #[clap(name = "connection", alias = "c")]
    Connection(subcommands::ConnectionCommand),
    /// Migrate one database to another
    #[clap(name = "migrate", alias = "m")]
    Migrate(subcommands::MigrateCommand),
    /// Dump a database to the filesystem [unstable]
    #[clap(name = "dump", alias = "b")]
    Dump(subcommands::DumpCommand),
    /// Restore a database from a dump [unstable]
    #[clap(name = "restore", alias = "s")]
    Restore(subcommands::RestoreCommand)
}

fn main() {
    let opts: CliOptions = CliOptions::parse();

    // TODO Follow XDG standard
    std::fs::create_dir_all("./data").unwrap();
    
    let result = match &opts.command {
        SubCommand::Connection(subcommand) => subcommand.handle(),
        SubCommand::Dump(subcommand) => subcommand.handle(),
        SubCommand::Restore(subcommand) => subcommand.handle(),
        SubCommand::Migrate(subcommand) => subcommand.handle()
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
