use std::process;
use dialoguer::Select;
use anyhow::Result;
use crate::guardian::ReadGuardian;
use crate::guardian::WriteGuardian;
use crate::RUNTIME;
use super::ConnectionRepository;
use super::Server;

pub struct Database {
    // TODO Consider making this a reference so that a .list_databases method
    // could return a list of "Database" instances for some nice ergonomic code.
    // There's no real requirement that Database needs to own the "Server"
    // instance.
    server: Server,
    pub db_name: String
}

impl Database {
    pub fn select(server: Server, db_name: String) -> Self {
        Database { server, db_name }
    }

    // I wonder if I could make some nifty ergnomic interface for this that
    // would only need the ConnectionRepository instance when the database name
    // isn't given
    //
    // TODO Change this to some kind of parsing framework that can support
    // Database instances not owning the Server instance they are related to.
    pub fn from_str(connections: &mut dyn ConnectionRepository, input: &str) -> Result<Self> {
        // TODO maybe use rsplit_once when it is stabilized?
        // let (server_name, db_name) = input.rsplit_once('@')
        //     .with_context(|| "No database specified")?;
        let mut parts = input.rsplitn(2, '@');
        let server_name = parts.next()
            .ok_or_else(|| anyhow!("No connection given"))?;
        let server = connections.get_connection(server_name)?;
        let database_names;
        // TODO change to .ok_or_else once I can figure out the error types a
        // bit better.
        let db_name = match parts.next() {
            Some(part) => part,
            None => {
                let server_connection = server.connect()?;
                database_names = RUNTIME.block_on(
                    server_connection.list_database_names(None, None))?;
                let db = Select::new()
                    .with_prompt("Select a database")
                    .default(0)
                    .items(&database_names)
                    .interact()?;

                &database_names[db]
            }
        };
        let db_name = db_name.to_owned();

        Ok(Database::select(server, db_name))
    }

    pub fn dump(&self) -> Result<ReadGuardian, std::io::Error> {
        let mut command = process::Command::new("mongodump");

        match &self.server.repl_set_name {
            Some(repl_set_name) =>
                command.arg(format!("--host={}/{}", repl_set_name, self.server.host)),
            None =>
                command.arg(format!("--host={}", self.server.host))
        };

        command
            .arg(format!("--username={}", self.server.username))
            .arg(format!("--password={}", self.server.password))
            .arg(format!("--db={}", self.db_name))
            .arg("--archive");

        if let Some(ref auth_source) = self.server.auth_source {
            command.arg(format!("--authenticationDatabase={}", auth_source));
        }

        if self.server.use_ssl {
            command.arg("--ssl");
        }

        ReadGuardian::adopt(command)
    }

    pub fn restore(&self, source: Option<&str>) -> Result<WriteGuardian, std::io::Error> {
        let mut command = process::Command::new("mongorestore");

        match self.server.repl_set_name {
            Some(ref repl_set_name) =>
                command.arg(format!("--host={}/{}", repl_set_name, self.server.host)),
            None =>
                command.arg(format!("--host={}", self.server.host))
        };

        command
            .arg(format!("--username={}", self.server.username))
            .arg(format!("--password={}", self.server.password))
            .arg("--archive")
            // This should probably be provided as an option in mongodex
            .arg("--drop");

        if let Some(ref auth_source) = self.auth_source {
            command.arg(format!("--authenticationDatabase={}", auth_source));
        }

        match source {
            Some(source_name) => {
                command
                    .arg(format!("--nsFrom={}.*", source_name))
                    .arg(format!("--nsTo={}.*", self.db_name));
            },
            None => {
                command.arg(format!("--nsInclude={}", self.db_name));
            }
        }

        if self.server.use_ssl {
            command.arg("--ssl");
        }

        WriteGuardian::adopt(command)
    }
}

impl std::ops::Deref for Database {
    type Target = Server;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}
