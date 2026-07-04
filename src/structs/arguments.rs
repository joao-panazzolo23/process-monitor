///Derive is a macro Decorator that generates code by itself. It needs to be imported at cargo.toml
use clap::Parser;

#[derive(Parser, Debug)]
pub struct AppArguments {
    #[arg(index = 1)]
    pub(crate) limit: usize,
    #[arg(index = 2)]
    pub(crate) order_by: String,
    #[arg(index = 3)]
    pub(crate) interval: u64,
    #[arg(index = 4)]
    pub(crate) filter_by: Option<String>,
}
