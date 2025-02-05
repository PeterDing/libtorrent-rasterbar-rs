mod flags;

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

    /// libtorrent/torrent_status.hpp
    ///
    /// holds a snapshot of the status of a torrent, as queried by
    /// torrent_handle::status().
    #[derive(Debug)]
    pub struct TorrentStatus {
        // may be set to an error code describing why the torrent was paused, in
        // case it was paused by an error. If the torrent is not paused or if it's
        // paused but not because of an error, this error_code is not set.
        // if the error is attributed specifically to a file, error_file is set to
        // the index of that file in the .torrent file.
        pub errc: String, // error message

        // if the torrent is stopped because of an disk I/O error, this field
        // contains the index of the file in the torrent that encountered the
        // error. If the error did not originate in a file in the torrent, there
        // are a few special values this can be set to: error_file_none,
        // error_file_ssl_ctx, error_file_exception, error_file_partfile or
        // error_file_metadata;
        pub error_file: i8, // default: torrent_status::error_file_none;

        // the path to the directory where this torrent's files are stored.
        // It's typically the path as was given to async_add_torrent() or
        // add_torrent() when this torrent was started. This field is only
        // included if the torrent status is queried with
        // ``torrent_handle::query_save_path``.
        pub save_path: String,

        // the name of the torrent. Typically this is derived from the
        // .torrent file. In case the torrent was started without metadata,
        // and hasn't completely received it yet, it returns the name given
        // to it when added to the session. See ``session::add_torrent``.
        // This field is only included if the torrent status is queried
        // with ``torrent_handle::query_name``.
        pub name: String,

        // the time until the torrent will announce itself to the tracker.
        pub next_announce: i64, // timestamp

        // the URL of the last working tracker. If no tracker request has
        // been successful yet, it's set to an empty string.
        pub current_tracker: String,

        // the number of bytes downloaded and uploaded to all peers, accumulated,
        // *this session* only. The session is considered to restart when a
        // torrent is paused and restarted again. When a torrent is paused, these
        // counters are reset to 0. If you want complete, persistent, stats, see
        // ``all_time_upload`` and ``all_time_download``.
        pub total_download: i64,
        pub total_upload: i64,

        // counts the amount of bytes send and received this session, but only
        // the actual payload data (i.e the interesting data), these counters
        // ignore any protocol overhead. The session is considered to restart
        // when a torrent is paused and restarted again. When a torrent is
        // paused, these counters are reset to 0.
        pub total_payload_download: i64,
        pub total_payload_upload: i64,

        // the number of bytes that has been downloaded and that has failed the
        // piece hash test. In other words, this is just how much crap that has
        // been downloaded since the torrent was last started. If a torrent is
        // paused and then restarted again, this counter will be reset.
        pub total_failed_bytes: i64,

        // the number of bytes that has been downloaded even though that data
        // already was downloaded. The reason for this is that in some situations
        // the same data can be downloaded by mistake. When libtorrent sends
        // requests to a peer, and the peer doesn't send a response within a
        // certain timeout, libtorrent will re-request that block. Another
        // situation when libtorrent may re-request blocks is when the requests
        // it sends out are not replied in FIFO-order (it will re-request blocks
        // that are skipped by an out of order block). This is supposed to be as
        // low as possible. This only counts bytes since the torrent was last
        // started. If a torrent is paused and then restarted again, this counter
        // will be reset.
        pub total_redundant_bytes: i64,

        // a bitmask that represents which pieces we have (set to true) and the
        // pieces we don't have. It's a pointer and may be set to 0 if the
        // torrent isn't downloading or seeding.
        pub pieces: Vec<bool>,

        // a bitmask representing which pieces has had their hash checked. This
        // only applies to torrents in *seed mode*. If the torrent is not in seed
        // mode, this bitmask may be empty.
        pub verified_pieces: Vec<bool>,

        // the total number of bytes of the file(s) that we have. All this does
        // not necessarily has to be downloaded during this session (that's
        // ``total_payload_download``).
        pub total_done: i64,

        // the total number of bytes to download for this torrent. This
        // may be less than the size of the torrent in case there are
        // pad files. This number only counts bytes that will actually
        // be requested from peers.
        pub total: i64,

        // the number of bytes we have downloaded, only counting the pieces that
        // we actually want to download. i.e. excluding any pieces that we have
        // but have priority 0 (i.e. not wanted).
        // Once a torrent becomes seed, any piece- and file priorities are
        // forgotten and all bytes are considered "wanted".
        pub total_wanted_done: i64,

        // The total number of bytes we want to download. This may be smaller
        // than the total torrent size in case any pieces are prioritized to 0,
        // i.e.  not wanted.
        // Once a torrent becomes seed, any piece- and file priorities are
        // forgotten and all bytes are considered "wanted".
        pub total_wanted: i64,

        // are accumulated upload and download payload byte counters. They are
        // saved in and restored from resume data to keep totals across sessions.
        pub all_time_upload: i64,
        pub all_time_download: i64,

        // the posix-time when this torrent was added. i.e. what ``time(nullptr)``
        // returned at the time.
        pub added_time: i64,

        // the posix-time when this torrent was finished. If the torrent is not
        // yet finished, this is 0.
        pub completed_time: i64,

        // the time when we, or one of our peers, last saw a complete copy of
        // this torrent.
        pub last_seen_complete: i64,

        // The allocation mode for the torrent. See storage_mode_t for the
        // options. For more information, see storage-allocation_.
        pub storage_mode: u8, // default: storage_mode_sparse;

        // a value in the range [0, 1], that represents the progress of the
        // torrent's current task. It may be checking files or downloading.
        pub progress: f32,

        // progress parts per million (progress * 1000000) when disabling
        // floating point operations, this is the only option to query progress
        //
        // reflects the same value as ``progress``, but instead in a range [0,
        // 1000000] (ppm = parts per million). When floating point operations are
        // disabled, this is the only alternative to the floating point value in
        // progress.
        pub progress_ppm: i32,

        // the position this torrent has in the download
        // queue. If the torrent is a seed or finished, this is -1.
        pub queue_position: i32,

        // the total rates for all peers for this torrent. These will usually
        // have better precision than summing the rates from all peers. The rates
        // are given as the number of bytes per second.
        pub download_rate: i32,
        pub upload_rate: i32,

        // the total transfer rate of payload only, not counting protocol
        // chatter. This might be slightly smaller than the other rates, but if
        // projected over a long time (e.g. when calculating ETA:s) the
        // difference may be noticeable.
        pub download_payload_rate: i32,
        pub upload_payload_rate: i32,

        // the number of peers that are seeding that this client is
        // currently connected to.
        pub num_seeds: i32,

        // the number of peers this torrent currently is connected to. Peer
        // connections that are in the half-open state (is attempting to connect)
        // or are queued for later connection attempt do not count. Although they
        // are visible in the peer list when you call get_peer_info().
        pub num_peers: i32,

        // if the tracker sends scrape info in its announce reply, these fields
        // will be set to the total number of peers that have the whole file and
        // the total number of peers that are still downloading. set to -1 if the
        // tracker did not send any scrape data in its announce reply.
        pub num_complete: i32,   // default: -1
        pub num_incomplete: i32, // default: -1

        // the number of seeds in our peer list and the total number of peers
        // (including seeds). We are not necessarily connected to all the peers
        // in our peer list. This is the number of peers we know of in total,
        // including banned peers and peers that we have failed to connect to.
        pub list_seeds: i32,
        pub list_peers: i32,

        // the number of peers in this torrent's peer list that is a candidate to
        // be connected to. i.e. It has fewer connect attempts than the max fail
        // count, it is not a seed if we are a seed, it is not banned etc. If
        // this is 0, it means we don't know of any more peers that we can try.
        pub connect_candidates: i32,

        // the number of pieces that has been downloaded. It is equivalent to:
        // ``std::accumulate(pieces->begin(), pieces->end())``. So you don't have
        // to count yourself. This can be used to see if anything has updated
        // since last time if you want to keep a graph of the pieces up to date.
        // Note that these pieces have not necessarily been written to disk yet,
        // and there is a risk the write to disk will fail.
        pub num_pieces: i32,

        // the number of distributed copies of the torrent. Note that one copy
        // may be spread out among many peers. It tells how many copies there are
        // currently of the rarest piece(s) among the peers this client is
        // connected to.
        pub distributed_full_copies: i32,

        // tells the share of pieces that have more copies than the rarest
        // piece(s). Divide this number by 1000 to get the fraction.
        //
        // For example, if ``distributed_full_copies`` is 2 and
        // ``distributed_fraction`` is 500, it means that the rarest pieces have
        // only 2 copies among the peers this torrent is connected to, and that
        // 50% of all the pieces have more than two copies.
        //
        // If we are a seed, the piece picker is deallocated as an optimization,
        // and piece availability is no longer tracked. In this case the
        // distributed copies members are set to -1.
        pub distributed_fraction: i32,

        // the number of distributed copies of the file. note that one copy may
        // be spread out among many peers. This is a floating point
        // representation of the distributed copies.
        //
        // the integer part tells how many copies
        //   there are of the rarest piece(s)
        //
        // the fractional part tells the fraction of pieces that
        //   have more copies than the rarest piece(s).
        pub distributed_copies: f32,

        // the size of a block, in bytes. A block is a sub piece, it is the
        // number of bytes that each piece request asks for and the number of
        // bytes that each bit in the ``partial_piece_info``'s bitset represents,
        // see get_download_queue(). This is typically 16 kB, but it may be
        // smaller, if the pieces are smaller.
        pub block_size: i32,

        // the number of unchoked peers in this torrent.
        pub num_uploads: i32,

        // the number of peer connections this torrent has, including half-open
        // connections that hasn't completed the bittorrent handshake yet. This
        // is always >= ``num_peers``.
        pub num_connections: i32,

        // the set limit of upload slots (unchoked peers) for this torrent.
        pub uploads_limit: i32,

        // the set limit of number of connections for this torrent.
        pub connections_limit: i32,

        // the number of peers in this torrent that are waiting for more
        // bandwidth quota from the torrent rate limiter. This can determine if
        // the rate you get from this torrent is bound by the torrents limit or
        // not. If there is no limit set on this torrent, the peers might still
        // be waiting for bandwidth quota from the global limiter, but then they
        // are counted in the ``session_status`` object.
        pub up_bandwidth_queue: i32,
        pub down_bandwidth_queue: i32,

        // A rank of how important it is to seed the torrent, it is used to
        // determine which torrents to seed and which to queue. It is based on
        // the peer to seed ratio from the tracker scrape. For more information,
        // see queuing_. Higher value means more important to seed
        pub seed_rank: i32,

        // the main state the torrent is in. See torrent_status::state_t.
        pub state: u8, // default: checking_resume_data

        // true if this torrent has unsaved changes
        // to its download state and statistics since the last resume data
        // was saved.
        pub need_save_resume: bool,

        // true if all pieces have been downloaded.
        pub is_seeding: bool,

        // true if all pieces that have a priority > 0 are downloaded. There is
        // only a distinction between finished and seeding if some pieces or
        // files have been set to priority 0, i.e. are not downloaded.
        pub is_finished: bool,

        // true if this torrent has metadata (either it was started from a
        // .torrent file or the metadata has been downloaded). The only scenario
        // where this can be false is when the torrent was started torrent-less
        // (i.e. with just an info-hash and tracker ip, a magnet link for
        // instance).
        pub has_metadata: bool,

        // true if there has ever been an incoming connection attempt to this
        // torrent.
        pub has_incoming: bool,

        // this is true if this torrent's storage is currently being moved from
        // one location to another. This may potentially be a long operation
        // if a large file ends up being copied from one drive to another.
        pub moving_storage: bool,

        // these are set to true if this torrent is allowed to announce to the
        // respective peer source. Whether they are true or false is determined by
        // the queue logic/auto manager. Torrents that are not auto managed will
        // always be allowed to announce to all peer sources.
        pub announcing_to_trackers: bool,
        pub announcing_to_lsd: bool,
        pub announcing_to_dht: bool,

        // the timestamps of the last time this torrent uploaded or downloaded
        // payload to any peer.
        pub last_upload: i64,
        pub last_download: i64,

        // these are cumulative counters of for how long the torrent has been in
        // different states. active means not paused and added to session. Whether
        // it has found any peers or not is not relevant.
        // finished means all selected files/pieces were downloaded and available
        // to other peers (this is always a subset of active time).
        // seeding means all files/pieces were downloaded and available to
        // peers. Being available to peers does not imply there are other peers
        // asking for the payload.
        pub active_duration: i64,
        pub finished_duration: i64,
        pub seeding_duration: i64,

        // reflects several of the torrent's flags. For more
        // information, see ``torrent_handle::flags()``.
        pub flags: u64,
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

    /// libtorrent/torrent_handle.hpp
    ///
    // holds the state of a block in a piece. Who we requested
    // it from and how far along we are at downloading it.
    #[derive(Debug)]
    pub struct BlockInfo {
        // the number of bytes that have been received for this block
        bytes_progress: u32, // default 15

        // the total number of bytes in this block.
        block_size: u32, // default 15

        // the state this block is in (see block_state_t)
        // this is the enum used for the block_info::state field.
        //
        // enum block_state_t
        // {
        // 	// This block has not been downloaded or requested form any peer.
        // 	none,
        // 	// The block has been requested, but not completely downloaded yet.
        // 	requested,
        // 	// The block has been downloaded and is currently queued for being
        // 	// written to disk.
        // 	writing,
        // 	// The block has been written to disk.
        // 	finished
        // };
        state: u8, // default 2

        // the number of peers that is currently requesting this block. Typically
        // this is 0 or 1, but at the end of the torrent blocks may be requested
        // by more peers in parallel to speed things up.
        num_peers: u32, // default 14
    }

    /// libtorrent/torrent_handle.hpp
    ///
    /// This class holds information about pieces that have outstanding requests
    /// or outstanding writes
    #[derive(Debug)]
    pub struct PartialPieceInfo {
        // the index of the piece in question. ``blocks_in_piece`` is the number
        // of blocks in this particular piece. This number will be the same for
        // most pieces, but
        // the last piece may have fewer blocks than the standard pieces.
        pub piece_index: i32,

        // the number of blocks in this piece
        pub blocks_in_piece: i32,

        // the number of blocks that are in the finished state
        pub finished: i32,

        // the number of blocks that are in the writing state
        pub writing: i32,

        // the number of blocks that are in the requested state
        pub requested: i32,

        // this is an array of ``blocks_in_piece`` number of
        // items. One for each block in the piece.
        //
        // .. warning:: This is a pointer that points to an array
        //	that's owned by the session object. The next time
        //	get_download_queue() is called, it will be invalidated.
        //	In the case of piece_info_alert, these pointers point into the alert
        //	object itself, and will be invalidated when the alert destruct.
        pub blocks: Vec<BlockInfo>,
    }

    #[derive(Debug)]
    pub struct PieceInfo {
        partial_pieces: Vec<PartialPieceInfo>,
        blocks: Vec<BlockInfo>,
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

    #[derive(Debug)]
    pub struct Log {
        pub message: String,
        pub timestamp: i64,
    }

    #[derive(Debug)]
    pub struct TwoSessionStats {
        pub stats: Vec<i64>,
        pub timestamp: i64,
        pub prev_stats: Vec<i64>,
        pub prev_timestamp: i64,
    }

    unsafe extern "C++" {
        include!("libtorrent-rasterbar-sys/wrap/wrapper.hpp");

        type Session;
        type TorrentHandle;

        /// Create a new session
        fn create_session(
            session_params: &[ParamPair],
            save_state_flags: u32,
            session_state_path: &str,
            resume_dir: &str,
            torrent_dir: &str,
            log_size: u32,
        ) -> Result<UniquePtr<Session>>;

        // Session impl
        // {{{
        fn add_torrent(self: &Session, torrent_path: &str, torrent_param_list: &[ParamPair]) -> Result<()>;

        fn add_magnet(self: &Session, magnet_uri: &str, torrent_param_list: &[ParamPair]) -> Result<()>;

        fn remove_torrent(self: &Session, info_hash_str: &str, delete_files: bool);

        fn get_stats(self: &Session) -> TwoSessionStats;

        fn get_torrent_handle(self: &Session, info_hash_str: &str) -> UniquePtr<TorrentHandle>;

        fn pause(self: &Session);
        fn resume(self: &Session);
        fn is_paused(self: &Session) -> bool;

        /// Get the list of torrents in the session
        fn get_torrents(self: &Session) -> Vec<TorrentInfo>;

        fn get_logs(self: Pin<&mut Session>) -> Vec<Log>;
        // }}}

        // TorrentHandle impl
        // {{{
        fn is_valid(self: &TorrentHandle) -> bool;

        fn add_tracker(self: &TorrentHandle, tracker_url: &str, tier: u8);

        fn scrape_tracker(self: &TorrentHandle);
        fn force_recheck(self: &TorrentHandle);
        fn force_reannounce(self: &TorrentHandle);
        fn clear_error(self: &TorrentHandle);

        // sets and gets the torrent state flags. See torrent_flags_t.
        // The ``set_flags`` overload that take a mask will affect all
        // flags part of the mask, and set their values to what the
        // ``flags`` argument is set to. This allows clearing and
        // setting flags in a single function call.
        // The ``set_flags`` overload that just takes flags, sets all
        // the specified flags and leave any other flags unchanged.
        // ``unset_flags`` clears the specified flags, while leaving
        // any other flags unchanged.
        //
        // The `seed_mode` flag is special, it can only be cleared once the
        // torrent has been added, and it can only be set as part of the
        // add_torrent_params flags, when adding the torrent.
        fn set_flags(self: &TorrentHandle, flags: u64);
        fn set_flags_with_mask(self: &TorrentHandle, flags: u64, mask: u64);
        fn unset_flags(self: &TorrentHandle, flags: u64);

        fn get_torrent_info(self: &TorrentHandle) -> TorrentInfo;

        fn get_peers(self: &TorrentHandle) -> Vec<PeerInfo>;

        fn get_file_progress(self: &TorrentHandle, piece_granularity: bool) -> Vec<i64>;

        fn get_piece_info(self: &TorrentHandle) -> PieceInfo;

        fn get_piece_availability(self: &TorrentHandle) -> Vec<i32>;

        fn get_trackers(self: &TorrentHandle) -> Vec<AnnounceEntry>;

        fn get_torrent_status(self: &TorrentHandle) -> TorrentStatus;
        // }}}
    }
}
