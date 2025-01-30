mod session_handler;

mod test_session;

#[cxx::bridge(namespace = "libtorrent_wrapper")]
pub mod ffi {
    pub struct ParamPair<'a> {
        pub key: &'a str,
        pub value: &'a str,
    }

    #[derive(Debug)]
    pub struct FileEntry {
        pub file_path: String,
        pub file_name: String,
        pub file_size: u64,
    }

    /// libtorrent/torrent_info.hpp
    ///
    /// the torrent_info class holds the information found in a .torrent file.
    #[derive(Debug)]
    pub struct TorrentInfo {
        /// The information about files in the torrent, including paths, sizes, and
        /// piece mapping
        pub files: Vec<FileEntry>,

        /// List of tracker URLs and their tier priority
        pub trackers: Vec<String>,

        /// List of similar torrents by their info-hash (BEP 38)
        pub similar_torrents: Vec<String>,

        /// List of collection names this torrent belongs to (BEP 38)
        pub collections: Vec<String>,

        /// List of web seed entries (HTTP/URL seeds)
        pub web_seeds: Vec<String>,

        /// If this torrent contains any DHT nodes, they are put in this vector in
        /// their original form (host name and port number).
        pub nodes: Vec<String>,

        /// the total number of bytes the torrent-file
        /// represents. Note that this is the number of pieces times the piece
        /// size (modulo the last piece possibly being smaller). With pad files,
        /// the total size will be larger than the sum of all (regular) file
        /// sizes.
        pub total_size: u64,

        /// ``piece_length`` and ``num_pieces`` are the number of byte
        /// for each piece and the total number of pieces, respectively. The
        /// difference between ``piece_size`` and ``piece_length`` is that
        /// ``piece_size`` takes the piece index as argument and gives you the
        /// exact size of that piece. It will always be the same as
        /// ``piece_length`` except in the case of the last piece, which may be
        /// smaller.
        pub piece_length: u32,
        pub num_pieces: u32,

        /// the number of blocks there are in the typical piece. There
        /// may be fewer in the last piece)
        pub blocks_per_piece: u32,

        /// the info-hash of the torrent. For BitTorrent v2 support, use
        /// ``info_hashes()`` to get an object that may hold both a v1 and v2
        /// info-hash
        pub info_hash: String,

        /// the number of files in the torrent
        pub num_files: u32,

        /// the name of the torrent.
        /// name contains UTF-8 encoded string.
        pub name: String,

        /// ``creation_date`` returns the creation date of the torrent as time_t
        /// (`posix time`_). If there's no time stamp in the torrent file, 0 is
        /// returned.
        /// .. _`posix time`: http://www.opengroup.org/onlinepubs/009695399/functions/time.html
        pub creation_date: i64,

        /// the creator string in the torrent. If there is
        /// no creator string it will return an empty string.
        pub creator: String,

        /// the comment associated with the torrent. If
        /// there's no comment, it will return an empty string.
        /// comment contains UTF-8 encoded string.
        pub comment: String,

        /// SSL certificate in x509 format (empty if not SSL torrent)
        pub ssl_cert: String,

        /// Flags indicating torrent properties
        pub is_private: bool, // True if this is a private torrent
        pub is_i2p: bool, // True if this is an i2p torrent
    }

    /// libtorrent/peer_info.hpp
    ///
    /// holds information and statistics about one peer
    /// that libtorrent is connected to
    #[derive(Debug)]
    pub struct PeerInfo {
        /// A human readable string describing the software at the other end of
        /// the connection. In some cases this information is not available, then
        /// it will contain a string that may give away something about which
        /// software is running in the other end. In the case of a web seed, the
        /// server type and version will be a part of this string. This is UTF-8
        /// encoded.
        pub client: String,

        /// a bitfield, with one bit per piece in the torrent. Each bit tells you
        /// if the peer has that piece (if it's set to 1) or if the peer miss that
        /// piece (set to 0).
        pub pieces: Vec<bool>,
        /// TODO: use bitvec

        /// the total number of bytes downloaded from and uploaded to this peer.
        /// These numbers do not include the protocol chatter, but only the
        /// payload data.
        pub total_download: i64,
        pub total_upload: i64,

        /// the time since we last sent a request to this peer and since any
        /// transfer occurred with this peer
        /// nanoseconds
        pub last_request: i64,
        pub last_active: i64,

        /// the time until all blocks in the request queue will be downloaded
        /// nanoseconds
        pub download_queue_time: i64,

        /// tells you in which state the peer is in. It is set to
        /// any combination of the peer_flags_t flags (u32) above.
        pub flags: u32,

        /// a combination of flags describing from which sources this peer
        /// was received. A combination of the peer_source_flags_t (u8) above.
        pub source: u8,

        /// the current upload and download speed we have to and from this peer
        /// (including any protocol messages). updated about once per second
        pub up_speed: i32,
        pub down_speed: i32,

        /// The transfer rates of payload data only updated about once per second
        pub payload_up_speed: i32,
        pub payload_down_speed: i32,

        /// the peer's id as used in the bittorrent protocol. This id can be used
        /// to extract 'fingerprints' from the peer. Sometimes it can tell you
        /// which client the peer is using. See identify_client()_
        pub pid: String,

        /// the number of bytes we have requested from this peer, but not yet
        /// received.
        pub queue_bytes: i32,

        /// the number of seconds until the current front piece request will time
        /// out. This timeout can be adjusted through
        /// ``settings_pack::request_timeout``.
        /// -1 means that there is not outstanding request.
        pub request_timeout: i32,

        /// the number of bytes allocated
        /// and used for the peer's send buffer, respectively.
        pub send_buffer_size: i32,
        pub used_send_buffer: i32,

        /// the number of bytes
        /// allocated and used as receive buffer, respectively.
        pub receive_buffer_size: i32,
        pub used_receive_buffer: i32,
        pub receive_buffer_watermark: i32,

        /// the number of pieces this peer has participated in sending us that
        /// turned out to fail the hash check.
        pub num_hashfails: i32,

        /// this is the number of requests we have sent to this peer that we
        /// haven't got a response for yet
        pub download_queue_length: i32,

        /// the number of block requests that have timed out, and are still in the
        /// download queue
        pub timed_out_requests: i32,

        /// the number of busy requests in the download queue. A busy request is a
        /// request for a block we've also requested from a different peer
        pub busy_requests: i32,

        /// the number of requests messages that are currently in the send buffer
        /// waiting to be sent.
        pub requests_in_buffer: i32,

        /// the number of requests that is tried to be maintained (this is
        /// typically a function of download speed)
        pub target_dl_queue_length: i32,

        /// the number of piece-requests we have received from this peer
        /// that we haven't answered with a piece yet.
        pub upload_queue_length: i32,

        /// the number of times this peer has "failed". i.e. failed to connect or
        /// disconnected us. The failcount is decremented when we see this peer in
        /// a tracker response or peer exchange message.
        pub failcount: i32,

        /// You can know which piece, and which part of that piece, that is
        /// currently being downloaded from a specific peer by looking at these
        /// four members. ``downloading_piece_index`` is the index of the piece
        /// that is currently being downloaded. This may be set to -1 if there's
        /// currently no piece downloading from this peer. If it is >= 0, the
        /// other three members are valid. ``downloading_block_index`` is the
        /// index of the block (or sub-piece) that is being downloaded.
        /// ``downloading_progress`` is the number of bytes of this block we have
        /// received from the peer, and ``downloading_total`` is the total number
        /// of bytes in this block.
        pub downloading_piece_index: i32,
        pub downloading_block_index: i32,
        pub downloading_progress: i32,
        pub downloading_total: i32,

        /// the kind of connection this peer uses. See ConnectionType flags.
        pub connection_type: u8,

        /// the number of bytes this peer has pending in the disk-io thread.
        /// Downloaded and waiting to be written to disk. This is what is capped
        /// by ``settings_pack::max_queued_disk_bytes``.
        pub pending_disk_bytes: i32,

        /// number of outstanding bytes to read
        /// from disk
        pub pending_disk_read_bytes: i32,

        /// the number of bytes this peer has been assigned to be allowed to send
        /// and receive until it has to request more quota from the bandwidth
        /// manager.
        pub send_quota: i32,
        pub receive_quota: i32,

        /// an estimated round trip time to this peer, in milliseconds. It is
        /// estimated by timing the TCP ``connect()``. It may be 0 for
        /// incoming connections.
        pub rtt: i32,

        /// the number of pieces this peer has.
        pub num_pieces: i32,

        /// the highest download and upload rates seen on this connection. They
        /// are given in bytes per second. This number is reset to 0 on reconnect.
        pub download_rate_peak: i32,
        pub upload_rate_peak: i32,

        /// the progress of the peer in the range [0, 1]. This is always 0 when
        /// floating point operations are disabled, instead use ``progress_ppm``.
        pub progress: f32, // [0, 1]

        /// indicates the download progress of the peer in the range [0, 1000000]
        /// (parts per million).
        pub progress_ppm: i32,

        /// the IP-address to this peer. The type is an asio endpoint. For
        /// more info, see the asio_ documentation. This field is not valid for
        /// i2p peers. Instead use the i2p_destination() function.
        //
        /// .. _asio: http://asio.sourceforge.net/asio-0.3.8/doc/asio/reference.html
        pub ip: String, // ip:port

        /// the IP and port pair the socket is bound to locally. i.e. the IP
        /// address of the interface it's going out over. This may be useful for
        /// multi-homed clients with multiple interfaces to the internet.
        /// This field is not valid for i2p peers.
        pub local_endpoint: String, // ip:port

        /// bitmasks indicating what state this peer
        /// is in with regards to sending and receiving data. The states are
        /// defined as independent flags of type BandwidthStateFlags, in this
        /// class.
        pub read_state: u8,
        pub write_state: u8,

        /// If this peer is an i2p peer, this function returns the destination
        /// address of the peer: sha256_hash
        pub i2p_destination: String,
    }

    /// libtorrent/announce_entry.hpp
    ///
    #[derive(Debug)]
    pub struct AnnounceInfoHash {
        /// if this tracker has returned an error or warning message
        /// that message is stored here
        message: String,

        /// if this tracker failed the last time it was contacted
        /// this error code specifies what error occurred
        last_error: String, // error massage

        /// the time of next tracker announce
        next_announce: i64, // seconds

        /// no announces before this time
        min_announce: i64,

        /// these are either -1 or the scrape information this tracker last
        /// responded with. *incomplete* is the current number of downloaders in
        /// the swarm, *complete* is the current number of seeds in the swarm and
        /// *downloaded* is the cumulative number of completed downloads of this
        /// torrent, since the beginning of time (from this tracker's point of
        /// view).
        ///
        /// if this tracker has returned scrape data, these fields are filled in
        /// with valid numbers. Otherwise they are set to -1. ``incomplete`` counts
        /// the number of current downloaders. ``complete`` counts the number of
        /// current peers completed the download, or "seeds". ``downloaded`` is the
        /// cumulative number of completed downloads.
        scrape_incomplete: i32, // default -1
        scrape_complete: i32,   // default -1
        scrape_downloaded: i32, // default -1

        /// the number of times in a row we have failed to announce to this
        /// tracker.
        fails: u8, // default 7

        /// true while we're waiting for a response from the tracker.
        updating: bool, // default true

        /// set to true when we get a valid response from an announce
        /// with event=started. If it is set, we won't send start in the subsequent
        /// announces.
        start_sent: bool, // default true

        /// set to true when we send a event=completed.
        complete_sent: bool, // default true

        /// internal
        triggered_manually: bool, // default true
    }

    /// libtorrent/announce_entry.hpp
    ///
    /// announces are sent to each tracker using every listen socket
    /// this class holds information about one listen socket for one tracker
    #[derive(Debug)]
    pub struct AnnounceEndpoint {
        /// the local endpoint of the listen interface associated with this endpoint
        pub local_endpoint: String, // ip:port

        /// torrents can be announced using multiple info hashes
        /// for different protocol versions
        ///
        /// info_hashes[0] is the v1 info hash (SHA1)
        /// info_hashes[1] is the v2 info hash (truncated SHA-256)
        pub info_hashes: Vec<AnnounceInfoHash>,

        /// set to false to not announce from this endpoint
        pub enabled: bool, // default true
    }

    /// libtorrent/announce_entry.hpp
    ///
    /// this class holds information about one bittorrent tracker, as it
    /// relates to a specific torrent.
    #[derive(Debug)]
    pub struct AnnounceEntry {
        /// tracker URL as it appeared in the torrent file
        pub url: String,

        /// the current ``&trackerid=`` argument passed to the tracker.
        /// this is optional and is normally empty (in which case no
        /// trackerid is sent).
        pub trackerid: String,

        /// each local listen socket (endpoint) will announce to the tracker. This
        /// list contains state per endpoint.
        pub endpoints: Vec<AnnounceEndpoint>,

        /// the tier this tracker belongs to
        pub tier: u8,

        /// the max number of failures to announce to this tracker in
        /// a row, before this tracker is not used anymore. 0 means unlimited
        pub fail_limit: u8,

        /// flags for the source bitmask, each indicating where
        /// we heard about this tracker
        /// enum tracker_source
        /// {
        /// 	// the tracker was part of the .torrent file
        /// 	source_torrent = 1,
        /// 	// the tracker was added programmatically via the add_tracker() function
        /// 	source_client = 2,
        /// 	// the tracker was part of a magnet link
        /// 	source_magnet_link = 4,
        /// 	// the tracker was received from the swarm via tracker exchange
        /// 	source_tex = 8
        /// };

        /// a bitmask specifying which sources we got this tracker from.
        pub source: u8, // default 4

        /// set to true the first time we receive a valid response
        /// from this tracker.
        pub verified: bool, // default 1
    }

    unsafe extern "C++" {
        include!("libtorrent-rasterbar-sys/wrap/wrapper.hpp");

        type Session;

        /// Create a new session
        fn create_session(
            session_params: &[ParamPair],
            save_state_flags: u32,
            session_state_path: &str,
            resume_dir: &str,
            torrent_dir: &str,
        ) -> Result<UniquePtr<Session>>;

        fn add_torrent(&self, torrent_path: &str, torrent_param_list: &[ParamPair]) -> Result<()>;

        fn add_magnet(&self, magnet_uri: &str, torrent_param_list: &[ParamPair]) -> Result<()>;

        /// Get the list of torrents in the session
        fn get_torrents(&self) -> Vec<TorrentInfo>;

        fn get_torrent_info(&self, info_hash_str: &str) -> TorrentInfo;

        fn get_peers(self: Pin<&mut Self>, info_hash_str: &str) -> Vec<PeerInfo>;

        fn get_file_progress(self: Pin<&mut Self>, info_hash_str: &str, piece_granularity: bool) -> Vec<i64>;

        fn get_piece_availability(self: Pin<&mut Self>, info_hash_str: &str) -> Vec<i32>;

        fn get_trackers(self: Pin<&mut Self>, info_hash_str: &str) -> Vec<AnnounceEntry>;
    }
}
