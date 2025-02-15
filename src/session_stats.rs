#![allow(non_camel_case_types)]

use libtorrent_rasterbar_sys::ffi;
use serde::Serialize;

/// Defination from libtorrent/src/session_stats.cpp
/// Value from libtorrent/src/performance_counters.cpp
#[repr(usize)]
pub enum Metrics {
    /// counters
    // {{{
    /// ``error_peers`` is the total number of peer disconnects
    /// caused by an error (not initiated by this client) and
    /// disconnected initiated by this client (``disconnected_peers``).
    peer_error_peers = 0,
    peer_disconnected_peers = 1,

    /// these counters break down the peer errors into more specific
    /// categories. These errors are what the underlying transport
    /// reported (i.e. TCP or uTP)
    peer_eof_peers = 2,
    peer_connreset_peers = 3,
    peer_connrefused_peers = 4,
    peer_connaborted_peers = 5,
    peer_notconnected_peers = 6,
    peer_perm_peers = 7,
    peer_buffer_peers = 8,
    peer_unreachable_peers = 9,
    peer_broken_pipe_peers = 10,
    peer_addrinuse_peers = 11,
    peer_no_access_peers = 12,
    peer_invalid_arg_peers = 13,
    peer_aborted_peers = 14,

    /// the total number of incoming piece requests we've received followed
    /// by the number of rejected piece requests for various reasons.
    /// max_piece_requests mean we already had too many outstanding requests
    /// from this peer, so we rejected it. cancelled_piece_requests are ones
    /// where the other end explicitly asked for the piece to be rejected.
    peer_piece_requests = 15,
    peer_max_piece_requests = 16,
    peer_invalid_piece_requests = 17,
    peer_choked_piece_requests = 18,
    peer_cancelled_piece_requests = 19,
    peer_piece_rejects = 20,

    /// these counters break down the peer errors into
    /// whether they happen on incoming or outgoing peers.
    peer_error_incoming_peers = 21,
    peer_error_outgoing_peers = 22,

    /// these counters break down the peer errors into
    /// whether they happen on encrypted peers (just
    /// encrypted handshake) and rc4 peers (full stream
    /// encryption). These can indicate whether encrypted
    /// peers are more or less likely to fail
    peer_error_rc4_peers = 23,
    peer_error_encrypted_peers = 24,

    /// these counters break down the peer errors into
    /// whether they happen on uTP peers or TCP peers.
    /// these may indicate whether one protocol is
    /// more error prone
    peer_error_tcp_peers = 25,
    peer_error_utp_peers = 26,

    /// these counters break down the reasons to
    /// disconnect peers.
    peer_connect_timeouts = 43,
    peer_uninteresting_peers = 44,
    peer_timeout_peers = 45,
    peer_no_memory_peers = 46,
    peer_too_many_peers = 47,
    peer_transport_timeout_peers = 48,
    peer_num_banned_peers = 49,
    peer_banned_for_hash_failure = 50,

    peer_connection_attempts = 51,
    peer_connection_attempt_loops = 52,
    peer_boost_connection_attempts = 53,
    peer_missed_connection_attempts = 54,
    peer_no_peer_connection_attempts = 55,
    peer_incoming_connections = 56,

    /// the number of peer connections for each kind of socket.
    /// ``num_peers_half_open`` counts half-open (connecting) peers, no other
    /// count includes those peers.
    /// ``num_peers_up_unchoked_all`` is the total number of unchoked peers,
    /// whereas ``num_peers_up_unchoked`` only are unchoked peers that count
    /// against the limit (i.e. excluding peers that are unchoked because the
    /// limit doesn't apply to them). ``num_peers_up_unchoked_optimistic`` is
    /// the number of optimistically unchoked peers.
    peer_num_tcp_peers = 220,
    peer_num_socks5_peers = 221,
    peer_num_http_proxy_peers = 222,
    peer_num_utp_peers = 223,
    peer_num_i2p_peers = 224,
    peer_num_ssl_peers = 225,
    peer_num_ssl_socks5_peers = 226,
    peer_num_ssl_http_proxy_peers = 227,
    peer_num_ssl_utp_peers = 228,

    peer_num_peers_half_open = 229,
    peer_num_peers_connected = 230,
    peer_num_peers_up_interested = 231,
    peer_num_peers_down_interested = 232,
    peer_num_peers_up_unchoked_all = 233,
    peer_num_peers_up_unchoked_optimistic = 234,
    peer_num_peers_up_unchoked = 235,
    peer_num_peers_down_unchoked = 236,
    peer_num_peers_up_requests = 237,
    peer_num_peers_down_requests = 238,
    peer_num_peers_end_game = 241,
    peer_num_peers_up_disk = 239,
    peer_num_peers_down_disk = 240,

    /// These counters count the number of times the
    /// network thread wakes up for each respective
    /// reason. If these counters are very large, it
    /// may indicate a performance issue, causing the
    /// network thread to wake up too ofte, wasting CPU.
    /// mitigate it by increasing buffers and limits
    /// for the specific trigger that wakes up the
    /// thread.
    net_on_read_counter = 57,
    net_on_write_counter = 58,
    net_on_tick_counter = 59,
    net_on_lsd_counter = 60,
    net_on_lsd_peer_counter = 61,
    net_on_udp_counter = 62,
    net_on_accept_counter = 63,
    net_on_disk_queue_counter = 64,
    net_on_disk_counter = 65,

    /// total number of bytes sent and received by the session
    net_sent_payload_bytes = 127,
    net_sent_bytes = 128,
    net_sent_ip_overhead_bytes = 129,
    net_sent_tracker_bytes = 130,
    net_recv_payload_bytes = 131,
    net_recv_bytes = 132,
    net_recv_ip_overhead_bytes = 133,
    net_recv_tracker_bytes = 134,

    /// the number of sockets currently waiting for upload and download
    /// bandwidth from the rate limiter.
    net_limiter_up_queue = 279,
    net_limiter_down_queue = 280,

    /// the number of upload and download bytes waiting to be handed out from
    /// the rate limiter.
    net_limiter_up_bytes = 281,
    net_limiter_down_bytes = 282,

    /// the number of bytes downloaded that had to be discarded because they
    /// failed the hash check
    net_recv_failed_bytes = 135,

    /// the number of downloaded bytes that were discarded because they
    /// were downloaded multiple times (from different peers)
    net_recv_redundant_bytes = 136,

    /// is false by default and set to true when
    /// the first incoming connection is established
    /// this is used to know if the client is behind
    /// NAT or not.
    net_has_incoming_connections = 278,
    // }}}
    /// gauges
    /// {{{
    /// these gauges count the number of torrents in
    /// different states. Each torrent only belongs to
    /// one of these states. For torrents that could
    /// belong to multiple of these, the most prominent
    /// in picked. For instance, a torrent with an error
    /// counts as an error-torrent, regardless of its other
    /// state.
    ses_num_checking_torrents = 211,
    ses_num_stopped_torrents = 212,
    ses_num_upload_only_torrents = 213,
    ses_num_downloading_torrents = 214,
    ses_num_seeding_torrents = 215,
    ses_num_queued_seeding_torrents = 216,
    ses_num_queued_download_torrents = 217,
    ses_num_error_torrents = 218,

    /// the number of torrents that don't have the
    /// IP filter applied to them.
    ses_non_filter_torrents = 219,

    /// these count the number of times a piece has passed the
    /// hash check, the number of times a piece was successfully
    /// written to disk and the number of total possible pieces
    /// added by adding torrents. e.g. when adding a torrent with
    /// 1000 piece, num_total_pieces_added is incremented by 1000.
    ses_num_piece_passed = 107,
    ses_num_piece_failed = 108,

    ses_num_have_pieces = 109,
    ses_num_total_pieces_added = 110,

    /// the number of allowed unchoked peers
    ses_num_unchoke_slots = 253,

    /// the number of listen sockets that are currently accepting incoming
    /// connections
    ses_num_outstanding_accept = 289,

    /// bittorrent message counters. These counters are incremented
    /// every time a message of the corresponding type is received from
    /// or sent to a bittorrent peer.
    ses_num_incoming_choke = 66,
    ses_num_incoming_unchoke = 67,
    ses_num_incoming_interested = 68,
    ses_num_incoming_not_interested = 69,
    ses_num_incoming_have = 70,
    ses_num_incoming_bitfield = 71,
    ses_num_incoming_request = 72,
    ses_num_incoming_piece = 73,
    ses_num_incoming_cancel = 74,
    ses_num_incoming_dht_port = 75,
    ses_num_incoming_suggest = 76,
    ses_num_incoming_have_all = 77,
    ses_num_incoming_have_none = 78,
    ses_num_incoming_reject = 79,
    ses_num_incoming_allowed_fast = 80,
    ses_num_incoming_ext_handshake = 81,
    ses_num_incoming_pex = 82,
    ses_num_incoming_metadata = 83,
    ses_num_incoming_extended = 84,

    ses_num_outgoing_choke = 85,
    ses_num_outgoing_unchoke = 86,
    ses_num_outgoing_interested = 87,
    ses_num_outgoing_not_interested = 88,
    ses_num_outgoing_have = 89,
    ses_num_outgoing_bitfield = 90,
    ses_num_outgoing_request = 91,
    ses_num_outgoing_piece = 92,
    ses_num_outgoing_cancel = 93,
    ses_num_outgoing_dht_port = 94,
    ses_num_outgoing_suggest = 95,
    ses_num_outgoing_have_all = 96,
    ses_num_outgoing_have_none = 97,
    ses_num_outgoing_reject = 98,
    ses_num_outgoing_allowed_fast = 99,
    ses_num_outgoing_ext_handshake = 100,
    ses_num_outgoing_pex = 101,
    ses_num_outgoing_metadata = 102,
    ses_num_outgoing_extended = 103,
    ses_num_outgoing_hash_request = 104,
    ses_num_outgoing_hashes = 105,
    ses_num_outgoing_hash_reject = 106,

    /// the number of wasted downloaded bytes by reason of the bytes being
    /// wasted.
    ses_waste_piece_timed_out = 121,
    ses_waste_piece_cancelled = 122,
    ses_waste_piece_unknown = 123,
    ses_waste_piece_seed = 124,
    ses_waste_piece_end_game = 125,
    ses_waste_piece_closing = 126,

    /// the number of pieces considered while picking pieces
    picker_piece_picker_partial_loops = 35,
    picker_piece_picker_suggest_loops = 36,
    picker_piece_picker_sequential_loops = 37,
    picker_piece_picker_reverse_rare_loops = 38,
    picker_piece_picker_rare_loops = 39,
    picker_piece_picker_rand_start_loops = 40,
    picker_piece_picker_rand_loops = 41,
    picker_piece_picker_busy_loops = 42,

    /// This breaks down the piece picks into the event that
    /// triggered it
    picker_reject_piece_picks = 27,
    picker_unchoke_piece_picks = 28,
    picker_incoming_redundant_piece_picks = 29,
    picker_incoming_piece_picks = 30,
    picker_end_game_piece_picks = 31,
    picker_snubbed_piece_picks = 32,
    picker_interesting_piece_picks = 33,
    picker_hash_fail_piece_picks = 34,

    /// the number of microseconds it takes from receiving a request from a
    /// peer until we're sending the response back on the socket.
    disk_request_latency = 242,

    disk_disk_blocks_in_use = 243,

    /// ``queued_disk_jobs`` is the number of disk jobs currently queued,
    /// waiting to be executed by a disk thread.
    disk_queued_disk_jobs = 244,
    disk_num_running_disk_jobs = 245,
    disk_num_read_jobs = 246,
    disk_num_write_jobs = 247,
    disk_num_jobs = 248,
    disk_blocked_disk_jobs = 251,

    disk_num_writing_threads = 249,
    disk_num_running_threads = 250,

    /// the number of bytes we have sent to the disk I/O
    /// thread for writing. Every time we hear back from
    /// the disk I/O thread with a completed write job, this
    /// is updated to the number of bytes the disk I/O thread
    /// is actually waiting for to be written (as opposed to
    /// bytes just hanging out in the cache)
    disk_queued_write_bytes = 252,

    /// the number of blocks written and read from disk in total. A block is 16
    /// kiB. ``num_blocks_written`` and ``num_blocks_read``
    disk_num_blocks_written = 111,
    disk_num_blocks_read = 112,

    /// the total number of blocks run through SHA-1 hashing
    disk_num_blocks_hashed = 113,

    /// the number of disk I/O operation for reads and writes. One disk
    /// operation may transfer more then one block.
    disk_num_write_ops = 114,
    disk_num_read_ops = 115,

    /// the number of blocks that had to be read back from disk in order to
    /// hash a piece (when verifying against the piece hash)
    disk_num_read_back = 116,

    /// cumulative time spent in various disk jobs, as well
    /// as total for all disk jobs. Measured in microseconds
    disk_disk_read_time = 117,
    disk_disk_write_time = 118,
    disk_disk_hash_time = 119,
    disk_disk_job_time = 120,

    /// for each kind of disk job, a counter of how many jobs of that kind
    /// are currently blocked by a disk fence
    disk_num_fenced_read = 254,
    disk_num_fenced_write = 255,
    disk_num_fenced_hash = 256,
    disk_num_fenced_move_storage = 257,
    disk_num_fenced_release_files = 258,
    disk_num_fenced_delete_files = 259,
    disk_num_fenced_check_fastresume = 260,
    disk_num_fenced_save_resume_data = 261,
    disk_num_fenced_rename_file = 262,
    disk_num_fenced_stop_torrent = 263,
    disk_num_fenced_flush_piece = 264,
    disk_num_fenced_flush_hashed = 265,
    disk_num_fenced_flush_storage = 266,
    disk_num_fenced_file_priority = 267,
    disk_num_fenced_load_torrent = 268,
    disk_num_fenced_clear_piece = 269,
    disk_num_fenced_tick_storage = 270,

    /// The number of nodes in the DHT routing table
    dht_dht_nodes = 271,

    /// The number of replacement nodes in the DHT routing table
    dht_dht_node_cache = 272,

    /// the number of torrents currently tracked by our DHT node
    dht_dht_torrents = 273,

    /// the number of peers currently tracked by our DHT node
    dht_dht_peers = 274,

    /// the number of immutable data items tracked by our DHT node
    dht_dht_immutable_data = 275,

    /// the number of mutable data items tracked by our DHT node
    dht_dht_mutable_data = 276,

    /// the number of RPC observers currently allocated
    dht_dht_allocated_observers = 277,

    /// the total number of DHT messages sent and received
    dht_dht_messages_in = 137,
    dht_dht_messages_out = 139,

    /// the number of incoming DHT requests that were dropped. There are a few
    /// different reasons why incoming DHT packets may be dropped:
    ///
    /// 1. there wasn't enough send quota to respond to them.
    /// 2. the Denial of service logic kicked in, blocking the peer
    /// 3. ignore_dark_internet is enabled, and the packet came from a
    ///    non-public IP address
    /// 4. the bencoding of the message was invalid
    dht_dht_messages_in_dropped = 138,

    /// the number of outgoing messages that failed to be
    /// sent
    dht_dht_messages_out_dropped = 140,

    /// the total number of bytes sent and received by the DHT
    dht_dht_bytes_in = 141,
    dht_dht_bytes_out = 142,

    /// the number of DHT messages we've sent and received
    /// by kind.
    dht_dht_ping_in = 143,
    dht_dht_ping_out = 144,
    dht_dht_find_node_in = 145,
    dht_dht_find_node_out = 146,
    dht_dht_get_peers_in = 147,
    dht_dht_get_peers_out = 148,
    dht_dht_announce_peer_in = 149,
    dht_dht_announce_peer_out = 150,
    dht_dht_get_in = 151,
    dht_dht_get_out = 152,
    dht_dht_put_in = 153,
    dht_dht_put_out = 154,
    dht_dht_sample_infohashes_in = 155,
    dht_dht_sample_infohashes_out = 156,

    /// the number of failed incoming DHT requests by kind of request
    dht_dht_invalid_announce = 157,
    dht_dht_invalid_get_peers = 158,
    dht_dht_invalid_find_node = 159,
    dht_dht_invalid_put = 160,
    dht_dht_invalid_get = 161,
    dht_dht_invalid_sample_infohashes = 162,

    /// The number of times a lost packet has been interpreted as congestion,
    /// cutting the congestion window in half. Some lost packets are not
    /// interpreted as congestion, notably MTU-probes
    utp_utp_packet_loss = 163,

    /// The number of timeouts experienced. This is when a connection doesn't
    /// hear back from the other end within a sliding average RTT + 2 average
    /// deviations from the mean (approximately). The actual time out is
    /// configurable and also depends on the state of the socket.
    utp_utp_timeout = 164,

    /// The total number of packets sent and received
    utp_utp_packets_in = 165,
    utp_utp_packets_out = 166,

    /// The number of packets lost but re-sent by the fast-retransmit logic.
    /// This logic is triggered after 3 duplicate ACKs.
    utp_utp_fast_retransmit = 167,

    /// The number of packets that were re-sent, for whatever reason
    utp_utp_packet_resend = 168,

    /// The number of incoming packets where the delay samples were above
    /// and below the delay target, respectively. The delay target is
    /// configurable and is a parameter to the LEDBAT congestion control.
    utp_utp_samples_above_target = 169,
    utp_utp_samples_below_target = 170,

    /// The total number of packets carrying payload received and sent,
    /// respectively.
    utp_utp_payload_pkts_in = 171,
    utp_utp_payload_pkts_out = 172,

    /// The number of packets received that are not valid uTP packets (but
    /// were sufficiently similar to not be treated as DHT or UDP tracker
    /// packets).
    utp_utp_invalid_pkts_in = 173,

    /// The number of duplicate payload packets received. This may happen if
    /// the outgoing ACK is lost.
    utp_utp_redundant_pkts_in = 174,

    /// the number of uTP sockets in each respective state
    utp_num_utp_idle = 283,
    utp_num_utp_syn_sent = 284,
    utp_num_utp_connected = 285,
    utp_num_utp_fin_sent = 286,
    utp_num_utp_close_wait = 287,
    utp_num_utp_deleted = 288,

    /// the buffer sizes accepted by
    /// socket send and receive calls respectively.
    /// The larger the buffers are, the more efficient,
    /// because it require fewer system calls per byte.
    /// The size is 1 << n, where n is the number
    /// at the end of the counter name. i.e.
    /// 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
    /// 16384, 32768, 65536, 131072, 262144, 524288, 1048576
    /// bytes
    sock_bufs_socket_send_size3 = 175,
    sock_bufs_socket_send_size4 = 176,
    sock_bufs_socket_send_size5 = 177,
    sock_bufs_socket_send_size6 = 178,
    sock_bufs_socket_send_size7 = 179,
    sock_bufs_socket_send_size8 = 180,
    sock_bufs_socket_send_size9 = 181,
    sock_bufs_socket_send_size10 = 182,
    sock_bufs_socket_send_size11 = 183,
    sock_bufs_socket_send_size12 = 184,
    sock_bufs_socket_send_size13 = 185,
    sock_bufs_socket_send_size14 = 186,
    sock_bufs_socket_send_size15 = 187,
    sock_bufs_socket_send_size16 = 188,
    sock_bufs_socket_send_size17 = 189,
    sock_bufs_socket_send_size18 = 190,
    sock_bufs_socket_send_size19 = 191,
    sock_bufs_socket_send_size20 = 192,
    sock_bufs_socket_recv_size3 = 193,
    sock_bufs_socket_recv_size4 = 194,
    sock_bufs_socket_recv_size5 = 195,
    sock_bufs_socket_recv_size6 = 196,
    sock_bufs_socket_recv_size7 = 197,
    sock_bufs_socket_recv_size8 = 198,
    sock_bufs_socket_recv_size9 = 199,
    sock_bufs_socket_recv_size10 = 200,
    sock_bufs_socket_recv_size11 = 201,
    sock_bufs_socket_recv_size12 = 202,
    sock_bufs_socket_recv_size13 = 203,
    sock_bufs_socket_recv_size14 = 204,
    sock_bufs_socket_recv_size15 = 205,
    sock_bufs_socket_recv_size16 = 206,
    sock_bufs_socket_recv_size17 = 207,
    sock_bufs_socket_recv_size18 = 208,
    sock_bufs_socket_recv_size19 = 209,
    sock_bufs_socket_recv_size20 = 210,

    /// if the outstanding tracker announce limit is reached, tracker
    /// announces are queued, to be issued when an announce slot opens up.
    /// this measure the number of tracker announces currently in the
    /// queue
    tracker_num_queued_tracker_announces = 290,
    // }}}
}

impl Metrics {
    pub fn index(self) -> usize {
        self as usize
    }
}

#[derive(Serialize, Debug, Default)]
pub struct TwoSessionStats {
    pub stats: Vec<i64>,
    pub timestamp: i64,
    pub prev_stats: Vec<i64>,
    pub prev_timestamp: i64,
}

impl From<ffi::TwoSessionStats> for TwoSessionStats {
    fn from(s: ffi::TwoSessionStats) -> Self {
        Self {
            stats: s.stats,
            timestamp: s.timestamp,
            prev_stats: s.prev_stats,
            prev_timestamp: s.prev_timestamp,
        }
    }
}

#[derive(Serialize, Debug, Default)]
pub struct SessionStats {
    pub two_session_stats: TwoSessionStats,
}
