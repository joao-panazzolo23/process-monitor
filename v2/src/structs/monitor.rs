use crate::structs::{process_info::ProcessInfo, system_info::SystemInfo};
use std::{collections::HashMap, thread::sleep, time::Duration};
use sysinfo::{Pid, Process, ProcessesToUpdate, System};

pub struct Monitor {
    system: System,
}

impl Monitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        sleep(Duration::from_millis(200));
        //system.refresh_all();
        system.refresh_cpu_usage();
        system.refresh_memory();
        system.refresh_processes(ProcessesToUpdate::All, true);
        return Self { system };
    }

    pub fn update(&mut self) {
        self.system.refresh_all();
    }

    pub fn system_info(&self) -> SystemInfo {
        SystemInfo {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            cpus_total_use: self.system.global_cpu_usage(),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            memory_total_gb: self.system.total_memory() as f64 / 1_073_741_824.0,
            memory_used_gb: self.system.used_memory() as f64 / 1_073_741_824.0,
            cpus: self.system.cpus().len() as f32,
        }
    }
    //todo: order by
    pub fn list_processes(&self) -> Vec<ProcessInfo> {
        let processes: &HashMap<Pid, Process> = self.system.processes();

        processes
            .iter()
            .map(|(pid, process_info)| {
                let father_pid = process_info.parent().map(|p| p.as_u32());

                ProcessInfo {
                    name: process_info.name().to_string_lossy().to_string(),
                    pid: pid.as_u32(),
                    cpu_usage: process_info.cpu_usage(),
                    memory_usage: process_info.memory() as f64 / 1_048_576.0, //todo: create global variable/function
                    father_pid: father_pid,
                    status: format!("{:?}", process_info.status()),
                }
            })
            .collect()
    }
}
