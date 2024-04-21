use anyhow::Result;
use clap::Parser;
use xcli::Opts;

#[tokio::main]
async fn main() -> Result<()>{
    let opts = Opts::parse();
    println!("{:?}",opts);
    Ok(())
}
