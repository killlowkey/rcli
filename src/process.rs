use serde::{Deserialize, Serialize};
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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut res = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        res.push(record);
    }

    let json = serde_json::to_string_pretty(&res)?;
    fs::write(output, json)?;
    Ok(())
}
