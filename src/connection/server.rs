use std::fmt::Display;
use std::io;
use std::ops::Deref;
use serde::Serialize;
use serde::Deserialize;
use dialoguer::Confirm;
use dialoguer::Password;
use dialoguer::Input;
use std::process;
use colored::*;

pub struct ServerList(pub Vec<Server>);

impl Deref for ServerList {
    type Target = Vec<Server>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ServerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = format!("{: <10}\t{: <70}", "Name", "Host");
        writeln!(f, "{}", header.bold())?;
        for server in self.0.iter() {
            writeln!(f, "{: <10}\t{: <70}", server.name, server.host)?;
        }

        Ok(())
    }
}

impl From<Vec<Server>> for ServerList {
    fn from(list: Vec<Server>) -> Self {
        ServerList(list)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Server {
    pub name: String,
    pub read_only: bool,
    pub(super) host: String,
    pub(super) username: String,
    pub(super) password: String,
    pub(super) use_ssl: bool,
    pub(super) repl_set_name: Option<String>,
    pub(super) auth_source: Option<String>
}

impl From<&Server> for mongodb::options::ClientOptions {
    fn from(server: &Server) -> Self {
        let hosts = server.host
            .split(',')
            .map(|host| mongodb::options::StreamAddress::parse(host).unwrap())
            .collect::<Vec<_>>();
        // Default options
        // TODO Figure out how to avoid all these clones
        let mut options = mongodb::options::ClientOptions::builder()
            .hosts(hosts)
            // Any way to avoid the clone here?
            .repl_set_name(server.repl_set_name.clone())
            .build();

        if !server.username.is_empty() {
            let credential = mongodb::options::Credential::builder()
                .username(server.username.clone())
                .password(server.password.clone())
                .source(server.auth_source.clone())
                .build();

            options.credential = Some(credential);
        }

        if server.use_ssl {
            let tls_options = mongodb::options::TlsOptions::default();
            let tls = mongodb::options::Tls::Enabled(tls_options);

            options.tls = Some(tls);
        }

        options
    }
}

impl Display for Server {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO Change this format to be a more verbose output and use a
        // dedicated method for printing a list of Servers
        todo!();
    }
}

impl Server {
    pub fn connect(&self) -> Result<mongodb::Client, mongodb::error::Error> {
        mongodb::Client::with_options(self.into())
    }

    #[cfg(target_os = "linux")]
    pub fn shell(self) -> io::Error {
        // TODO Come up with cross-platform implementation. Hopefully without
        // resorting to spawning child processes. I like that execvp totally
        // replaces the existing process.
        use std::os::unix::process::CommandExt;

        let mut command = process::Command::new("mongo");

        match &self.repl_set_name {
            Some(repl_set_name) =>
                command.arg(format!("--host={}/{}", repl_set_name, self.host)),
            None =>
                command.arg(format!("--host={}", self.host))
        };

        if !self.username.is_empty() {
            command
                .arg(format!("--username={}", self.username))
                .arg(format!("--password={}", self.password));
        }

        if let Some(auth_source) = &self.auth_source {
            command.arg(format!("--authenticationDatabase={}", auth_source));
        }

        if self.use_ssl {
            command.arg("--ssl");
        }

        command.exec()
    }

    pub fn prompt_details<T: Into<String>>(name: T) -> Result<Self, io::Error> {
        eprintln!("ENTER CONNECTION INFO");
        let read_only = Confirm::new()
            .with_prompt("Mark this server read only?")
            .interact()?;
        let host = Input::<String>::new()
            .with_prompt("Host")
            .interact()?;
        // let port = Input::<u16>::new()
            // .with_prompt("Port")
            // .interact()?;
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
        let repl_set_name = {
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
            name: name.into(),
            read_only,
            host,
            username,
            password,
            use_ssl,
            repl_set_name,
            auth_source
        })
    }

    pub fn prompt_update_details(&mut self) -> Result<(), io::Error> {
        eprintln!("ENTER CONNECTION INFO");
        self.read_only = Confirm::new()
            .default(self.read_only)
            .with_prompt("Mark this server read only?")
            .interact()?;
        self.host = Input::<String>::new()
            .default(self.host.clone())
            .with_prompt("Host")
            .interact()?;
        // let port = Input::<u16>::new()
            // .default(self.port)
            // .with_prompt("Port")
            // .interact()?;
        self.username = Input::<String>::new()
            .default(self.username.clone())
            .with_prompt("Username")
            .allow_empty(true)
            .interact()?;
        let new_password = Password::new()
            .with_prompt("Password (Input hidden, leave empty to keep existing)")
            .with_confirmation("Confirm password", "Password mismatch")
            .allow_empty_password(true)
            .interact()?;
        if !new_password.is_empty() {
            self.password = new_password;
        }
        self.use_ssl = Confirm::new()
            .default(self.use_ssl)
            .with_prompt("Use SSL?")
            .interact()?;
        self.repl_set_name = {
            let value = match self.repl_set_name {
                Some(ref repl_set_name) => Input::<String>::new()
                    .default(repl_set_name.clone())
                    .with_prompt("Replica Set (optional)")
                    .allow_empty(true)
                    .interact()?,
                None => Input::<String>::new()
                    .with_prompt("Replica Set (optional)")
                    .allow_empty(true)
                    .interact()?
            };

            if value.is_empty() { None } else { Some(value) }
        };
        self.auth_source = {
            let value = match self.auth_source {
                Some(ref auth_source) => Input::<String>::new()
                    .default(auth_source.clone())
                    .with_prompt("Auth Source (optional)")
                    .allow_empty(true)
                    .interact()?,
                None => Input::<String>::new()
                    .with_prompt("Auth Source (optional)")
                    .allow_empty(true)
                    .interact()?
            };

            if value.is_empty() { None } else { Some(value) }
        };

        Ok(())
    }
}
