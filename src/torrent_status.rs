#![allow(non_camel_case_types)]

use libtorrent_rasterbar_sys::ffi;
use serde::Serialize;

// the different overall states a torrent can be in
#[repr(u8)]
pub enum State {
    // internal
    unused_enum_for_backwards_compatibility,

    // The torrent has not started its download yet, and is
    // currently checking existing files.
    checking_files,

    // The torrent is trying to download metadata from peers.
    // This implies the ut_metadata extension is in use.
    downloading_metadata,

    // The torrent is being downloaded. This is the state
    // most torrents will be in most of the time. The progress
    // meter will tell how much of the files that has been
    // downloaded.
    downloading,

    // In this state the torrent has finished downloading but
    // still doesn't have the entire torrent. i.e. some pieces
    // are filtered and won't get downloaded.
    finished,

    // In this state the torrent has finished downloading and
    // is a pure seeder.
    seeding,

    unused_enum_for_backwards_compatibility_allocating,

    // The torrent is currently checking the fast resume data and
    // comparing it to the files on disk. This is typically
    // completed in a fraction of a second, but if you add a
    // large number of torrents at once, they will queue up.
    checking_resume_data,
}

/// libtorrent/torrent_status.hpp
///
/// holds a snapshot of the status of a torrent, as queried by
/// torrent_handle::status().
#[derive(Serialize, Debug)]
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
    //
    // see State
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

    // the info-hash for this torrent
    pub info_hash: String,

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
    //
    // see libtorrent_rasterbar_sys::flags::TorrentFlags
    pub flags: u64,
}

impl From<ffi::TorrentStatus> for TorrentStatus {
    fn from(ts: ffi::TorrentStatus) -> Self {
        Self {
            errc: ts.errc,
            error_file: ts.error_file,
            save_path: ts.save_path,
            name: ts.name,
            next_announce: ts.next_announce,
            current_tracker: ts.current_tracker,
            total_download: ts.total_download,
            total_upload: ts.total_upload,
            total_payload_download: ts.total_payload_download,
            total_payload_upload: ts.total_payload_upload,
            total_failed_bytes: ts.total_failed_bytes,
            total_redundant_bytes: ts.total_redundant_bytes,
            pieces: ts.pieces,
            verified_pieces: ts.verified_pieces,
            total_done: ts.total_done,
            total: ts.total,
            total_wanted_done: ts.total_wanted_done,
            total_wanted: ts.total_wanted,
            all_time_upload: ts.all_time_upload,
            all_time_download: ts.all_time_download,
            added_time: ts.added_time,
            completed_time: ts.completed_time,
            last_seen_complete: ts.last_seen_complete,
            storage_mode: ts.storage_mode,
            progress: ts.progress,
            progress_ppm: ts.progress_ppm,
            queue_position: ts.queue_position,
            download_rate: ts.download_rate,
            upload_rate: ts.upload_rate,
            download_payload_rate: ts.download_payload_rate,
            upload_payload_rate: ts.upload_payload_rate,
            num_seeds: ts.num_seeds,
            num_peers: ts.num_peers,
            num_complete: ts.num_complete,
            num_incomplete: ts.num_incomplete,
            list_seeds: ts.list_seeds,
            list_peers: ts.list_peers,
            connect_candidates: ts.connect_candidates,
            num_pieces: ts.num_pieces,
            distributed_full_copies: ts.distributed_full_copies,
            distributed_fraction: ts.distributed_fraction,
            distributed_copies: ts.distributed_copies,
            block_size: ts.block_size,
            num_uploads: ts.num_uploads,
            num_connections: ts.num_connections,
            uploads_limit: ts.uploads_limit,
            connections_limit: ts.connections_limit,
            up_bandwidth_queue: ts.up_bandwidth_queue,
            down_bandwidth_queue: ts.down_bandwidth_queue,
            seed_rank: ts.seed_rank,
            state: ts.state,
            need_save_resume: ts.need_save_resume,
            is_seeding: ts.is_seeding,
            is_finished: ts.is_finished,
            has_metadata: ts.has_metadata,
            has_incoming: ts.has_incoming,
            moving_storage: ts.moving_storage,
            announcing_to_trackers: ts.announcing_to_trackers,
            announcing_to_lsd: ts.announcing_to_lsd,
            announcing_to_dht: ts.announcing_to_dht,
            info_hash: ts.info_hash,
            last_upload: ts.last_upload,
            last_download: ts.last_download,
            active_duration: ts.active_duration,
            finished_duration: ts.finished_duration,
            seeding_duration: ts.seeding_duration,
            flags: ts.flags,
        }
    }
}
