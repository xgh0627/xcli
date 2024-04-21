use std::str::FromStr;
use clap::Parser;
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

    #[arg(short,long,default_value_t = true)]
    header: bool
}

#[derive(Debug,Clone,Copy)]
enum OutPutFormat {
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