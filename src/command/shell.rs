use anyhow::Result;
use anyhow::Error;
use anyhow::Context;
use crate::ConnectionRepository;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct ShellCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

#[cfg(target_os = "linux")]
impl Command for ShellCommand {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()> {
        let name = args.value_of("connection_name")
            .with_context(|| "Name argument not provided")?;

        let server = self.connections.get_connection(name)?;
        // This is a bit of a special function in that it only returns if there is
        // an error. This is why the code might look a little weird where it seems
        // to always error. On success the shell() function (which uses execvp
        // internally) does not return.
        let error = server.shell();

        Err(Error::from(error))
    }
}

#[cfg(not(target_os = "linux"))]
impl Command for ShellCommand {
    fn run(&mut self, _args: &clap::ArgMatches) -> Result<()> {
        unimplemented!();
    }
}
