use std::io;
use std::process;
use os_pipe::pipe;
use os_pipe::PipeReader;
use os_pipe::PipeWriter;

struct Guardian(std::process::Child);

impl std::ops::Drop for Guardian {
    fn drop(&mut self) {
        self.0.wait().expect("Put the cart before the horse");
    }
}

pub struct WriteGuardian {
    // PipeWriter MUST be dropped before guardian otherwise child process will
    // be stuck blocking waiting for the pipe to drop
    writer: PipeWriter,
    // Store guardian here to enforce lifetime equal to writer
    _guardian: Guardian
}

impl WriteGuardian {
    /// TODO Docs
    pub fn adopt(mut cmd: process::Command) -> Result<Self, io::Error> {
        let writer = {
            let (reader, writer) = pipe()?;
            cmd.stdin(reader);

            writer
        };
        let child = cmd.spawn()?;

        Ok(WriteGuardian { _guardian: Guardian(child), writer })
    }

    pub fn input(&mut self) -> &mut impl std::io::Write {
        &mut self.writer
    }
}

pub struct ReadGuardian {
    // PipeReader MUST be dropped before guardian otherwise child process will
    // be stuck blocking waiting for the pipe to drop
    reader: PipeReader,
    // Store guardian here to enforce lifetime equal to reader
    _guardian: Guardian
}

impl ReadGuardian {
    /// TODO Docs
    pub fn adopt(mut cmd: process::Command) -> Result<Self, io::Error> {
        let reader = {
            let (reader, writer) = pipe()?;
            cmd.stdout(writer);

            reader
        };
        let child = cmd.spawn()?;

        Ok(ReadGuardian { _guardian: Guardian(child), reader })
    }

    pub fn output(&mut self) -> &mut impl std::io::Read {
        &mut self.reader
    }
}
