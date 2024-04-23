use crate::cli::verify_input_file;
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode a base64 string")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

// 将字符串转为 Base64Format
fn parse_base64_format(format: &str) -> anyhow::Result<Base64Format, anyhow::Error> {
    format.parse()
}

// 实现了 From，也会自动实现 Into，可以将 OutputFormat 转为 &str
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

// 从 &str 转为 Base64Format
impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // <&'static str> 将转换后类型限定为 &str
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
