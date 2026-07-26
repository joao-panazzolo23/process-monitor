use crate::{
    enums::order_by::OrderBy,
    helpers::clean_monitor::clean_monitor,
    structs::{arguments::AppArguments, formatter, monitor::Monitor},
};
use std::time::Duration;

pub fn execute(mut monitor: Monitor, args: &AppArguments, order_by: OrderBy) {
    loop {
        clean_monitor();
        monitor.update();

        let system_info = &monitor.system_info();
        formatter::show_system_info(system_info);

        let mut processes = monitor.list_processes();

        if let Some(ref term) = args.filter_by {
            let term_lower = term.to_lowercase();
            processes.retain(|p| p.name.to_lowercase().contains(&term_lower));
        }

        //|a, b| is a closure. Equivalent to writing a function that returns the type evaluated.
        // When using Sort By, the signature is &mut self, mut compare: F
        // partial_cmp orders

        formatter::show_processes(&processes, args.limit);

        println!(
            "\nTotal: {} processes | Showing {}",
            processes.len(),
            processes.len().min(args.limit)
        );

        if args.interval == 0 {
            break;
        }

        println!("Updating in  {}s... Ctrl + C to stop.", args.interval);

        std::thread::sleep(Duration::from_secs(args.interval));
    }
}
