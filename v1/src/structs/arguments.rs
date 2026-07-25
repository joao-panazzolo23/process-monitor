///Derive is a macro Decorator that generates code by itself. It needs to be imported at cargo.toml
use clap::Parser;

#[derive(Parser, Debug)]
pub struct AppArguments {
    #[arg(long)]
    pub(crate) limit: usize,
    #[arg(long)]
    pub(crate) order_by: String,
    #[arg(long)]
    pub(crate) interval: u64,
    #[arg(long)]
    pub(crate) filter_by: Option<String>,
}
