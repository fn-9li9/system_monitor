# system_monitor

A real-time system monitor written in Rust. Displays CPU usage per core, memory,
swap, disk usage, network I/O, and top processes — all refreshed in the terminal
every 2 seconds.

---

## features

- cpu usage per core with color-coded progress bars
- total ram and swap usage
- disk space per mount point
- network traffic (received / transmitted) per interface
- top 5 processes sorted by cpu usage
- ansi color output: green / yellow / red based on load thresholds
- lightweight — single binary, no runtime dependencies

---

## requirements

- rust 1.70 or newer
- cargo

install rust via [rustup](https://rustup.rs):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## installation

clone the repository and build:
```bash
git clone https://github.com/youruser/system_monitor.git
cd system_monitor
make build
```

---

## usage

run in debug mode:
```bash
make run
# or
cargo run
```

run optimized release build:
```bash
make release
make run-release
```

press `ctrl+c` to exit.

---

## output example
```
+-----------------------------------------------------+
|           System Monitor - Rust                     |
+-----------------------------------------------------+

 System
  OS:       macOS
  Kernel:   23.6.0
  Host:     my-machine
  Uptime:   3h 22m 10s

 CPU
  Core  0: [######################...] 89.2%
  Core  1: [############.............] 48.1%
  Core  2: [######...............  ] 24.3%
  Total:   [################.........] 53.9%

 Memory
  [###########################........]  74.2%
  Used: 23.76 GB  Free: 8.24 GB  Total: 32.00 GB

 Disks
  /               [################....] 81.0%  (405.00 GB / 500.00 GB)
  /home           [################################....] 89.2%  (892.00 GB / 1000.00 GB)

 Network
  en0          down     12.40 MB  up      3.21 MB

 Top 5 Processes (CPU)
    CPU%      MEM         PID  Name
  ---------------------------------------------
    42.3%   1.20 GB        1234  firefox
    18.1%  512.00 MB       2341  code
     9.4%  256.00 MB       3412  cargo
     4.2%   64.00 MB       4123  rustc
     1.1%   32.00 MB       5231  node
```

---

## project structure
```
system_monitor/
├── Cargo.toml        # dependencies and project metadata
├── Cargo.lock        # locked dependency versions
├── Makefile          # build and dev commands
├── README.md         # this file
└── src/
    └── main.rs       # entry point and all monitor logic
```

---

## dependencies

| crate     | version | purpose                              |
|-----------|---------|--------------------------------------|
| sysinfo   | 0.38    | cross-platform system info retrieval |

---

## how it works

1. `System::new_all()` initializes and loads all system data on startup
2. `Disks::new_with_refreshed_list()` and `Networks::new_with_refreshed_list()` initialize I/O structs
3. every 2 seconds the main loop calls `refresh_all()`, `disks.refresh()`, and `networks.refresh()`
4. data is read and rendered to stdout using ansi escape codes for color
5. `bar()` renders a proportional ascii progress bar colored by load threshold:
   - green  → below 50%
   - yellow → 50–80%
   - red    → above 80%

---

## optional tooling

install `cargo-watch` to auto-rerun on file changes during development:
```bash
cargo install cargo-watch
make watch
```
