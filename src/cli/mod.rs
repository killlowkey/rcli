mod base64;
mod csv;
mod genpass;

use crate::cli::csv::CsvOpts;
use crate::cli::genpass::GenPassOpts;
use clap::Parser;
use std::path::Path;

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

    #[command(subcommand)]
    Base64(Base64Subcommand),
}

// 校验文件是否存在
pub fn verify_input_file(filename: &str) -> anyhow::Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

pub use self::{base64::Base64Format, base64::Base64Subcommand, csv::OutputFormat};

// 运行所有测试：cargo nextest run
// 运行单个测试：cargo nextest run -- test_verify_input_file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
