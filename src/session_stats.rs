#![allow(non_camel_case_types)]

use libtorrent_rasterbar_sys::ffi;
use serde::Serialize;

/// libtorrent/src/session_stats.cpp
#[repr(usize)]
pub enum Metrics {
    // counters
    // {{{
    // ``error_peers`` is the total number of peer disconnects
    // caused by an error (not initiated by this client) and
    // disconnected initiated by this client (``disconnected_peers``).
    peer_error_peers,
    peer_disconnected_peers,

    // these counters break down the peer errors into more specific
    // categories. These errors are what the underlying transport
    // reported (i.e. TCP or uTP)
    peer_eof_peers,
    peer_connreset_peers,
    peer_connrefused_peers,
    peer_connaborted_peers,
    peer_notconnected_peers,
    peer_perm_peers,
    peer_buffer_peers,
    peer_unreachable_peers,
    peer_broken_pipe_peers,
    peer_addrinuse_peers,
    peer_no_access_peers,
    peer_invalid_arg_peers,
    peer_aborted_peers,

    // the total number of incoming piece requests we've received followed
    // by the number of rejected piece requests for various reasons.
    // max_piece_requests mean we already had too many outstanding requests
    // from this peer, so we rejected it. cancelled_piece_requests are ones
    // where the other end explicitly asked for the piece to be rejected.
    peer_piece_requests,
    peer_max_piece_requests,
    peer_invalid_piece_requests,
    peer_choked_piece_requests,
    peer_cancelled_piece_requests,
    peer_piece_rejects,

    // these counters break down the peer errors into
    // whether they happen on incoming or outgoing peers.
    peer_error_incoming_peers,
    peer_error_outgoing_peers,

    // these counters break down the peer errors into
    // whether they happen on encrypted peers (just
    // encrypted handshake) and rc4 peers (full stream
    // encryption). These can indicate whether encrypted
    // peers are more or less likely to fail
    peer_error_rc4_peers,
    peer_error_encrypted_peers,

    // these counters break down the peer errors into
    // whether they happen on uTP peers or TCP peers.
    // these may indicate whether one protocol is
    // more error prone
    peer_error_tcp_peers,
    peer_error_utp_peers,

    // these counters break down the reasons to
    // disconnect peers.
    peer_connect_timeouts,
    peer_uninteresting_peers,
    peer_timeout_peers,
    peer_no_memory_peers,
    peer_too_many_peers,
    peer_transport_timeout_peers,
    peer_num_banned_peers,
    peer_banned_for_hash_failure,

    peer_connection_attempts,
    peer_connection_attempt_loops,
    peer_boost_connection_attempts,
    peer_missed_connection_attempts,
    peer_no_peer_connection_attempts,
    peer_incoming_connections,

    // the number of peer connections for each kind of socket.
    // ``num_peers_half_open`` counts half-open (connecting) peers, no other
    // count includes those peers.
    // ``num_peers_up_unchoked_all`` is the total number of unchoked peers,
    // whereas ``num_peers_up_unchoked`` only are unchoked peers that count
    // against the limit (i.e. excluding peers that are unchoked because the
    // limit doesn't apply to them). ``num_peers_up_unchoked_optimistic`` is
    // the number of optimistically unchoked peers.
    peer_num_tcp_peers,
    peer_num_socks5_peers,
    peer_num_http_proxy_peers,
    peer_num_utp_peers,
    peer_num_i2p_peers,
    peer_num_ssl_peers,
    peer_num_ssl_socks5_peers,
    peer_num_ssl_http_proxy_peers,
    peer_num_ssl_utp_peers,

    peer_num_peers_half_open,
    peer_num_peers_connected,
    peer_num_peers_up_interested,
    peer_num_peers_down_interested,
    peer_num_peers_up_unchoked_all,
    peer_num_peers_up_unchoked_optimistic,
    peer_num_peers_up_unchoked,
    peer_num_peers_down_unchoked,
    peer_num_peers_up_requests,
    peer_num_peers_down_requests,
    peer_num_peers_end_game,
    peer_num_peers_up_disk,
    peer_num_peers_down_disk,

    // These counters count the number of times the
    // network thread wakes up for each respective
    // reason. If these counters are very large, it
    // may indicate a performance issue, causing the
    // network thread to wake up too ofte, wasting CPU.
    // mitigate it by increasing buffers and limits
    // for the specific trigger that wakes up the
    // thread.
    net_on_read_counter,
    net_on_write_counter,
    net_on_tick_counter,
    net_on_lsd_counter,
    net_on_lsd_peer_counter,
    net_on_udp_counter,
    net_on_accept_counter,
    net_on_disk_queue_counter,
    net_on_disk_counter,

    // total number of bytes sent and received by the session
    net_sent_payload_bytes,
    net_sent_bytes,
    net_sent_ip_overhead_bytes,
    net_sent_tracker_bytes,
    net_recv_payload_bytes,
    net_recv_bytes,
    net_recv_ip_overhead_bytes,
    net_recv_tracker_bytes,

    // the number of sockets currently waiting for upload and download
    // bandwidth from the rate limiter.
    net_limiter_up_queue,
    net_limiter_down_queue,

    // the number of upload and download bytes waiting to be handed out from
    // the rate limiter.
    net_limiter_up_bytes,
    net_limiter_down_bytes,

    // the number of bytes downloaded that had to be discarded because they
    // failed the hash check
    net_recv_failed_bytes,

    // the number of downloaded bytes that were discarded because they
    // were downloaded multiple times (from different peers)
    net_recv_redundant_bytes,

    // is false by default and set to true when
    // the first incoming connection is established
    // this is used to know if the client is behind
    // NAT or not.
    net_has_incoming_connections,
    // }}}

    // gauges
    // {{{
    // these gauges count the number of torrents in
    // different states. Each torrent only belongs to
    // one of these states. For torrents that could
    // belong to multiple of these, the most prominent
    // in picked. For instance, a torrent with an error
    // counts as an error-torrent, regardless of its other
    // state.
    ses_num_checking_torrents,
    ses_num_stopped_torrents,
    ses_num_upload_only_torrents,
    ses_num_downloading_torrents,
    ses_num_seeding_torrents,
    ses_num_queued_seeding_torrents,
    ses_num_queued_download_torrents,
    ses_num_error_torrents,

    // the number of torrents that don't have the
    // IP filter applied to them.
    ses_non_filter_torrents,

    // these count the number of times a piece has passed the
    // hash check, the number of times a piece was successfully
    // written to disk and the number of total possible pieces
    // added by adding torrents. e.g. when adding a torrent with
    // 1000 piece, num_total_pieces_added is incremented by 1000.
    ses_num_piece_passed,
    ses_num_piece_failed,

    ses_num_have_pieces,
    ses_num_total_pieces_added,

    // the number of allowed unchoked peers
    ses_num_unchoke_slots,

    // the number of listen sockets that are currently accepting incoming
    // connections
    ses_num_outstanding_accept,

    // bittorrent message counters. These counters are incremented
    // every time a message of the corresponding type is received from
    // or sent to a bittorrent peer.
    ses_num_incoming_choke,
    ses_num_incoming_unchoke,
    ses_num_incoming_interested,
    ses_num_incoming_not_interested,
    ses_num_incoming_have,
    ses_num_incoming_bitfield,
    ses_num_incoming_request,
    ses_num_incoming_piece,
    ses_num_incoming_cancel,
    ses_num_incoming_dht_port,
    ses_num_incoming_suggest,
    ses_num_incoming_have_all,
    ses_num_incoming_have_none,
    ses_num_incoming_reject,
    ses_num_incoming_allowed_fast,
    ses_num_incoming_ext_handshake,
    ses_num_incoming_pex,
    ses_num_incoming_metadata,
    ses_num_incoming_extended,

    ses_num_outgoing_choke,
    ses_num_outgoing_unchoke,
    ses_num_outgoing_interested,
    ses_num_outgoing_not_interested,
    ses_num_outgoing_have,
    ses_num_outgoing_bitfield,
    ses_num_outgoing_request,
    ses_num_outgoing_piece,
    ses_num_outgoing_cancel,
    ses_num_outgoing_dht_port,
    ses_num_outgoing_suggest,
    ses_num_outgoing_have_all,
    ses_num_outgoing_have_none,
    ses_num_outgoing_reject,
    ses_num_outgoing_allowed_fast,
    ses_num_outgoing_ext_handshake,
    ses_num_outgoing_pex,
    ses_num_outgoing_metadata,
    ses_num_outgoing_extended,
    ses_num_outgoing_hash_request,
    ses_num_outgoing_hashes,
    ses_num_outgoing_hash_reject,

    // the number of wasted downloaded bytes by reason of the bytes being
    // wasted.
    ses_waste_piece_timed_out,
    ses_waste_piece_cancelled,
    ses_waste_piece_unknown,
    ses_waste_piece_seed,
    ses_waste_piece_end_game,
    ses_waste_piece_closing,

    // the number of pieces considered while picking pieces
    picker_piece_picker_partial_loops,
    picker_piece_picker_suggest_loops,
    picker_piece_picker_sequential_loops,
    picker_piece_picker_reverse_rare_loops,
    picker_piece_picker_rare_loops,
    picker_piece_picker_rand_start_loops,
    picker_piece_picker_rand_loops,
    picker_piece_picker_busy_loops,

    // This breaks down the piece picks into the event that
    // triggered it
    picker_reject_piece_picks,
    picker_unchoke_piece_picks,
    picker_incoming_redundant_piece_picks,
    picker_incoming_piece_picks,
    picker_end_game_piece_picks,
    picker_snubbed_piece_picks,
    picker_interesting_piece_picks,
    picker_hash_fail_piece_picks,

    // the number of microseconds it takes from receiving a request from a
    // peer until we're sending the response back on the socket.
    disk_request_latency,

    disk_disk_blocks_in_use,

    // ``queued_disk_jobs`` is the number of disk jobs currently queued,
    // waiting to be executed by a disk thread.
    disk_queued_disk_jobs,
    disk_num_running_disk_jobs,
    disk_num_read_jobs,
    disk_num_write_jobs,
    disk_num_jobs,
    disk_blocked_disk_jobs,

    disk_num_writing_threads,
    disk_num_running_threads,

    // the number of bytes we have sent to the disk I/O
    // thread for writing. Every time we hear back from
    // the disk I/O thread with a completed write job, this
    // is updated to the number of bytes the disk I/O thread
    // is actually waiting for to be written (as opposed to
    // bytes just hanging out in the cache)
    disk_queued_write_bytes,

    // the number of blocks written and read from disk in total. A block is 16
    // kiB. ``num_blocks_written`` and ``num_blocks_read``
    disk_num_blocks_written,
    disk_num_blocks_read,

    // the total number of blocks run through SHA-1 hashing
    disk_num_blocks_hashed,

    // the number of disk I/O operation for reads and writes. One disk
    // operation may transfer more then one block.
    disk_num_write_ops,
    disk_num_read_ops,

    // the number of blocks that had to be read back from disk in order to
    // hash a piece (when verifying against the piece hash)
    disk_num_read_back,

    // cumulative time spent in various disk jobs, as well
    // as total for all disk jobs. Measured in microseconds
    disk_disk_read_time,
    disk_disk_write_time,
    disk_disk_hash_time,
    disk_disk_job_time,

    // for each kind of disk job, a counter of how many jobs of that kind
    // are currently blocked by a disk fence
    disk_num_fenced_read,
    disk_num_fenced_write,
    disk_num_fenced_hash,
    disk_num_fenced_move_storage,
    disk_num_fenced_release_files,
    disk_num_fenced_delete_files,
    disk_num_fenced_check_fastresume,
    disk_num_fenced_save_resume_data,
    disk_num_fenced_rename_file,
    disk_num_fenced_stop_torrent,
    disk_num_fenced_flush_piece,
    disk_num_fenced_flush_hashed,
    disk_num_fenced_flush_storage,
    disk_num_fenced_file_priority,
    disk_num_fenced_load_torrent,
    disk_num_fenced_clear_piece,
    disk_num_fenced_tick_storage,

    // The number of nodes in the DHT routing table
    dht_dht_nodes,

    // The number of replacement nodes in the DHT routing table
    dht_dht_node_cache,

    // the number of torrents currently tracked by our DHT node
    dht_dht_torrents,

    // the number of peers currently tracked by our DHT node
    dht_dht_peers,

    // the number of immutable data items tracked by our DHT node
    dht_dht_immutable_data,

    // the number of mutable data items tracked by our DHT node
    dht_dht_mutable_data,

    // the number of RPC observers currently allocated
    dht_dht_allocated_observers,

    // the total number of DHT messages sent and received
    dht_dht_messages_in,
    dht_dht_messages_out,

    // the number of incoming DHT requests that were dropped. There are a few
    // different reasons why incoming DHT packets may be dropped:
    //
    // 1. there wasn't enough send quota to respond to them.
    // 2. the Denial of service logic kicked in, blocking the peer
    // 3. ignore_dark_internet is enabled, and the packet came from a
    //    non-public IP address
    // 4. the bencoding of the message was invalid
    dht_dht_messages_in_dropped,

    // the number of outgoing messages that failed to be
    // sent
    dht_dht_messages_out_dropped,

    // the total number of bytes sent and received by the DHT
    dht_dht_bytes_in,
    dht_dht_bytes_out,

    // the number of DHT messages we've sent and received
    // by kind.
    dht_dht_ping_in,
    dht_dht_ping_out,
    dht_dht_find_node_in,
    dht_dht_find_node_out,
    dht_dht_get_peers_in,
    dht_dht_get_peers_out,
    dht_dht_announce_peer_in,
    dht_dht_announce_peer_out,
    dht_dht_get_in,
    dht_dht_get_out,
    dht_dht_put_in,
    dht_dht_put_out,
    dht_dht_sample_infohashes_in,
    dht_dht_sample_infohashes_out,

    // the number of failed incoming DHT requests by kind of request
    dht_dht_invalid_announce,
    dht_dht_invalid_get_peers,
    dht_dht_invalid_find_node,
    dht_dht_invalid_put,
    dht_dht_invalid_get,
    dht_dht_invalid_sample_infohashes,

    // The number of times a lost packet has been interpreted as congestion,
    // cutting the congestion window in half. Some lost packets are not
    // interpreted as congestion, notably MTU-probes
    utp_utp_packet_loss,

    // The number of timeouts experienced. This is when a connection doesn't
    // hear back from the other end within a sliding average RTT + 2 average
    // deviations from the mean (approximately). The actual time out is
    // configurable and also depends on the state of the socket.
    utp_utp_timeout,

    // The total number of packets sent and received
    utp_utp_packets_in,
    utp_utp_packets_out,

    // The number of packets lost but re-sent by the fast-retransmit logic.
    // This logic is triggered after 3 duplicate ACKs.
    utp_utp_fast_retransmit,

    // The number of packets that were re-sent, for whatever reason
    utp_utp_packet_resend,

    // The number of incoming packets where the delay samples were above
    // and below the delay target, respectively. The delay target is
    // configurable and is a parameter to the LEDBAT congestion control.
    utp_utp_samples_above_target,
    utp_utp_samples_below_target,

    // The total number of packets carrying payload received and sent,
    // respectively.
    utp_utp_payload_pkts_in,
    utp_utp_payload_pkts_out,

    // The number of packets received that are not valid uTP packets (but
    // were sufficiently similar to not be treated as DHT or UDP tracker
    // packets).
    utp_utp_invalid_pkts_in,

    // The number of duplicate payload packets received. This may happen if
    // the outgoing ACK is lost.
    utp_utp_redundant_pkts_in,

    // the number of uTP sockets in each respective state
    utp_num_utp_idle,
    utp_num_utp_syn_sent,
    utp_num_utp_connected,
    utp_num_utp_fin_sent,
    utp_num_utp_close_wait,
    utp_num_utp_deleted,

    // the buffer sizes accepted by
    // socket send and receive calls respectively.
    // The larger the buffers are, the more efficient,
    // because it require fewer system calls per byte.
    // The size is 1 << n, where n is the number
    // at the end of the counter name. i.e.
    // 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192,
    // 16384, 32768, 65536, 131072, 262144, 524288, 1048576
    // bytes
    sock_bufs_socket_send_size3,
    sock_bufs_socket_send_size4,
    sock_bufs_socket_send_size5,
    sock_bufs_socket_send_size6,
    sock_bufs_socket_send_size7,
    sock_bufs_socket_send_size8,
    sock_bufs_socket_send_size9,
    sock_bufs_socket_send_size10,
    sock_bufs_socket_send_size11,
    sock_bufs_socket_send_size12,
    sock_bufs_socket_send_size13,
    sock_bufs_socket_send_size14,
    sock_bufs_socket_send_size15,
    sock_bufs_socket_send_size16,
    sock_bufs_socket_send_size17,
    sock_bufs_socket_send_size18,
    sock_bufs_socket_send_size19,
    sock_bufs_socket_send_size20,
    sock_bufs_socket_recv_size3,
    sock_bufs_socket_recv_size4,
    sock_bufs_socket_recv_size5,
    sock_bufs_socket_recv_size6,
    sock_bufs_socket_recv_size7,
    sock_bufs_socket_recv_size8,
    sock_bufs_socket_recv_size9,
    sock_bufs_socket_recv_size10,
    sock_bufs_socket_recv_size11,
    sock_bufs_socket_recv_size12,
    sock_bufs_socket_recv_size13,
    sock_bufs_socket_recv_size14,
    sock_bufs_socket_recv_size15,
    sock_bufs_socket_recv_size16,
    sock_bufs_socket_recv_size17,
    sock_bufs_socket_recv_size18,
    sock_bufs_socket_recv_size19,
    sock_bufs_socket_recv_size20,

    // if the outstanding tracker announce limit is reached, tracker
    // announces are queued, to be issued when an announce slot opens up.
    // this measure the number of tracker announces currently in the
    // queue
    tracker_num_queued_tracker_announces,
    // }}}
}

impl Metrics {
    pub fn index(self) -> usize {
        self as usize
    }
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
pub struct SessionStats {
    pub two_session_stats: TwoSessionStats,
}
