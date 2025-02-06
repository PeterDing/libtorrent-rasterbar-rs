use libtorrent_rasterbar_sys::ffi;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Log {
    pub message: String,
    pub timestamp: i64,
}

impl From<ffi::Log> for Log {
    fn from(log: ffi::Log) -> Log {
        Log {
            message: log.message,
            timestamp: log.timestamp,
        }
    }
}
