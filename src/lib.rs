mod cli;
mod process;

pub use cli::*;
pub use process::*;


#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
