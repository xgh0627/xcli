use clap::Parser;

#[derive(Debug,Parser)]
pub struct CsvOpts {
    #[arg(short,long)]
    input: String,

    #[arg(short,long,default_value = "output.json")]
    output: String,

    delimiter: char,

    header: bool
}