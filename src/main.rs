#[macro_use] extern crate clap;
#[macro_use] extern crate anyhow;

mod connection;
mod subcommands;
mod guardian;

use connection::ConnectionRepository;
use anyhow::Result;
use directories::ProjectDirs;
use sqlx::prelude::*;

async fn init_path(path: &std::path::Path) -> Result<&std::path::Path> {
    // TODO with_context
    async_std::fs::create_dir_all(path).await?;

    Ok(path)
}

#[async_std::main]
async fn main() -> Result<()> {
    let app = clap_app!(mongodex =>
        (version: "0.1")
        (author: "Noah Shuart <shuart.noah.s@gmail.com>")
        (about: "CLI tool for managing multiple MongoDB databases across \
            multiple servers.")
        (@subcommand connection =>
            (about: "Manage saved database connections")
            (alias: "c")
            (@subcommand list =>
                (about: "List all saved connections"))
            (@subcommand add =>
                (about: "Add a new connection")
                (@arg name: +required))
            (@subcommand remove =>
                (about: "Remove a saved connection")
                (@arg name: +required))
            (@subcommand edit =>
                (about: "Edit a saved connection")
                (@arg name: +required)))
        (@subcommand listdatabases =>
            (about: "List the databases currently present on the specified \
                server by establishing a connection to the server and \
                running the 'listDatabases' command.")
            (alias: "ld")
            (@arg name: +required))
        (@subcommand migrate =>
            (about: "Migrate one database to another")
            (alias: "m")
            (@arg source: +required "Source connection")
            (@arg destination: --to +required +takes_value "Destination \
                connection"))
        (@subcommand dump =>
            (about: "Dump a database toa filesystem [unstable]")
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
            (about: "Opens a shell to the specified connection")
            (@arg connection_name: +required "The name of the connection"))
    );

    let cli_options = app.get_matches();
    let project_directories = ProjectDirs::from("us", "InTheVoid", "Mongodex")
        .expect("No standard app directory available on this platform");
    let data_directory = init_path(project_directories.data_dir());

    let db_connection = async {
        let db_path = data_directory.await?.join("connections.db");
        let mut connection = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .connect()
            .await?;

        // Run migrations against database to ensure schema is valid
        sqlx::migrate!("./migrations")
            .run(&mut connection)
            .await?;

        Ok(connection) as Result<sqlx::SqliteConnection>
    };

    let connections = async {
        let repository = ConnectionRepository::new(db_connection.await?);

        Ok(repository) as Result<ConnectionRepository>
    };

    // TODO SubCommand trait, *SubCommand classes, and a SubCommand factory that
    // unifies the db_connection initialization and stuff

    let mut connections = connections.await?;

    match cli_options.subcommand() {
        ("connection", Some(args)) =>
            subcommands::connection(&mut connections, args).await,
        ("listdatabases", Some(args)) =>
            subcommands::listdatabases(&mut connections, args).await,
        ("migrate", Some(args)) =>
            subcommands::migrate(&mut connections, args).await,
        ("dump", Some(args)) =>
            subcommands::dump(&mut connections, args).await,
        ("restore", Some(args)) =>
            subcommands::restore(&mut connections, args).await,
        ("shell", Some(args)) =>
            subcommands::shell(&mut connections, args).await,
        _ => Ok(())
    }
}
