use cxx::UniquePtr;
use libtorrent_rasterbar_sys::ffi::{create_session, ParamPair, Session};

pub use libtorrent_rasterbar_sys::flags::SaveStateFlags;

mod errors;
mod session_stats;
mod tests;

use errors::{LTError, LTResult};

/// the main libtorrent-rasterbar API.
pub struct LTSession {
    inner: UniquePtr<Session>,
}

impl LTSession {
    /// creates a new session.
    pub fn new(
        session_params: &[(&str, &str)],
        save_state_flags: u32,
        session_state_path: &str,
        resume_dir: &str,
        torrent_dir: &str,
        log_size: u32,
    ) -> LTResult<Self> {
        let params: Vec<_> = session_params
            .iter()
            .map(|(k, v)| ParamPair { key: k, value: v })
            .collect();

        let ses = create_session(
            &params,
            save_state_flags,
            session_state_path,
            resume_dir,
            torrent_dir,
            log_size,
        )
        .map_err(|e| LTError::FailedToCreateSession(e.to_string()))?;

        Ok(Self { inner: ses })
    }

    /// adds a torrent file.
    pub fn add_torrent(&self, torrent_path: &str, torrent_param_list: &[(&str, &str)]) -> LTResult<()> {
        let params: Vec<_> = torrent_param_list
            .iter()
            .map(|(k, v)| ParamPair { key: k, value: v })
            .collect();

        self.inner
            .add_torrent(torrent_path, &params)
            .map_err(|e| LTError::FailedToAddTorrent(e.to_string()))
    }

    /// adds a magnet link.
    pub fn add_magnet(&self, magnet_uri: &str, torrent_param_list: &[(&str, &str)]) -> LTResult<()> {
        let params: Vec<_> = torrent_param_list
            .iter()
            .map(|(k, v)| ParamPair { key: k, value: v })
            .collect();

        self.inner
            .add_magnet(magnet_uri, &params)
            .map_err(|e| LTError::FailedToAddMagnet(e.to_string()))
    }
}
