# Libtorrent Rasterbar Rust Bindings

Rust bindings for the [libtorrent-rasterbar](https://www.libtorrent.org/) C++ library, providing a native Rust interface for working with the BitTorrent protocol.

## Features

- Primary-featured BitTorrent client implementation
- Ergonomic Rust API for libtorrent-rasterbar
- Cross-platform (Linux, macOS)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
libtorrent-rasterbar = "0.1"
```

## Dependencies

### Linux

```bash
sudo apt install libboost-tools-dev libboost-dev libboost-system-dev libboost-filesystem-dev
```

### macOS

```bash
brew install boost-build boost openssl@3
```

## Usage

Basic example of creating a session and adding a magnet:

```rust
use libtorrent_rasterbar::{LTSession, SaveStateFlags};

fn main() -> anyhow::Result<()> {
    let ses = LTSession::new(
        false, // min_memory_usage
        false, // high_performance_seed
        &[
            ("user_agent", "libtorrent-rasterbar-rs/2.0.11"),
            ("alert_mask", "error,peer,port_mapping,storage,tracker,connect,status,ip_block,performance_warning,dht,incoming_request,dht_operation,port_mapping_log,file_progress"),
        ],
        SaveStateFlags::save_dht_state.bits(),
        "/tmp/session_state",
        "/tmp/resume_data",
        "/tmp/torrents",
        100, // log_size
    )?;

    ses.add_torrent(
        "magnet:...",
        &[
            ("max_connections", "100"),
            ("max_uploads", "-1"),
            ("save_path", "/downloads"),
        ],
    )?;

    Ok(())
}
```

## API Overview

### Core Types

- `LTSession`: Main session manager
- `SessionStats`: Session statistics and metrics
- `TorrentHandle`: Handle to an individual torrent
- `TorrentStatus`: Current status of a torrent
- `PeerInfo`: Information about connected peers
- `PieceInfo`: Piece download information

### Features

- Add torrents from files or magnet links
- Configure session settings
- Manage torrent priorities
- Query detailed torrent and peer information
- Handle tracker announces and DHT operations
- Monitor download progress and statistics

## Acknowledgements

This project builds upon the excellent [libtorrent-rasterbar](https://www.libtorrent.org/) C++ library by Arvid Norberg and the libtorrent contributors.
