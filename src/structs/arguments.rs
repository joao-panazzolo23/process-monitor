///Derive is a macro Decorator that generates code by itself. It needs to be imported at cargo.toml
use clap::Parser;

#[derive(Parser)]
pub struct AppArguments {
    pub(crate) limit: usize,
    pub(crate) order_by: String,
    pub(crate) filter_by: Option<String>,
    pub(crate) interval: u64,
}
