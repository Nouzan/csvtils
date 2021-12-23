use std::path::PathBuf;

use clap_verbosity_flag::Verbosity;
use log::Level;
use structopt::StructOpt;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone, Copy, StructOpt)]
pub(crate) enum Command {
    /// Reverse the records in the csv content.
    Reverse,
}

#[derive(Debug, StructOpt)]
pub(crate) struct Opt {
    #[structopt(flatten)]
    verbose: Verbosity,

    #[structopt(long)]
    pub ignore_header: bool,

    #[structopt(long, short, parse(from_os_str))]
    pub file: PathBuf,

    #[structopt(subcommand)]
    pub cmd: Command,
}

impl Opt {
    #[instrument]
    pub(crate) fn init_from_args() -> anyhow::Result<Self> {
        let mut opt = Opt::from_args();

        // set default log level.
        opt.verbose.set_default(Some(Level::Info));

        let level = match opt.verbose.log_level() {
            Some(Level::Trace) => LevelFilter::TRACE,
            Some(Level::Debug) => LevelFilter::DEBUG,
            Some(Level::Info) => LevelFilter::INFO,
            Some(Level::Warn) => LevelFilter::WARN,
            Some(Level::Error) => LevelFilter::ERROR,
            None => LevelFilter::OFF,
        };
        let fmt = tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(false)
            .compact();
        tracing_subscriber::registry()
            .with(level)
            .with(fmt)
            .try_init()?;
        Ok(opt)
    }
}
