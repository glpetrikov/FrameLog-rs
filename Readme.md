# FrameLog-rs

**Version 0.2.2** - Early development, API may change

FrameLog is a lightweight library for outputting logs to the console and file, 
version on Rust

## Version 0.2.2 alpha


![Language](https://img.shields.io/badge/language-rust-blue.svg)
![License](https://img.shields.io/badge/license-MIT-greeen.svg)

![Status](https://img.shields.io/badge/status-Alpha-lightgrey.svg)

[![Crates.io](https://img.shields.io/crates/v/framelog-rs.svg)](https://crates.io/crates/framelog-rs)

\
![GitHub Repo stars](https://img.shields.io/github/stars/glpetrikov/FrameLog-rs?style=social)
![GitHub forks](https://img.shields.io/github/forks/glpetrikov/FrameLog-rs?style=social)
![GitHub issues](https://img.shields.io/github/issues/glpetrikov/FrameLog-rs)

## Features
- **Colorful console output** - ANSI colors for different log levels
- **File logging** - Write logs to files with append mode
- **Lightweight** - Only ~4-7 KiB compiled size
- **Thread-safe** - `Send` + `Sync` implementations
- **Flexible file operations** - Read, write, delete log files

## Dependencies
- Standard Rust library

## License
FrameLog-rs is distributed under the **MIT License**.  
See [LICENSE](LICENSE) for details.

## Supported Platforms

Any platform with the **Rust**

## Performance & Size

FrameLog is **lightweight**:

**Why so small?**
- Minimal dependencies

**Perfect for:**
- Docker containers (minimal images)
- Fast compilation times
- Quick program startup
- for projects where you just need a logger

## Status
**FrameLog** is currently in **Alpha** stage

## Quick Start
```bash
git clone https://github.com/glpetrikov/FrameLog-rs
cd FrameLog-rs
cargo test 
```

## Installations
```toml
[dependencies]
framelog-rs = "0.2.2"
```

## Building
install rustc
```bash
cargo build
```

## Example
```rust
use framelog_rs::*;

fn main() -> std::io::Result<()> {
    // Console logging
    info!("Application started");
    warn!("Memory usage: {}%", 85);
    error!("Failed to connect");
    
    // File logging
    let mut fh = FileHandler::new("app.log")?;
    fh.writeln("Server running on port 8080")?;
    
    Ok(())
}
```

## Authors
**Gleb Petrikov**

## Roadmap
Reach C++ FrameLog level
