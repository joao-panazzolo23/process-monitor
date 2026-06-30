use std::str::FromStr;

use crate::enums::order_by::OrderBy;
use crate::structs::arguments::AppArguments;
use crate::structs::executer::execute;
use crate::structs::monitor::Monitor;
use clap::Parser;

mod enums;
mod structs;

fn main() {
    let mut args = AppArguments::parse();
    let order_by = OrderBy::from_str(&args.order_by);
    let mut monitor = Monitor::new();

    execute(monitor, &args, order_by);
}
