use crate::cli::OutputFormat;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut res = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组迭代器 [(header, record), ...]
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        res.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&res)?,
        OutputFormat::Yaml => serde_yaml::to_string(&res)?,
    };

    fs::write(output, content)?;
    Ok(())
}
