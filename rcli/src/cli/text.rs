use crate::cli::{verify_file, verify_path};
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(about = "Sign a message with private key")]
    Sign(TextSignOpts),

    #[command(about = "Verify a signed message ")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Copy, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

// 将字符串转为 TextSignFormat
fn parse_text_sign_format(format: &str) -> anyhow::Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

// 实现了 From，也会自动实现 Into，可以将 OutputFormat 转为 &str
impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

// 从 &str 转为 Base64Format
impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // <&'static str> 将转换后类型限定为 &str
        // *self 需要实现 Copy Trait
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
