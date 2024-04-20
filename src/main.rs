use anyhow::Result;
use clap::Parser;
fn main() -> Result<()>{
    let opts = Opts::parse();
    println!("{:?}",opts);
    Ok(())
}

#[derive(Debug,Parser)]
#[command(version,about,name = "xcli",long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug,Parser)]
enum SubCommand {
    #[command(name = "csv",about = "show csv or convert to other formats")]
    Csv(CsvOpts)
}

#[derive(Debug,Parser)]
struct CsvOpts {
    #[arg(short,long)]
    input: String,

    #[arg(short,long,default_value = "output.json")]
    output: String,

    #[arg(short,long,default_value_t = ',')]
    delimiter: char,

    #[arg(long,default_value_t = true)]
    header: bool,
}
