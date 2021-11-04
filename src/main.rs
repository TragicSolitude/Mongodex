#[macro_use] extern crate clap;
#[macro_use] extern crate anyhow;
#[macro_use] extern crate lazy_static;

use std::fs;
use std::path;
use shaku::HasProvider;
use anyhow::Result;

mod connection;
mod guardian;
mod sqlite;
mod command;

pub use connection::Server;
pub use connection::Database;
pub use connection::ConnectionRepository;

lazy_static! {
    static ref PROJECT_DIRECTORIES: directories::ProjectDirs =
        directories::ProjectDirs::from("us", "InTheVoid", "Mongodex")
            .expect("No standard app directory available on this platform");

    static ref DATA_DIRECTORY: &'static path::Path = {
        let path = PROJECT_DIRECTORIES.data_dir();
        fs::create_dir_all(path)
            .expect("Could not create data directory");

        path
    };

    static ref RUNTIME: tokio::runtime::Runtime =
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("Failed to initialize runtime");
}

shaku::module! {
    AppModule {
        components = [],
        providers = [
            sqlite::SqliteConnection,
            connection::SqliteConnectionRepository,
            command::NullCommand
        ]
    }
}

use command::Command;

fn main() -> Result<()> {
    // TODO SSH Tunneling
    let cli_app = clap_app!(mongodex =>
        (version: "0.1")
        (author: "Noah Shuart <shuart.noah.s@gmail.com>")
        (about: "CLI tool for managing multiple MongoDB databases")
        (@subcommand connection =>
            (about: "Manage saved database connections")
            (alias: "c")
            (@subcommand list =>
                (about: "List all saved connections"))
            (@subcommand show =>
                (about: "Show details about a connection")
                (@arg name: +required))
            (@subcommand add =>
                (about: "Add a new connection")
                (@arg name: +required))
            (@subcommand remove =>
                (about: "Remove a saved connection")
                (@arg name: +required))
            (@subcommand edit =>
                (about: "Edit a saved connection")
                (@arg name: +required)))
        (@subcommand tunnel =>
            (about: "Manage saved tunnels")
            (alias: "t")
            (@subcommand list =>
                (about: "List all saved tunnels")))
        (@subcommand listdatabases =>
            (about: "List the databases currently present on the specified \
                server by establishing a connection to the server and \
                running the 'listDatabases' command.\
                \
                NOTE: Due to some bugs in the MongoDB driver for Rust, this \
                command doesn't work for Atlas instances above M2 unless the \
                instance is running at least version 4.2.")
            (alias: "ld")
            (@arg name: +required))
        (@subcommand migrate =>
            (about: "Migrate one database to another")
            (alias: "m")
            (@arg source: +required "Source connection")
            (@arg destination: --to +required +takes_value "Destination \
                connection"))
        (@subcommand dump =>
            (about: "Dump a database to a filesystem [unstable]")
            (alias: "b")
            (@arg source: +required "Which saved connection to use. This \
                should be specified in the format [database@]saved-connection \
                where a specific database can be provided. On connections with \
                the database name saved, the database provided here is \
                ignored. If not saved and not provided here, the database name \
                will be prompted from a list of databases available on the \
                server.")
            (@arg destination_file: +required "The destination file path for \
                the dump. If the file doesn't exist it will be created \
                otherwise it is truncated before dumping the database."))
        (@subcommand restore =>
            (about: "Restore a database from a dump [unstable]")
            (alias: "s")
            (@arg dump_file: +required "File to restore, created by the dump \
                command. Using a file not created by the dump command is \
                undefined behavior; do so at your own risk.")
            (@arg destination: +required "The database to restore to")
            (@arg from: -f --from +takes_value "If restoring to a different \
                database (by name) then specify the name of the original \
                database that was backed up."))
        (@subcommand shell =>
            (about: "Opens a shell to the specified connection (Linux Only)")
            (@arg connection_name: +required "The name of the connection"))
    );

    let args = cli_app.get_matches();
    let (command, subargs) = command::get_command(&args);
    let app = AppModule::builder()
        .with_provider_override::<dyn Command>(command)
        .build();
    let mut command: Box<dyn Command> = app.provide().unwrap_or_default();

    command.run(subargs)
}
