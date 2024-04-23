mod csv;
mod genpass;

use crate::cli::csv::CsvOpts;
use crate::cli::genpass::GenPassOpts;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other format")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

pub use self::csv::OutputFormat;
