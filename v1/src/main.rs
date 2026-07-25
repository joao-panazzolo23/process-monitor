use std::str::FromStr;

use crate::enums::order_by::OrderBy;
use crate::structs::arguments::AppArguments;
use crate::structs::executer::execute;
use crate::structs::monitor::Monitor;
use clap::Parser;

mod enums;
mod helpers;
mod structs;

fn main() {
    let args = AppArguments::parse();
    // let order_by = match OrderBy::from_str(&args.order_by) {
    //     Ok(value) => value,
    //     Err(error) => error,
    // };
    let order_by = OrderBy::from_str(&args.order_by).unwrap();
    let monitor = Monitor::new();

    execute(monitor, &args, order_by);
}
