# libtorrent-rasterbar-sys

Low-level Rust bindings for [libtorrent-rasterbar](https://github.com/arvidn/libtorrent), a feature-complete BitTorrent implementation.

## Features

- Primary FFI bindings to libtorrent-rasterbar's C++ API
- Safe Rust wrappers around core libtorrent types and functionality

## Dependencies

### Linux

```bash
sudo apt install libboost-tools-dev libboost-dev libboost-system-dev libboost-filesystem-dev
```

### macOS

```bash
brew install boost-build boost openssl@3
```

## Building

This crate requires:

- Rust 1.78+
- C++14 compiler
- Boost libraries

The build process will:

```bash
cargo build
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
libtorrent-rasterbar-sys = "0.1.0"
```

Basic example:

```rust
use libtorrent_rasterbar_sys::ffi::{create_session, ParamPair};
use libtorrent_rasterbar_sys::flags::SaveStateFlags;

fn main() {
    // Create a new session
    let session = create_session(
        false, // min_memory_usage
        false, // high_performance_seed
        &[
            ParamPair {
                key: "user_agent",
                value: "libtorrent-rasterbar-sys/0.1.0",
            },
            ParamPair {
                key: "alert_mask",
                value: "error,peer,port_mapping,storage,tracker,connect,status,ip_block,performance_warning,dht,incoming_request,dht_operation,port_mapping_log,file_progress",
            },
        ], // session params
        SaveStateFlags::save_dht_state.bits(), // save_state_flags
        "/tmp/session.state", // session_state_path
        "/tmp/resume", // resume_dir
        "/tmp/torrents", // torrent_dir
        1000, // log_size
    ).unwrap();

    // Add a torrent
    session.add_torrent("path/to/file.torrent", &[
        ParamPair {
            key: "max_connections",
            value: "100",
        },
        ParamPair {
            key: "max_uploads",
            value: "-1",
        },
        ParamPair {
            key: "save_path",
            value: "/tmp/files",
        },
    ]).unwrap();
}
```

## Architecture

The crate is structured in layers:

1. Raw C++ bindings using cxx (see `ffi` module)
2. Safe Rust wrappers around core types
3. High-level APIs for common operations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

When submitting a PR, please:

1. Add tests for new functionality
2. Update documentation as needed
3. Follow the existing code style
4. Add a description of your changes

## Status

This project is currently in active development. While the core functionality is working, some advanced features may still be missing or incomplete.

Please report any issues you encounter on the GitHub issue tracker.
