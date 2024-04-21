mod csv;

use clap::Parser;

pub use self::{csv::*};

#[derive(Debug,Parser)]
#[command(name = "xcli", version, author, about, long_about = None)]
pub struct Opts {
     #[command(subcommand)]
     pub cmd: SubCommand,
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"),Ok("-".into()));
    }
}