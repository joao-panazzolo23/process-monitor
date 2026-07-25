use crate::structs::monitor::Monitor;
use slint::{ModelRc, VecModel};

slint::include_modules!();
pub mod structs;

fn main() -> Result<(), slint::PlatformError> {
    let monitor = Monitor::new();
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

    app.set_processes(model);

    app.run()
}
