mod cli;

use crate::cli::{Command, Opt};
use anyhow::Result;
use csv::{ReaderBuilder, WriterBuilder};

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate tracing_attributes;

fn main() -> Result<()> {
    let opt = Opt::init_from_args()?;
    debug!("opt={:?}", opt);
    match opt.cmd {
        Command::Reverse => {
            let mut reader = ReaderBuilder::default()
                .has_headers(!opt.ignore_header)
                .from_path(&opt.file)?;
            let mut writer = WriterBuilder::default().from_writer(std::io::stdout());
            let mut stack = Vec::new();
            if !opt.ignore_header {
                let header = reader.headers()?;
                writer.write_record(header)?;
            }
            for record in reader.records() {
                stack.push(record?);
            }
            while let Some(record) = stack.pop() {
                writer.write_record(&record)?;
            }
        }
    }
    Ok(())
}
