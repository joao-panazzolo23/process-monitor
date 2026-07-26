slint::include_modules!();
use crate::{enums::order_by::OrderBy, structs::monitor::Monitor};
use slint::{Model, ModelRc, VecModel};
pub mod enums;
pub mod structs;

///TODO: refactor. this is messy af.
fn main() -> Result<(), slint::PlatformError> {
    let mut monitor = Monitor::new();
    let app = AppWindow::new()?;

    let ui_processes: Vec<Process> = monitor
        .list_processes()
        .into_iter()
        .map(|p| Process {
            pid: p.pid as i32,
            name: p.name.into(),
            cpu_usage: p.cpu_usage,
            memory_usage: p.memory_usage as f32,
        })
        .collect();

    let model = ModelRc::new(VecModel::from(ui_processes));
    app.set_processes(ModelRc::from(model.clone()));

    let model = model.clone();

    //this is very messy. i should create another function just to use it here
    app.on_sort_requested(move |column| {
        monitor.update();

        let mut processes = monitor.list_processes();

        let order_by = OrderBy::try_from(column).unwrap();

        match order_by {
            OrderBy::Cpu => processes.sort_by(|a, b| {
                b.cpu_usage
                    .partial_cmp(&a.cpu_usage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            OrderBy::Memory => processes.sort_by(|a, b| {
                b.memory_usage
                    .partial_cmp(&a.memory_usage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            OrderBy::Name => processes.sort_by(|a, b| {
                b.name
                    .partial_cmp(&a.name)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            OrderBy::Pid => processes.sort_by(|a, b| {
                b.pid
                    .partial_cmp(&a.pid)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
        };

        for (i, p) in processes.iter().enumerate() {
            model.set_row_data(
                i,
                Process {
                    pid: p.pid as i32,
                    name: p.name.clone().into(),
                    cpu_usage: p.cpu_usage,
                    memory_usage: p.memory_usage as f32,
                },
            );
        }
    });

    app.run()
}
