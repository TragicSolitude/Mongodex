use dialoguer::Select;
use crate::error::Error;
use crate::guardian::ReadGuardian;
use crate::guardian::WriteGuardian;
use super::Server;

pub struct Database {
    server: Server,
    pub db_name: String
}

impl Database {
    pub fn select(server: Server, db_name: String) -> Self {
        Database { server, db_name }
    }

    pub fn dump(&self) -> Result<ReadGuardian, std::io::Error> {
        self.server.dump(&self.db_name)
    }

    pub fn restore(&self, source: Option<&str>) -> Result<WriteGuardian, std::io::Error> {
        self.server.restore(&self.db_name, source)
    }
}

impl std::str::FromStr for Database {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.rsplitn(2, '@');
        let server = match parts.next() {
            Some(part) => Server::load_saved(part)?,
            None => return Err(Error::NoConnection)
        };
        let items;
        let db_name = match parts.next() {
            Some(part) => part,
            None => {
                items = server.list_databases()?;
                let db = Select::new()
                    .with_prompt("Select a database")
                    .default(0)
                    .items(&items)
                    .interact()?;
            
                &items[db]
            }
        };
        let db_name = db_name.to_owned();

        // TODO Maybe change this to use server.database(db_name) to keep object
        // construction consistent?
        Ok(Database::select(server, db_name))
    }
}

impl std::ops::Deref for Database {
    type Target = Server;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}