use colored::Colorize;

use crate::structs::{process_info::ProcessInfo, system_info::SystemInfo};

pub fn show_system_info(info: &SystemInfo) {
    //cleans the screen
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", "=== Rust Process Monitor ===".green().bold());
    println!(
        "SO:{} | Kernel:{} | CPUs: {}",
        info.os_name.cyan(),
        info.kernel_version.cyan(),
        info.cpus.to_string()
    );

    let cpu_bar = build_bar(info.cpus_total_use as f64, 100.0, 30);

    println!("CPU:      {} {:.1}%", cpu_bar, info.cpus_total_use);

    let percentage_mem = (info.memory_used_gb / info.memory_total_gb) * 100.0;
    let memory_bar = build_bar(info.memory_used_gb, info.memory_total_gb, 30);

    println!(
        "Memoria: {} {:.1}/{:.1} GB ({:.1}%)",
        memory_bar, info.memory_used_gb, info.memory_total_gb, percentage_mem
    );
    println!();
}

fn build_bar(value: f64, maximum: f64, width: usize) -> String {
    let percentage = (value / maximum).min(1.0);
    let filled = (percentage * width as f64) as usize;
    let empty = width - filled;

    let text_bar = format!("[{}{}]", "#".repeat(filled), " ".repeat(empty));

    if percentage > 0.8 {
        text_bar.red().to_string()
    } else if percentage > 0.5 {
        text_bar.yellow().to_string()
    } else {
        // text_bar.green().to_string(); using an ; makes it a statement.
        // In Rust, IFs are a EXPRESSION.
        text_bar.green().to_string()
    }
}

pub fn show_processes(processes: &[ProcessInfo], limit: usize) {
    println!(
        "{:>7} {:<25} {:>8} {:>10} {:>10}",
        "PID".white().bold(),
        "NAME".white().bold(),
        "CPU".white().bold(),
        "MEM (MB)".white().bold(),
        "STATUS".white().bold()
    );
    println!("{}", "-".repeat(65).dimmed());

    for proc in processes.iter().take(limit) {
        let cpu_color = if proc.cpu_usage > 50.0 {
            format!("{:>7.1}", proc.cpu_usage).red().bold().to_string()
        } else if proc.cpu_usage > 10.0 {
            format!("{:>7.1}", proc.cpu_usage).red().bold().to_string()
        } else {
            format!("{:>7.1}", proc.cpu_usage).red().bold().to_string()
        };

        let mem_color = if proc.memory_usage > 500.0 {
            format!("{:>9.1}", proc.memory_usage).red().to_string()
        } else if proc.memory_usage > 100.0 {
            format!("{:>9.1}", proc.memory_usage).yellow().to_string()
        } else {
            format!("{:>9.1}", proc.memory_usage).green().to_string()
        };

        let name = if proc.name.len() > 24 {
            format!("{}...", &proc.name[..21])
        } else {
            proc.name.clone()
        };

        println!(
            "{:>7} {:<25} {} {} {:>10}",
            proc.pid,
            name.cyan(),
            cpu_color,
            mem_color,
            proc.status.dimmed()
        );

        println!("{}", "-".repeat(65).dimmed());
    }
}
