use dialoguer::Select;
use crate::error::Error;
use crate::connection_info::ConnectionInfo;
use crate::guardian::ReadGuardian;
use crate::guardian::WriteGuardian;

pub struct ConnectionTarget {
    info: ConnectionInfo,
    pub db_name: String
}

impl std::str::FromStr for ConnectionTarget {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.rsplitn(2, '@');
        let info = match parts.next() {
            Some(part) => ConnectionInfo::load_saved(part)?,
            None => return Err(Error::NoConnection)
        };
        let items;
        let db_name = match parts.next() {
            Some(part) => part,
            None => {
                items = info.list_databases()?;
                let db = Select::new()
                    .with_prompt("Select a database")
                    .default(0)
                    .items(&items)
                    .interact()?;
            
                &items[db]
            }
        };
        let db_name = db_name.to_owned();

        Ok(ConnectionTarget { info, db_name })
    }
}

impl ConnectionTarget {
    pub fn dump(&self) -> Result<ReadGuardian, std::io::Error> {
        self.info.dump(&self.db_name)
    }

    pub fn restore(&self, destination: Option<&str>) -> Result<WriteGuardian, std::io::Error> {
        self.info.restore(&self.db_name, destination)
    }
}

impl std::ops::Deref for ConnectionTarget {
    type Target = ConnectionInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}