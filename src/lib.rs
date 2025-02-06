use announce_entry::AnnounceEntry;
use cxx::UniquePtr;
use libtorrent_rasterbar_sys::ffi::{create_session, ParamPair, Session, TorrentHandle};

pub use libtorrent_rasterbar_sys::flags::{SaveStateFlags, TorrentFlags};

mod announce_entry;
mod errors;
mod log;
mod peer_info;
mod piece_info;
mod session_stats;
mod torrent_info;
mod torrent_status;

mod tests;

pub use errors::{LTError, LTResult};
pub use log::Log;
pub use peer_info::PeerInfo;
pub use piece_info::PieceInfo;
pub use session_stats::{Metrics, SessionStats};
pub use torrent_info::TorrentInfo;
pub use torrent_status::TorrentStatus;

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

    /// removes a torrent
    pub fn remove_torrent(&self, info_hash_str: &str, delete_files: bool) {
        self.inner.remove_torrent(info_hash_str, delete_files)
    }

    /// get the session stats
    pub fn get_stats(&self) -> SessionStats {
        SessionStats {
            two_session_stats: self.inner.get_stats().into(),
        }
    }

    /// get the torrent handle by info hash
    pub fn get_torrent_handle(&self, info_hash_str: &str) -> LTTorrentHandle {
        LTTorrentHandle::new(self.inner.get_torrent_handle(info_hash_str))
    }

    pub fn pause(&self) {
        self.inner.pause();
    }

    pub fn resume(&self) {
        self.inner.resume();
    }

    pub fn is_paused(&self) -> bool {
        self.inner.is_paused()
    }

    /// Get the list of torrents in the session
    pub fn get_torrents(&self) -> Vec<TorrentInfo> {
        self.inner.get_torrents().into_iter().map(TorrentInfo::from).collect()
    }

    pub fn get_logs(&mut self) -> Vec<Log> {
        self.inner.pin_mut().get_logs().into_iter().map(Log::from).collect()
    }
}

pub struct LTTorrentHandle {
    inner: UniquePtr<TorrentHandle>,
}

impl LTTorrentHandle {
    fn new(handle: UniquePtr<TorrentHandle>) -> LTTorrentHandle {
        LTTorrentHandle { inner: handle }
    }

    pub fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    pub fn add_tracker(&self, tracker_url: &str, tier: u8) {
        self.inner.add_tracker(tracker_url, tier);
    }

    pub fn scrape_tracker(&self) {
        self.inner.scrape_tracker();
    }

    pub fn force_recheck(&self) {
        self.inner.force_recheck();
    }
    pub fn force_reannounce(&self) {
        self.inner.force_reannounce();
    }

    pub fn clear_error(&self) {
        self.inner.clear_error();
    }

    /// sets and gets the torrent state flags. See torrent_flags_t.
    /// The ``set_flags`` overload that take a mask will affect all
    /// flags part of the mask, and set their values to what the
    /// ``flags`` argument is set to. This allows clearing and
    /// setting flags in a single function call.
    /// The ``set_flags`` overload that just takes flags, sets all
    /// the specified flags and leave any other flags unchanged.
    /// ``unset_flags`` clears the specified flags, while leaving
    /// any other flags unchanged.
    ///
    /// The `seed_mode` flag is special, it can only be cleared once the
    /// torrent has been added, and it can only be set as part of the
    /// add_torrent_params flags, when adding the torrent.
    ///
    /// flags: TorrentFlags
    pub fn set_flags(&self, flags: u64) {
        self.inner.set_flags(flags);
    }

    /// sets and gets the torrent state flags. See torrent_flags_t.
    /// The ``set_flags`` overload that take a mask will affect all
    /// flags part of the mask, and set their values to what the
    /// ``flags`` argument is set to. This allows clearing and
    /// setting flags in a single function call.
    /// The ``set_flags`` overload that just takes flags, sets all
    /// the specified flags and leave any other flags unchanged.
    /// ``unset_flags`` clears the specified flags, while leaving
    /// any other flags unchanged.
    ///
    /// The `seed_mode` flag is special, it can only be cleared once the
    /// torrent has been added, and it can only be set as part of the
    /// add_torrent_params flags, when adding the torrent.
    ///
    /// flags: TorrentFlags
    pub fn set_flags_with_mask(&self, flags: u64, mask: u64) {
        self.inner.set_flags_with_mask(flags, mask);
    }

    /// sets and gets the torrent state flags. See torrent_flags_t.
    /// The ``set_flags`` overload that take a mask will affect all
    /// flags part of the mask, and set their values to what the
    /// ``flags`` argument is set to. This allows clearing and
    /// setting flags in a single function call.
    /// The ``set_flags`` overload that just takes flags, sets all
    /// the specified flags and leave any other flags unchanged.
    /// ``unset_flags`` clears the specified flags, while leaving
    /// any other flags unchanged.
    ///
    /// The `seed_mode` flag is special, it can only be cleared once the
    /// torrent has been added, and it can only be set as part of the
    /// add_torrent_params flags, when adding the torrent.
    ///
    /// flags: TorrentFlags
    pub fn unset_flags(&self, flags: u64) {
        self.inner.unset_flags(flags);
    }

    pub fn get_torrent_info(&self) -> TorrentInfo {
        self.inner.get_torrent_info().into()
    }

    pub fn get_peers(&self) -> Vec<PeerInfo> {
        self.inner.get_peers().into_iter().map(PeerInfo::from).collect()
    }

    pub fn get_file_progress(&self, piece_granularity: bool) -> Vec<i64> {
        self.inner.get_file_progress(piece_granularity)
    }

    pub fn get_piece_info(&self) -> PieceInfo {
        self.inner.get_piece_info().into()
    }

    pub fn get_piece_availability(&self) -> Vec<i32> {
        self.inner.get_piece_availability()
    }

    pub fn get_trackers(&self) -> Vec<AnnounceEntry> {
        self.inner.get_trackers().into_iter().map(AnnounceEntry::from).collect()
    }

    pub fn get_torrent_status(&self) -> TorrentStatus {
        self.inner.get_torrent_status().into()
    }
}
