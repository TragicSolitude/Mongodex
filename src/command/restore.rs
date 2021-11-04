use std::io;
use std::fs;
use anyhow::Result;
use anyhow::Context;
use crate::ConnectionRepository;
use crate::Database;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct RestoreCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

impl Command for RestoreCommand {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()> {
        let dump_file_path = args.value_of("dump_file")
            .with_context(|| "Dump file path was not given")?;
        let destination_name = args.value_of("destination")
            .with_context(|| "Destination was not specified")?;
        let from = args.value_of("from");

        let destination = Database::from_str(self.connections.as_mut(), destination_name)?;

        if destination.read_only {
            return Err(anyhow!("Destination is read only"));
        }

        let num_bytes_copied = {
            let mut file = fs::File::open(dump_file_path)?;
            let mut guardian = destination.restore(from.as_deref())?;

            io::copy(&mut file, guardian.input())?
        };

        println!("Wrote {} bytes", num_bytes_copied);

        Ok(())
    }
}
