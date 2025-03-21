use cxx::UniquePtr;

use libtorrent_rasterbar_sys::ffi::{ParamPair, Session, TorrentHandle, create_session};

pub use libtorrent_rasterbar_sys::flags::{
    BandwidthStateFlags, ConnectionType, PauseFlags, PeerFlags, PeerSourceFlags, SaveStateFlags, TorrentFlags,
};

mod announce_entry;
mod download_priority;
mod errors;
mod log;
mod peer_info;
mod piece_info;
mod session_stats;
mod torrent_info;
mod torrent_status;

mod tests;

pub use announce_entry::AnnounceEntry;
pub use download_priority::DownloadPriority;
pub use errors::{LTError, LTResult};
pub use log::Log;
pub use peer_info::PeerInfo;
pub use piece_info::PieceInfo;
pub use session_stats::{Metrics, SessionStats};
pub use torrent_info::TorrentInfo;
pub use torrent_status::{State, TorrentStatus};

/// the main libtorrent-rasterbar API.
pub struct LTSession {
    inner: UniquePtr<Session>,
}

impl LTSession {
    /// creates a new session.
    ///
    /// The default values of the session settings are set for a regular
    /// bittorrent client running on a desktop system. There are functions that
    /// can set the session settings to pre set settings for other environments.
    /// These can be used for the basis, and should be tweaked to fit your needs
    /// better.
    ///
    /// ``min_memory_usage`` returns settings that will use the minimal amount of
    /// RAM, at the potential expense of upload and download performance. It
    /// adjusts the socket buffer sizes, disables the disk cache, lowers the send
    /// buffer watermarks so that each connection only has at most one block in
    /// use at any one time. It lowers the outstanding blocks send to the disk
    /// I/O thread so that connections only have one block waiting to be flushed
    /// to disk at any given time. It lowers the max number of peers in the peer
    /// list for torrents. It performs multiple smaller reads when it hashes
    /// pieces, instead of reading it all into memory before hashing.
    ///
    /// This configuration is intended to be the starting point for embedded
    /// devices. It will significantly reduce memory usage.
    ///
    /// ``high_performance_seed`` returns settings optimized for a seed box,
    /// serving many peers and that doesn't do any downloading. It has a 128 MB
    /// disk cache and has a limit of 400 files in its file pool. It support fast
    /// upload rates by allowing large send buffers.
    ///
    /// ``session_param_list`` is a list of key-value pairs that will be used to
    /// override the default values.
    /// The definations and default values of the session settings are in
    /// libtorrent/include/settings_pack.hpp
    /// libtorrent/src/settings_pack.cpp
    pub fn new(
        min_memory_usage: bool,
        high_performance_seed: bool,
        session_param_list: &[(&str, &str)],
        save_state_flags: u32,
        session_state_path: &str,
        resume_dir: &str,
        torrent_dir: &str,
        log_size: u32,
    ) -> LTResult<Self> {
        let params: Vec<_> = session_param_list
            .iter()
            .map(|(k, v)| ParamPair { key: k, value: v })
            .collect();

        let ses = create_session(
            min_memory_usage,
            high_performance_seed,
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

    pub fn get_all_torrent_status(&self) -> Vec<TorrentStatus> {
        self.inner
            .get_all_torrent_status()
            .into_iter()
            .map(TorrentStatus::from)
            .collect()
    }

    pub fn get_logs(&mut self) -> Vec<Log> {
        self.inner.pin_mut().get_logs().into_iter().map(Log::from).collect()
    }
}

unsafe impl Sync for LTSession {}
unsafe impl Send for LTSession {}

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

    /// ``scrape_tracker()`` will send a scrape request to a tracker. By
    /// default (``idx`` = -1) it will scrape the last working tracker. If
    /// ``idx`` is >= 0, the tracker with the specified index will scraped.
    ///
    /// A scrape request queries the tracker for statistics such as total
    /// number of incomplete peers, complete peers, number of downloads etc.
    ///
    /// This request will specifically update the ``num_complete`` and
    /// ``num_incomplete`` fields in the torrent_status struct once it
    /// completes. When it completes, it will generate a scrape_reply_alert.
    /// If it fails, it will generate a scrape_failed_alert.
    pub fn scrape_tracker(&self) {
        self.inner.scrape_tracker();
    }

    /// ``force_recheck`` puts the torrent back in a state where it assumes to
    /// have no resume data. All peers will be disconnected and the torrent
    /// will stop announcing to the tracker. The torrent will be added to the
    /// checking queue, and will be checked (all the files will be read and
    /// compared to the piece hashes). Once the check is complete, the torrent
    /// will start connecting to peers again, as normal.
    /// The torrent will be placed last in queue, i.e. its queue position
    /// will be the highest of all torrents in the session.
    pub fn force_recheck(&self) {
        self.inner.force_recheck();
    }

    /// ``force_reannounce()`` will force this torrent to do another tracker
    /// request, to receive new peers. The ``seconds`` argument specifies how
    /// many seconds from now to issue the tracker announces.
    ///
    /// If the tracker's ``min_interval`` has not passed since the last
    /// announce, the forced announce will be scheduled to happen immediately
    /// as the ``min_interval`` expires. This is to honor trackers minimum
    /// re-announce interval settings.
    ///
    /// The ``tracker_index`` argument specifies which tracker to re-announce.
    /// If set to -1 (which is the default), all trackers are re-announce.
    ///
    /// The ``flags`` argument can be used to affect the re-announce. See
    /// ignore_min_interval.
    ///
    /// ``force_dht_announce`` will announce the torrent to the DHT
    /// immediately.
    ///
    /// ``force_lsd_announce`` will announce the torrent on LSD
    /// immediately.
    pub fn force_reannounce(&self) {
        self.inner.force_reannounce();
    }
    pub fn force_dht_announce(&self) {
        self.inner.force_dht_announce();
    }
    pub fn force_lsd_announce(&self) {
        self.inner.force_lsd_announce();
    }

    pub fn clear_error(&self) {
        self.inner.clear_error();
    }

    /// ``set_upload_limit`` will limit the upload bandwidth used by this
    /// particular torrent to the limit you set. It is given as the number of
    /// bytes per second the torrent is allowed to upload.
    /// ``set_download_limit`` works the same way but for download bandwidth
    /// instead of upload bandwidth. Note that setting a higher limit on a
    /// torrent then the global limit
    /// (``settings_pack::upload_rate_limit``) will not override the global
    /// rate limit. The torrent can never upload more than the global rate
    /// limit.
    ///
    /// ``upload_limit`` and ``download_limit`` will return the current limit
    /// setting, for upload and download, respectively.
    ///
    /// Local peers are not rate limited by default. see peer-classes_.
    pub fn set_upload_limit(&self, limit: i32) {
        self.inner.set_upload_limit(limit);
    }
    pub fn upload_limit(&self) -> i32 {
        self.inner.upload_limit()
    }
    pub fn set_download_limit(&self, limit: i32) {
        self.inner.set_download_limit(limit);
    }
    pub fn download_limit(&self) -> i32 {
        self.inner.download_limit()
    }

    /// This will disconnect all peers and clear the peer list for this
    /// torrent. New peers will have to be acquired before resuming, from
    /// trackers, DHT or local service discovery, for example.
    pub fn clear_peers(&self) {
        self.inner.clear_peers();
    }

    /// ``set_max_uploads()`` sets the maximum number of peers that's unchoked
    /// at the same time on this torrent. If you set this to -1, there will be
    /// no limit. This defaults to infinite. The primary setting controlling
    /// this is the global unchoke slots limit, set by unchoke_slots_limit in
    /// settings_pack.
    ///
    /// ``max_uploads()`` returns the current settings.
    pub fn set_max_uploads(&self, max_uploads: i32) {
        self.inner.set_max_uploads(max_uploads);
    }
    pub fn max_uploads(&self) -> i32 {
        self.inner.max_uploads()
    }

    /// ``set_max_connections()`` sets the maximum number of connection this
    /// torrent will open. If all connections are used up, incoming
    /// connections may be refused or poor connections may be closed. This
    /// must be at least 2. The default is unlimited number of connections. If
    /// -1 is given to the function, it means unlimited. There is also a
    /// global limit of the number of connections, set by
    /// ``connections_limit`` in settings_pack.
    ///
    /// ``max_connections()`` returns the current settings.
    pub fn set_max_connections(&self, max_connections: i32) {
        self.inner.set_max_connections(max_connections);
    }
    pub fn max_connections(&self) -> i32 {
        self.inner.max_connections()
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
    pub fn flags(&self) -> u64 {
        self.inner.flags()
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

    pub fn pause(&self) {
        if !self.is_valid() {
            return;
        }

        self.unset_flags(TorrentFlags::auto_managed.bits());
        self.inner.pause(PauseFlags::graceful_pause.bits());
    }

    pub fn resume(&self) {
        if !self.is_valid() {
            return;
        }

        self.set_flags(TorrentFlags::auto_managed.bits());
    }

    /// ``index`` must be in the range [0, number_of_files).
    ///
    /// ``file_priority()`` queries or sets the priority of file ``index``.
    ///
    /// ``prioritize_files()`` takes a vector that has at as many elements as
    /// there are files in the torrent. Each entry is the priority of that
    /// file. The function sets the priorities of all the pieces in the
    /// torrent based on the vector.
    ///
    /// ``get_file_priorities()`` returns a vector with the priorities of all
    /// files.
    ///
    /// The priority values are the same as for piece_priority(). See
    /// download_priority_t.
    ///
    /// Whenever a file priority is changed, all other piece priorities are
    /// reset to match the file priorities. In order to maintain special
    /// priorities for particular pieces, piece_priority() has to be called
    /// again for those pieces.
    ///
    /// You cannot set the file priorities on a torrent that does not yet have
    /// metadata or a torrent that is a seed. ``file_priority(int, int)`` and
    /// prioritize_files() are both no-ops for such torrents.
    ///
    /// Since changing file priorities may involve disk operations (of moving
    /// files in- and out of the part file), the internal accounting of file
    /// priorities happen asynchronously. i.e. setting file priorities and then
    /// immediately querying them may not yield the same priorities just set.
    /// To synchronize with the priorities taking effect, wait for the
    /// file_prio_alert.
    ///
    /// When combining file- and piece priorities, the resume file will record
    /// both. When loading the resume data, the file priorities will be applied
    /// first, then the piece priorities.
    ///
    /// Moving data from a file into the part file is currently not
    /// supported. If a file has its priority set to 0 *after* it has already
    /// been created, it will not be moved into the partfile.
    pub fn set_file_priority(&self, index: i32, priority: u8) {
        self.inner.set_file_priority(index, priority);
    }
    pub fn get_file_priority(&self, index: i32) -> u8 {
        self.inner.get_file_priority(index)
    }
    pub fn set_prioritize_files(&self, files: &[u8]) {
        self.inner.set_prioritize_files(files);
    }
    pub fn get_file_priorities(&self) -> Vec<u8> {
        self.inner.get_file_priorities()
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

    pub fn make_magnet_uri(&self) -> String {
        self.inner.make_magnet_uri()
    }
}
