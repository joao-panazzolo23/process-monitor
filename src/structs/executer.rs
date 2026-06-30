use crate::{
    enums::order_by::OrderBy,
    structs::{arguments::AppArguments, formatter, monitor::Monitor},
};

pub fn execute(mut monitor: Monitor, args: &AppArguments, order_by: OrderBy) {
    loop {
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
        // 
        // partial_cmp orders 
        match order_by {
            OrderBy::Cpu => processes.sort_by(|a, b| {
                b.cpu_usage
                    .partial_cmp(&a.cpu_usage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            OrderBy::Memory => todo!(),
            OrderBy::Name => todo!(),
            OrderBy::Pid => todo!(),
        }
    }
}
