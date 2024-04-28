use std::{fmt::Display, str::FromStr};
use clap::Parser;
use crate::CmdExector;

use super::verify_file;

#[derive(Debug,Parser)]
pub struct CsvOpts {
    #[arg(short,long,value_parser = verify_file)]
    input: String,

    #[arg(long,value_parser = parse_format,default_value = "json")]
    format: OutPutFormat,

    #[arg(short,long)]
    output: Option<String>,

    #[arg(short,long,default_value_t = ',')]
    delimiter: char,

    #[arg(long,default_value_t = true)]
    header: bool
}

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        }else{
            format!("output.{}",self.format)
        };
        crate::process_csv(&self.input,output,self.format)?;
        Ok(())
    }
}

#[derive(Debug,Clone,Copy)]
pub enum OutPutFormat {
    Json,
    Yaml
}

fn parse_format(format: &str) -> Result<OutPutFormat,anyhow::Error> {
    format.parse()
}

impl FromStr for OutPutFormat {
    
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutPutFormat::Json),
            "yaml" => Ok(OutPutFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format"))
        }
    }

}

impl Display for OutPutFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<OutPutFormat> for &'static str {
    fn from(value: OutPutFormat) -> Self {
        match value {
            OutPutFormat::Json => "json",
            OutPutFormat::Yaml => "yaml",
        }
    }
}