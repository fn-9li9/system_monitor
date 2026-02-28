use std::io::{self, Write};
use std::{thread, time::Duration};
use sysinfo::{Disks, Networks, System};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn bar(value: f32, max: f32, width: usize) -> String {
    let filled = ((value / max) * width as f32) as usize;
    let empty = width.saturating_sub(filled);
    let color = if value / max > 0.8 {
        "\x1b[31m"
    } else if value / max > 0.5 {
        "\x1b[33m"
    } else {
        "\x1b[32m"
    };
    format!(
        "{}[{}{}]\x1b[0m",
        color,
        "#".repeat(filled),
        ".".repeat(empty)
    )
}

fn format_bytes(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn main() {
    let mut sys = System::new_all();
    let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();

    println!("\x1b[36m+======================================+");
    println!("|     System Monitor in Rust           |");
    println!("+======================================+\x1b[0m");
    println!("Starting... (Ctrl+C to exit)\n");
    thread::sleep(Duration::from_secs(1));

    loop {
        sys.refresh_all();
        disks.refresh(true);
        networks.refresh(true);

        clear_screen();

        // -- header
        println!("\x1b[36m+-----------------------------------------------------+");
        println!("|           System Monitor - Rust                     |");
        println!("+-----------------------------------------------------+\x1b[0m");

        // -- general info
        println!("\n\x1b[1m System\x1b[0m");
        println!(
            "  OS:       {}",
            System::name().unwrap_or("Unknown".to_string())
        );
        println!(
            "  Kernel:   {}",
            System::kernel_version().unwrap_or("?".to_string())
        );
        println!(
            "  Host:     {}",
            System::host_name().unwrap_or("?".to_string())
        );
        let uptime = System::uptime();
        println!(
            "  Uptime:   {}h {}m {}s",
            uptime / 3600,
            (uptime % 3600) / 60,
            uptime % 60
        );

        // -- cpu
        println!("\n\x1b[1m CPU\x1b[0m");
        let cpus = sys.cpus();

        // get model and core count from the system
        let cpu_brand = cpus
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or("Unknown".to_string());
        let core_count = cpus.len();

        println!("  Model:    {}", cpu_brand);
        println!("  Cores:    {}", core_count);

        for (i, cpu) in cpus.iter().enumerate() {
            let usage = cpu.cpu_usage();
            println!("  Core {:>2}: {} {:>5.1}%", i, bar(usage, 100.0, 25), usage);
        }
        let total_cpu: f32 = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32;
        println!(
            "  \x1b[1mTotal:   {} {:.1}%\x1b[0m",
            bar(total_cpu, 100.0, 25),
            total_cpu
        );

        // -- ram
        println!("\n\x1b[1m Memory\x1b[0m");
        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();
        let free_mem = total_mem - used_mem;
        let mem_pct = used_mem as f32 / total_mem as f32 * 100.0;

        println!("  {} {:.1}%", bar(mem_pct, 100.0, 35), mem_pct);
        println!(
            "  Used: {}  Free: {}  Total: {}",
            format_bytes(used_mem),
            format_bytes(free_mem),
            format_bytes(total_mem)
        );

        // -- swap
        let total_swap = sys.total_swap();
        if total_swap > 0 {
            println!("\n\x1b[1m Swap\x1b[0m");
            let used_swap = sys.used_swap();
            let swap_pct = used_swap as f32 / total_swap as f32 * 100.0;
            println!("  {} {:.1}%", bar(swap_pct, 100.0, 35), swap_pct);
            println!(
                "  Used: {}  Total: {}",
                format_bytes(used_swap),
                format_bytes(total_swap)
            );
        }

        // -- disks
        println!("\n\x1b[1m Disks\x1b[0m");
        for disk in &disks {
            let total = disk.total_space();
            let avail = disk.available_space();
            let used = total - avail;
            let pct = used as f32 / total as f32 * 100.0;
            println!(
                "  {:15} {} {:>5.1}%  ({} / {})",
                disk.mount_point().to_string_lossy(),
                bar(pct, 100.0, 20),
                pct,
                format_bytes(used),
                format_bytes(total)
            );
        }

        // -- network
        println!("\n\x1b[1m Network\x1b[0m");
        for (name, data) in &networks {
            if data.received() > 0 || data.transmitted() > 0 {
                println!(
                    "  {:12} down {:>10}  up {:>10}",
                    name,
                    format_bytes(data.received()),
                    format_bytes(data.transmitted())
                );
            }
        }

        // -- top 5 processes by cpu
        println!("\n\x1b[1m Top 5 Processes (CPU)\x1b[0m");
        let mut processes: Vec<_> = sys.processes().values().collect();
        processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

        println!("  {:>7}  {:>7}  {:>10}  Name", "CPU%", "MEM", "PID");
        println!("  {}", "-".repeat(45));
        for proc in processes.iter().take(5) {
            println!(
                "  {:>6.1}%  {:>7}  {:>10}  {}",
                proc.cpu_usage(),
                format_bytes(proc.memory()),
                proc.pid(),
                proc.name().to_string_lossy()
            );
        }

        println!("\n\x1b[90m  Refreshing every 2 seconds... (Ctrl+C to exit)\x1b[0m");

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(2));
    }
}
