use libtorrent_rasterbar_sys::ffi;
use serde::Serialize;

/// libtorrent/peer_info.hpp
///
/// holds information and statistics about one peer
/// that libtorrent is connected to
#[derive(Serialize, Debug)]
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
    /// TODO: use bitvec
    pub pieces: Vec<bool>,

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

impl From<ffi::PeerInfo> for PeerInfo {
    fn from(pi: ffi::PeerInfo) -> Self {
        Self {
            client: pi.client,
            pieces: pi.pieces,
            total_download: pi.total_download,
            total_upload: pi.total_upload,
            last_request: pi.last_request,
            last_active: pi.last_active,
            download_queue_time: pi.download_queue_time,
            flags: pi.flags,
            source: pi.source,
            up_speed: pi.up_speed,
            down_speed: pi.down_speed,
            payload_up_speed: pi.payload_up_speed,
            payload_down_speed: pi.payload_down_speed,
            pid: pi.pid,
            queue_bytes: pi.queue_bytes,
            request_timeout: pi.request_timeout,
            send_buffer_size: pi.send_buffer_size,
            used_send_buffer: pi.used_send_buffer,
            receive_buffer_size: pi.receive_buffer_size,
            used_receive_buffer: pi.used_receive_buffer,
            receive_buffer_watermark: pi.receive_buffer_watermark,
            num_hashfails: pi.num_hashfails,
            download_queue_length: pi.download_queue_length,
            timed_out_requests: pi.timed_out_requests,
            busy_requests: pi.busy_requests,
            requests_in_buffer: pi.requests_in_buffer,
            target_dl_queue_length: pi.target_dl_queue_length,
            upload_queue_length: pi.upload_queue_length,
            failcount: pi.failcount,
            downloading_piece_index: pi.downloading_piece_index,
            downloading_block_index: pi.downloading_block_index,
            downloading_progress: pi.downloading_progress,
            downloading_total: pi.downloading_total,
            connection_type: pi.connection_type,
            pending_disk_bytes: pi.pending_disk_bytes,
            pending_disk_read_bytes: pi.pending_disk_read_bytes,
            send_quota: pi.send_quota,
            receive_quota: pi.receive_quota,
            rtt: pi.rtt,
            num_pieces: pi.num_pieces,
            download_rate_peak: pi.download_rate_peak,
            upload_rate_peak: pi.upload_rate_peak,
            progress: pi.progress,
            progress_ppm: pi.progress_ppm,
            ip: pi.ip,
            local_endpoint: pi.local_endpoint,
            read_state: pi.read_state,
            write_state: pi.write_state,
            i2p_destination: pi.i2p_destination,
        }
    }
}
