use std::fs;

use serde::{Deserialize, Serialize};
use crate::cli::OutPutFormat;
use anyhow::Result;
use csv::Reader;

#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
   name: String,
   position: String,
   #[serde(rename="DOB")]
   dob: String,
   nationality: String,
   #[serde(rename = "Kit Number")]
   kit: u8
}

pub fn process_csv(input: &str,output: String,format: OutPutFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for results in reader.records() {
        let record = results?;
        let json_valuse = headers.iter().zip(record.iter())
        .collect::<serde_json::Value>();
        ret.push(json_valuse);
    }
    let content = match format {
        OutPutFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutPutFormat::Json => serde_json::to_string_pretty(&ret)?
    };
    fs::write(output, content)?;
    Ok(())
}