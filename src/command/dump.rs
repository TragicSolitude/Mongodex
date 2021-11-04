use std::fs;
use std::io;
use anyhow::Result;
use anyhow::Context;
use crate::ConnectionRepository;
use crate::Database;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct DumpCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

impl Command for DumpCommand {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()> {
        let source_str = args.value_of("source")
            .with_context(|| "Connection target not specified")?;
        let destination_file = args.value_of("destination_file")
            .with_context(|| "Destination file path not given")?;
        let connection_target = Database::from_str(self.connections.as_mut(), source_str)?;
        let num_bytes_copied = {
            let mut file = fs::File::create(&destination_file)?;
            let mut guardian = connection_target.dump()?;

            io::copy(guardian.output(), &mut file)?
        };

        println!("Wrote {} bytes", num_bytes_copied);

        // TODO Link dumps to connections somehow

        // TODO Implement some kind of compression for the stored file. Perhaps
        // this can be gzip to maintain compatibility with mongorestore

        // TODO Implement encryption for the stored file. This should use a
        // randomly generated key saved to the connection. Connections can
        // optionally also be encrypted using a key derived from a user-input
        // password.

        Ok(())
    }
}
