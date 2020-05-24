use std::io;
use std::process;
use serde::Serialize;
use serde::Deserialize;
use dialoguer::Input;
use dialoguer::Password;
use dialoguer::Confirm;
use super::Database;
use crate::PROJECT_DIRS;
use crate::error::Error;
use crate::guardian::WriteGuardian;
use crate::guardian::ReadGuardian;

lazy_static! {
    static ref DB: sled::Db = {
        let path = PROJECT_DIRS.data_dir().with_file_name("connections");

        sled::open(&path).expect("Could not load connections list")
    };
}

// TODO Use zerocopy types to avoid de/serialization costs
#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub read_only: bool,
    host: String,
    // port: u16,
    username: String,
    password: String,
    use_ssl: bool,
    repl_set_name: Option<String>,
    auth_source: Option<String>
}

impl Server {
    pub fn prompt_details() -> Result<Self, io::Error> {
        eprintln!("ENTER CONNECTION INFO");
        let read_only = Confirm::new()
            .with_prompt("Mark this database read-only?")
            .interact()?;
        let host = Input::<String>::new()
            .with_prompt("Host")
            .interact()?;
        // let port = Input::<u16>::new()
        //     .with_prompt("Port")
        //     .interact()?;
        let username = Input::<String>::new()
            .with_prompt("Username")
            .allow_empty(true)
            .interact()?;
        let password = Password::new()
            .with_prompt("Password (Input hidden)")
            .with_confirmation("Confirm password", "Password mismatch")
            .allow_empty_password(true)
            .interact()?;
        let use_ssl = Confirm::new()
            .with_prompt("Use SSL?")
            .interact()?;
        let replica_set = {
            let value = Input::<String>::new()
                .with_prompt("Replica Set (optional)")
                .allow_empty(true)
                .interact()?;
            
            if value.is_empty() { None } else { Some(value) }
        };
        let auth_source = {
            let value = Input::<String>::new()
                .with_prompt("Auth Source (optional)")
                .allow_empty(true)
                .interact()?;
            
            if value.is_empty() { None } else { Some(value) }
        };
    
        Ok(Server {
            read_only,
            host,
            username,
            password,
            use_ssl,
            repl_set_name: replica_set,
            auth_source
        })
    }

    pub fn load_saved(name: &str) -> Result<Self, Error> {
        let data = DB.get(&name)?
            .ok_or(Error::NoSuchConnection(name.to_owned()))?;
        
        let info = bincode::deserialize::<Self>(&data)?;

        Ok(info)
    }

    pub fn list_saved() -> impl Iterator<Item = Result<(sled::IVec, sled::IVec), sled::Error>> {
        // TODO Parse key and value here
        DB.iter()
    }

    pub fn remove_saved(name: &str) -> Result<sled::IVec, Error> {
        DB.remove(&name)?.ok_or(Error::NoSuchConnection(name.to_owned()))
    }

    pub fn save(&self, name: &str) -> Result<(), Error> {
        let data = bincode::serialize(self)?;
        DB.insert(name, data)?;
        
        // TODO Should we have a global flush on app exit or drop?
        DB.flush()?;

        Ok(())
    }

    pub fn list_databases(&self) -> Result<Vec<String>, Error> {
        // TODO
        Ok(vec!["testdb".to_string(), "testdb2".to_string(), "testdb3".to_string()])
    }

    #[allow(dead_code)]
    pub fn database(self, db_name: String) -> Database {
        Database::select(self, db_name)
    }

    pub fn dump(&self, db_name: &str) -> Result<ReadGuardian, io::Error> {
        let mut cmd = process::Command::new("mongodump");

        match &self.repl_set_name {
            Some(repl_set_name) =>
                cmd.arg(format!("--host={}/{}", repl_set_name, self.host)),
            None =>
                cmd.arg(format!("--host={}", self.host))
        };

        cmd
            .arg(format!("--username={}", self.username))
            .arg(format!("--password={}", self.password))
            .arg(format!("--db={}", db_name))
            .arg("--archive");
        
        if let Some(auth_source) = &self.auth_source {
            cmd.arg(format!("--authenticationDatabase={}", auth_source));
        }
        
        if self.use_ssl {
            cmd.arg("--ssl");
        }

        ReadGuardian::adopt(cmd)
    }

    pub fn restore(&self, destination: &str, source: Option<&str>) -> Result<WriteGuardian, io::Error> {
        let mut cmd = process::Command::new("mongorestore");

        match &self.repl_set_name {
            Some(repl_set_name) =>
                cmd.arg(format!("--host={}/{}", repl_set_name, self.host)),
            None =>
                cmd.arg(format!("--host={}", self.host))
        };

        cmd
            .arg(format!("--username={}", self.username))
            .arg(format!("--password={}", self.password))
            .arg("--archive");
        
        if let Some(auth_source) = &self.auth_source {
            cmd.arg(format!("--authenticationDatabase={}", auth_source));
        }
        
        match source {
            Some(source_name) => {
                cmd
                    .arg(format!("--nsFrom={}.*", source_name))
                    .arg(format!("--nsTo={}.*", destination));
            },
            None => {
                cmd.arg(format!("--nsInclude={}.*", destination));
            }
        }
        
        if self.use_ssl {
            cmd.arg("--ssl");
        }

        WriteGuardian::adopt(cmd)
    }
}