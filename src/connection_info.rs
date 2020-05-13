use std::io;
use std::process;
use serde::Serialize;
use serde::Deserialize;
use dialoguer::Input;
use dialoguer::Password;
use dialoguer::Confirm;
use crate::error::Error;
use crate::guardian::ReadGuardian;
use crate::guardian::WriteGuardian;

lazy_static! {
    static ref DB: sled::Db = sled::open("./data/connections")
        .expect("Could not load connections list");
}

// TODO Use zerocopy types to avoid de/serialization costs
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionInfo {
    host: String,
    // port: u16,
    username: String,
    password: String,
    use_ssl: bool
    // TODO option<Replica set>, option<authsource>, option<db name>
}

impl ConnectionInfo {
    pub fn prompt() -> Result<Self, io::Error> {
        eprintln!("ENTER CONNECTION INFO");
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
    
        Ok(ConnectionInfo { host, username, password, use_ssl })
    }

    pub fn load_saved(name: &str) -> Result<Self, Error> {
        let data = DB.get(&name)?
            .ok_or(Error::NoSuchConnection(name))?;
        
        let info = bincode::deserialize::<Self>(&data)?;

        Ok(info)
    }

    pub fn list() -> impl Iterator<Item = Result<(sled::IVec, sled::IVec), sled::Error>> {
        // TODO Parse key and value here
        DB.iter()
    }

    pub fn remove(name: &str) -> Result<sled::IVec, Error> {
        DB.remove(&name)?.ok_or(Error::NoSuchConnection(name))
    }

    pub fn save<'a>(&self, name: &str) -> Result<(), Error<'a>> {
        let data = bincode::serialize(self)?;
        DB.insert(name, data)?;

        Ok(())
    }

    pub fn list_databases<'a>(&self) -> Result<Vec<String>, Error<'a>> {
        // TODO
        Ok(vec!["testdb".to_string(), "testdb2".to_string(), "testdb3".to_string()])
    }

    pub fn dump(&self, db: &str) -> Result<ReadGuardian, io::Error> {
        // TODO Replica set and authentication database
        let mut cmd = process::Command::new("mongodump");

        // TODO Move into reusable function
        cmd
            .arg(format!("--host={}", self.host))
            .arg(format!("--username={}", self.username))
            .arg(format!("--password={}", self.password))
            .arg(format!("--db={}", db))
            .arg("--archive");
        
        if self.use_ssl {
            cmd.arg("--ssl");
        }

        ReadGuardian::adopt(cmd)
    }

    pub fn restore(&self, source: &str, dest: Option<&str>) -> Result<WriteGuardian, io::Error> {
        let mut cmd = process::Command::new("mongorestore");

        cmd
            .arg(format!("--host={}", self.host))
            .arg(format!("--username={}", self.username))
            .arg(format!("--password={}", self.password))
            .arg("--archive");
        
        match dest {
            Some(dest_name) => {
                cmd
                    .arg(format!("--nsFrom={}.*", source))
                    .arg(format!("--nsTo={}.*", dest_name));
            },
            None => {
                cmd.arg(format!("--nsInclude={}.*", source));
            }
        }
        
        if self.use_ssl {
            cmd.arg("--ssl");
        }

        WriteGuardian::adopt(cmd)
    }
}