mod csv;

use clap::Parser;

pub use self::{csv::*};

#[derive(Debug,Parser)]
#[command(name = "xcli", version, author, about, long_about = None)]
struct Opts {
     #[command(subcommand)]
     cmd: SubCommand,
}

#[derive(Debug,Parser)]
enum SubCommand {
    #[command(name = "csv",about = "show csv or convert csv to other formats")]
    Csv(CsvOpts),
}

fn verify_file(filename: &str) -> Result<String,&'static str> {
    if filename.len() == 0 || filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    }else{
        Err("file not found")
    }
}