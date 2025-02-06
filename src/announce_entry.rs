use libtorrent_rasterbar_sys::ffi;
use serde::Serialize;

/// libtorrent/announce_entry.hpp
///
#[derive(Serialize, Debug)]
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

impl From<ffi::AnnounceInfoHash> for AnnounceInfoHash {
    fn from(aih: ffi::AnnounceInfoHash) -> Self {
        Self {
            message: aih.message,
            last_error: aih.last_error,
            next_announce: aih.next_announce,
            min_announce: aih.min_announce,
            scrape_incomplete: aih.scrape_incomplete,
            scrape_complete: aih.scrape_complete,
            scrape_downloaded: aih.scrape_downloaded,
            fails: aih.fails,
            updating: aih.updating,
            start_sent: aih.start_sent,
            complete_sent: aih.complete_sent,
            triggered_manually: aih.triggered_manually,
        }
    }
}

/// libtorrent/announce_entry.hpp
///
/// announces are sent to each tracker using every listen socket
/// this class holds information about one listen socket for one tracker
#[derive(Serialize, Debug)]
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

impl From<ffi::AnnounceEndpoint> for AnnounceEndpoint {
    fn from(ae: ffi::AnnounceEndpoint) -> Self {
        Self {
            local_endpoint: ae.local_endpoint,
            info_hashes: ae.info_hashes.into_iter().map(AnnounceInfoHash::from).collect(),
            enabled: ae.enabled,
        }
    }
}

/// libtorrent/announce_entry.hpp
///
/// this class holds information about one bittorrent tracker, as it
/// relates to a specific torrent.
#[derive(Serialize, Debug)]
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
    ///   // the tracker was part of the .torrent file
    ///   source_torrent = 1,
    ///   // the tracker was added programmatically via the add_tracker() function
    ///   source_client = 2,
    ///   // the tracker was part of a magnet link
    ///   source_magnet_link = 4,
    ///   // the tracker was received from the swarm via tracker exchange
    ///   source_tex = 8
    /// };
    /// a bitmask specifying which sources we got this tracker from.
    pub source: u8, // default 4

    /// set to true the first time we receive a valid response
    /// from this tracker.
    pub verified: bool, // default 1
}

impl From<ffi::AnnounceEntry> for AnnounceEntry {
    fn from(ae: ffi::AnnounceEntry) -> Self {
        Self {
            url: ae.url,
            trackerid: ae.trackerid,
            endpoints: ae.endpoints.into_iter().map(AnnounceEndpoint::from).collect(),
            tier: ae.tier,
            fail_limit: ae.fail_limit,
            source: ae.source,
            verified: ae.verified,
        }
    }
}
