#![allow(non_camel_case_types)]

use bitflags::bitflags;

bitflags! {
    /// libtorrent/session_handler.hpp
    /// These flags are defined in struct session_handle
    pub struct SaveStateFlags: u32 {
        /// saves settings (i.e. the settings_pack)
        /// static constexpr save_state_flags_t save_settings = 0_bit;
        const save_settings = 1 << 0;

        /// saves dht state such as nodes and node-id, possibly accelerating
        /// joining the DHT if provided at next session startup.
        /// static constexpr save_state_flags_t save_dht_state = 2_bit;
        const save_dht_state = 1 << 2;

        /// load or save state from plugins
        /// static constexpr save_state_flags_t save_extension_state = 11_bit;
        const save_extension_state = 1 << 11;

        /// load or save the IP filter set on the session
        /// static constexpr save_state_flags_t save_ip_filter = 12_bit;
        const save_ip_filter = 1 << 12;
    }
}

bitflags! {
    /// libtorrent/torrent_flags.hpp
    /// These flags are defined in namespace torrent_flags
    pub struct TorrentFlags: u64 {
        /// If ``seed_mode`` is set, libtorrent will assume that all files
        /// are present for this torrent and that they all match the hashes in
        /// the torrent file. Each time a peer requests to download a block,
        /// the piece is verified against the hash, unless it has been verified
        /// already. If a hash fails, the torrent will automatically leave the
        /// seed mode and recheck all the files. The use case for this mode is
        /// if a torrent is created and seeded, or if the user already know
        /// that the files are complete, this is a way to avoid the initial
        /// file checks, and significantly reduce the startup time.
        ///
        /// Setting ``seed_mode`` on a torrent without metadata (a
        /// .torrent file) is a no-op and will be ignored.
        ///
        /// It is not possible to *set* the ``seed_mode`` flag on a torrent after it has
        /// been added to a session. It is possible to *clear* it though.
        /// constexpr torrent_flags_t seed_mode = 0_bit;
        const seed_mode = 1 << 0;

        /// If ``upload_mode`` is set, the torrent will be initialized in
        /// upload-mode, which means it will not make any piece requests. This
        /// state is typically entered on disk I/O errors, and if the torrent
        /// is also auto managed, it will be taken out of this state
        /// periodically (see ``settings_pack::optimistic_disk_retry``).
        ///
        /// This mode can be used to avoid race conditions when
        /// adjusting priorities of pieces before allowing the torrent to start
        /// downloading.
        ///
        /// If the torrent is auto-managed (``auto_managed``), the torrent
        /// will eventually be taken out of upload-mode, regardless of how it
        /// got there. If it's important to manually control when the torrent
        /// leaves upload mode, don't make it auto managed.
        /// constexpr torrent_flags_t upload_mode = 1_bit;
        const upload_mode = 1 << 1;

        /// determines if the torrent should be added in *share mode* or not.
        /// Share mode indicates that we are not interested in downloading the
        /// torrent, but merely want to improve our share ratio (i.e. increase
        /// it). A torrent started in share mode will do its best to never
        /// download more than it uploads to the swarm. If the swarm does not
        /// have enough demand for upload capacity, the torrent will not
        /// download anything. This mode is intended to be safe to add any
        /// number of torrents to, without manual screening, without the risk
        /// of downloading more than is uploaded.
        ///
        /// A torrent in share mode sets the priority to all pieces to 0,
        /// except for the pieces that are downloaded, when pieces are decided
        /// to be downloaded. This affects the progress bar, which might be set
        /// to "100% finished" most of the time. Do not change file or piece
        /// priorities for torrents in share mode, it will make it not work.
        ///
        /// The share mode has one setting, the share ratio target, see
        /// ``settings_pack::share_mode_target`` for more info.
        /// constexpr torrent_flags_t share_mode = 2_bit;
        const share_mode = 1 << 2;

        /// determines if the IP filter should apply to this torrent or not. By
        /// default all torrents are subject to filtering by the IP filter
        /// (i.e. this flag is set by default). This is useful if certain
        /// torrents needs to be exempt for some reason, being an auto-update
        /// torrent for instance.
        /// constexpr torrent_flags_t apply_ip_filter = 3_bit;
        const apply_ip_filter = 1 << 3;

        /// specifies whether or not the torrent is paused. i.e. it won't connect to the tracker or any of the peers
        /// until it's resumed. Note that a paused torrent that also has the
        /// auto_managed flag set can be started at any time by libtorrent's queuing
        /// logic. See queuing_.
        /// constexpr torrent_flags_t paused = 4_bit;
        const paused = 1 << 4;

        /// If the torrent is auto-managed (``auto_managed``), the torrent
        /// may be resumed at any point, regardless of how it paused. If it's
        /// important to manually control when the torrent is paused and
        /// resumed, don't make it auto managed.
        ///
        /// If ``auto_managed`` is set, the torrent will be queued,
        /// started and seeded automatically by libtorrent. When this is set,
        /// the torrent should also be started as paused. The default queue
        /// order is the order the torrents were added. They are all downloaded
        /// in that order. For more details, see queuing_.
        /// constexpr torrent_flags_t auto_managed = 5_bit;
        const auto_managed = 1 << 5;

        /// used in add_torrent_params to indicate that it's an error to attempt
        /// to add a torrent that's already in the session. If it's not considered an
        /// error, a handle to the existing torrent is returned.
        /// This flag is not saved by write_resume_data(), since it is only meant for
        /// adding torrents.
        /// constexpr torrent_flags_t duplicate_is_error = 6_bit;
        const duplicate_is_error = 1 << 6;

        /// on by default and means that this torrent will be part of state
        /// updates when calling post_torrent_updates().
        /// This flag is not saved by write_resume_data().
        /// constexpr torrent_flags_t update_subscribe = 7_bit;
        const update_subscribe = 1 << 7;

        /// sets the torrent into super seeding/initial seeding mode. If the torrent
        /// is not a seed, this flag has no effect.
        /// constexpr torrent_flags_t super_seeding = 8_bit;
        const super_seeding = 1 << 8;

        /// sets the sequential download state for the torrent. In this mode the
        /// piece picker will pick pieces with low index numbers before pieces with
        /// high indices. The actual pieces that are picked depend on other factors
        /// still, such as which pieces a peer has and whether it is in parole mode
        /// or "prefer whole pieces"-mode. Sequential mode is not ideal for streaming
        /// media. For that, see set_piece_deadline() instead.
        /// constexpr torrent_flags_t sequential_download = 9_bit;
        const sequential_download = 1 << 9;

        /// When this flag is set, the torrent will *force stop* whenever it
        /// transitions from a non-data-transferring state into a data-transferring
        /// state (referred to as being ready to download or seed). This is useful
        /// for torrents that should not start downloading or seeding yet, but want
        /// to be made ready to do so. A torrent may need to have its files checked
        /// for instance, so it needs to be started and possibly queued for checking
        /// (auto-managed and started) but as soon as it's done, it should be
        /// stopped.
        ///
        /// *Force stopped* means auto-managed is set to false and it's paused. As
        /// if the auto_manages flag is cleared and the paused flag is set on the torrent.
        ///
        /// Note that the torrent may transition into a downloading state while
        /// setting this flag, and since the logic is edge triggered you may
        /// miss the edge. To avoid this race, if the torrent already is in a
        /// downloading state when this call is made, it will trigger the
        /// stop-when-ready immediately.
        ///
        /// When the stop-when-ready logic fires, the flag is cleared. Any
        /// subsequent transitions between downloading and non-downloading states
        /// will not be affected, until this flag is set again.
        ///
        /// The behavior is more robust when setting this flag as part of adding
        /// the torrent. See add_torrent_params.
        ///
        /// The stop-when-ready flag fixes the inherent race condition of waiting
        /// for the state_changed_alert and then call pause(). The download/seeding
        /// will most likely start in between posting the alert and receiving the
        /// call to pause.
        ///
        /// A downloading state is one where peers are being connected. Which means
        /// just downloading the metadata via the ``ut_metadata`` extension counts
        /// as a downloading state. In order to stop a torrent once the metadata
        /// has been downloaded, instead set all file priorities to dont_download
        /// constexpr torrent_flags_t stop_when_ready = 10_bit;
        const stop_when_ready = 1 << 10;

        /// when this flag is set, the tracker list in the add_torrent_params
        /// object override any trackers from the torrent file. If the flag is
        /// not set, the trackers from the add_torrent_params object will be
        /// added to the list of trackers used by the torrent.
        /// This flag is set by read_resume_data() if there are trackers present in
        /// the resume data file. This effectively makes the trackers saved in the
        /// resume data take precedence over the original trackers. This includes if
        /// there's an empty list of trackers, to support the case where they were
        /// explicitly removed in the previous session.
        /// This flag is not saved by write_resume_data()
        /// constexpr torrent_flags_t override_trackers = 11_bit;
        const override_trackers = 1 << 11;

        /// If this flag is set, the web seeds from the add_torrent_params
        /// object will override any web seeds in the torrent file. If it's not
        /// set, web seeds in the add_torrent_params object will be added to the
        /// list of web seeds used by the torrent.
        /// This flag is set by read_resume_data() if there are web seeds present in
        /// the resume data file. This effectively makes the web seeds saved in the
        /// resume data take precedence over the original ones. This includes if
        /// there's an empty list of web seeds, to support the case where they were
        /// explicitly removed in the previous session.
        /// This flag is not saved by write_resume_data()
        /// constexpr torrent_flags_t override_web_seeds = 12_bit;
        const override_web_seeds = 1 << 12;

        /// if this flag is set (which it is by default) the torrent will be
        /// considered needing to save its resume data immediately, in the
        /// category if_metadata_changed. See resume_data_flags_t and
        /// save_resume_data() for details.
        ///
        /// This flag is cleared by a successful call to save_resume_data()
        /// This flag is not saved by write_resume_data(), since it represents an
        /// ephemeral state of a running torrent.
        /// constexpr torrent_flags_t need_save_resume = 13_bit;
        const need_save_resume = 1 << 13;

        /// set this flag to disable DHT for this torrent. This lets you have the DHT
        /// enabled for the whole client, and still have specific torrents not
        /// participating in it. i.e. not announcing to the DHT nor picking up peers
        /// from it.
        /// constexpr torrent_flags_t disable_dht = 19_bit;
        const disable_dht = 1 << 19;

        /// set this flag to disable local service discovery for this torrent.
        /// constexpr torrent_flags_t disable_lsd = 20_bit;
        const disable_lsd = 1 << 20;

        /// set this flag to disable peer exchange for this torrent.
        /// constexpr torrent_flags_t disable_pex = 21_bit;
        const disable_pex = 1 << 21;

        /// if this flag is set, the resume data will be assumed to be correct
        /// without validating it against any files on disk. This may be used when
        /// restoring a session by loading resume data from disk. It will save time
        /// and also delay any hard disk errors until files are actually needed. If
        /// the resume data cannot be trusted, or if a torrent is added for the first
        /// time to some save path that may already have some of the files, this flag
        /// should not be set.
        /// constexpr torrent_flags_t no_verify_files = 22_bit;
        const no_verify_files = 1 << 22;

        /// default all file priorities to dont_download. This is useful for adding
        /// magnet links where the number of files is unknown, but the
        /// file_priorities is still set for some files. Any file not covered by
        /// the file_priorities list will be set to normal download priority,
        /// unless this flag is set, in which case they will be set to 0
        /// (dont_download).
        /// constexpr torrent_flags_t default_dont_download = 23_bit;
        const default_dont_download = 1 << 23;

        /// this flag makes the torrent be considered an "i2p torrent" for purposes
        /// of the allow_i2p_mixed setting. When mixing regular peers and i2p peers
        /// is disabled, i2p torrents won't add normal peers to its peer list.
        /// Note that non i2p torrents may still allow i2p peers (on the off-chance
        /// that a tracker return them and the session is configured with a SAM
        /// connection).
        /// This flag is set automatically when adding a torrent that has at least
        /// one tracker whose hostname ends with .i2p.
        /// It's also set by parse_magnet_uri() if the tracker list contains such
        /// URL.
        /// constexpr torrent_flags_t i2p_torrent = 24_bit;
        const i2p_torrent = 1 << 24;
    }
}

bitflags! {
    /// libtorrent/torrent_flags.hpp
    /// These flags are defined in namespace torrent_flags
    pub struct PauseFlags: u8 {
        /// will delay the disconnect of peers that we're still downloading
        /// requests and will disconnect all idle peers. As soon as a peer is done
        /// outstanding requests from. The torrent will not accept any more
        /// transferring the blocks that were requested from it, it is
        /// disconnected. This is a graceful shut down of the torrent in the sense
        /// that no downloaded bytes are wasted.
        /// static constexpr pause_flags_t graceful_pause = 0_bit;
        const graceful_pause = 1 << 0;
    }
}

bitflags! {
    /// libtorrent/peer_info.hpp
    ///
    /// flags for the peer_info::flags field. Indicates various states
    /// the peer may be in. These flags are not mutually exclusive, but
    /// not every combination of them makes sense either.
    ///
    /// for peer_info.flags
    pub struct PeerFlags: u32 {
        /// **we** are interested in pieces from this peer.
        const interesting = 1 << 0;

        /// **we** have choked this peer.
        const choked = 1 << 1;

        /// the peer is interested in **us**
        const remote_interested = 1 << 2;

        /// the peer has choked **us**.
        const remote_choked = 1 << 3;

        /// means that this peer supports the
        /// `extension protocol`__.
        ///
        /// __ extension_protocol.html
        const supports_extensions = 1 << 4;

        /// The connection was initiated by us, the peer has a
        /// listen port open, and that port is the same as in the
        /// address of this peer. If this flag is not set, this
        /// peer connection was opened by this peer connecting to
        /// us.
        const outgoing_connection = 1 << 5;

        /// deprecated synonym for outgoing_connection
        const local_connection = 1 << 5;

        /// The connection is opened, and waiting for the
        /// handshake. Until the handshake is done, the peer
        /// cannot be identified.
        const handshake = 1 << 6;

        /// The connection is in a half-open state (i.e. it is
        /// being connected).
        const connecting = 1 << 7;

        /// The peer has participated in a piece that failed the
        /// hash check, and is now "on parole", which means we're
        /// only requesting whole pieces from this peer until
        /// it either fails that piece or proves that it doesn't
        /// send bad data.
        const on_parole = 1 << 9;

        /// This peer is a seed (it has all the pieces).
        const seed = 1 << 10;

        /// This peer is subject to an optimistic unchoke. It has
        /// been unchoked for a while to see if it might unchoke
        /// us in return an earn an upload/unchoke slot. If it
        /// doesn't within some period of time, it will be choked
        /// and another peer will be optimistically unchoked.
        const optimistic_unchoke = 1 << 11;

        /// This peer has recently failed to send a block within
        /// the request timeout from when the request was sent.
        /// We're currently picking one block at a time from this
        /// peer.
        const snubbed = 1 << 12;

        /// This peer has either explicitly (with an extension)
        /// or implicitly (by becoming a seed) told us that it
        /// will not downloading anything more, regardless of
        /// which pieces we have.
        const upload_only = 1 << 13;

        /// This means the last time this peer picket a piece,
        /// it could not pick as many as it wanted because there
        /// were not enough free ones. i.e. all pieces this peer
        /// has were already requested from other peers.
        const endgame_mode = 1 << 14;

        /// This flag is set if the peer was in holepunch mode
        /// when the connection succeeded. This typically only
        /// happens if both peers are behind a NAT and the peers
        /// connect via the NAT holepunch mechanism.
        const holepunched = 1 << 15;

        /// indicates that this socket is running on top of the
        /// I2P transport.
        const i2p_socket = 1 << 16;

        /// indicates that this socket is a uTP socket
        const utp_socket = 1 << 17;

        /// indicates that this socket is running on top of an SSL
        /// (TLS) channel
        const ssl_socket = 1 << 18;

        /// this connection is obfuscated with RC4
        const rc4_encrypted = 1 << 19;

        /// the handshake of this connection was obfuscated
        /// with a Diffie-Hellman exchange
        const plaintext_encrypted = 1 << 20;
    }
}

bitflags! {
    /// libtorrent/peer_info.hpp
    ///
    /// the flags indicating which sources a peer can
    /// have come from. A peer may have been seen from
    /// multiple sources
    ///
    /// for peer_info.source
    pub struct PeerSourceFlags: u8 {
        /// The peer was received from the tracker.
        const tracker = 1 << 0;

        /// The peer was received from the kademlia DHT.
        const dht = 1 << 1;

        // The peer was received from the peer exchange
        /// extension.
        const pex = 1 << 2;

        // The peer was received from the local service
        /// discovery (The peer is on the local network).
        const lsd = 1 << 3;

        /// The peer was added from the fast resume data.
        const resume_data = 1 << 4;

        /// we received an incoming connection from this peer
        const incoming = 1 << 5;
    }
}

bitflags! {
    /// libtorrent/peer_info.hpp
    ///
    /// flags indicating what is blocking network transfers in up- and down
    /// direction
    ///
    /// for peer_info.read_state and peer_info.write_state
    pub struct BandwidthStateFlags: u8 {
        /// The peer is not waiting for any external events to
        /// send or receive data.
        const bw_idle = 1 << 0;

        /// The peer is waiting for the rate limiter.
        const bw_limit = 1 << 1;

        /// The peer has quota and is currently waiting for a
        /// network read or write operation to complete. This is
        /// the state all peers are in if there are no bandwidth
        /// limits.
        const bw_network = 1 << 2;

        /// The peer is waiting for the disk I/O thread to catch
        /// up writing buffers to disk before downloading more.
        const bw_disk = 1 << 4;
    }
}

bitflags! {
    /// libtorrent/peer_info.hpp
    ///
    /// for peer_info.connection_type
    pub struct ConnectionType: u8 {
        /// Regular bittorrent connection
        const standard_bittorrent = 1 << 0;

        /// HTTP connection using the `BEP 19`_ protocol
        const web_seed = 1 << 1;

        /// HTTP connection using the `BEP 17`_ protocol
        const http_seed = 1 << 2;
    }
}
