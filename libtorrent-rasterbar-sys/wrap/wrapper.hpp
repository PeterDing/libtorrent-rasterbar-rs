#ifndef LIBTORRENT_WRAPPER_HPP_
#define LIBTORRENT_WRAPPER_HPP_

#include "../libtorrent/include/libtorrent/session.hpp"
#include "../libtorrent/include/libtorrent/torrent_handle.hpp"

#include "rust/cxx.h"
#include "states.hpp"
#include "time.hpp"

#include <deque>
#include <memory>

namespace libtorrent_wrapper {

// shared types
struct ParamPair;
struct DHTNode;
struct FileEntry;
struct TorrentInfo;
struct TorrentStatus;
struct PeerInfo;
struct PartialPieceInfo;
struct BlockInfo;
struct PieceInfo;
struct AnnounceInfoHash;
struct AnnounceEndpoint;
struct AnnounceEntry;
struct Log;
struct TwoSessionStats;

class TorrentHandle;

class Session {
  friend class TorrentHandle;

public:
  Session(lt::session_params params, std::string session_state_path,
          std::string resume_dir, std::string torrent_dir, std::uint32_t log_size);
  ~Session();

  void add_torrent(rust::Str torrent_path,
                   rust::Slice<const ParamPair> torrent_param_list) const;

  void add_magnet(rust::Str magnet_uri,
                  rust::Slice<const ParamPair> torrent_param_list) const;

  std::unique_ptr<TorrentHandle> get_torrent_handle(rust::Str info_hash_str) const;

  void remove_torrent(rust::Str info_hash_str, bool delete_files) const;

  TwoSessionStats get_stats() const;

  void pause() const;
  void resume() const;
  bool is_paused() const;

  rust::Vec<TorrentInfo> get_torrents() const;

  rust::Vec<TorrentStatus> get_all_torrent_status() const;

  void poll_alerts();

  rust::Vec<Log> get_logs();

private:
  void add_torrent_from_parmas(lt::add_torrent_params atp,
                               rust::Slice<const ParamPair> torrent_param_list) const;

  std::string get_resume_file_path(lt::sha1_hash info_hash) const;
  std::string get_torrent_file_path(lt::sha1_hash info_hash) const;

  void load_all_resume_data() const;

  void pop_alerts();

  bool handle_alert(lt::alert* a);

  lt::torrent_handle find_torrent_handle(rust::Str info_hash_str) const;

  void save_all_resume() const;

  std::string m_session_state_path;
  std::string m_resume_dir;
  std::string m_torrent_dir;

  std::shared_ptr<lt::session> lt_session;

  SessionStats m_session_stats;
  TorrentState m_torrent_state;
  DHTStats m_dht_stats;

  PeerState m_peer_state;
  FileProgressState m_file_progress_state;
  PieceInfoState m_piece_info_state;
  PieceAvailabilityState m_piece_availability_state;
  TrackerState m_tracker_state;

  std::mutex m_pop_alerts_mutex; // protects pop_alerts

  bool m_running;
  std::shared_ptr<std::thread> m_thread;

  std::uint32_t m_log_size;
  std::deque<std::pair<lt::time_point, std::string>> m_events; // for log
};

// The default values of the session settings are set for a regular
// bittorrent client running on a desktop system. There are functions that
// can set the session settings to pre set settings for other environments.
// These can be used for the basis, and should be tweaked to fit your needs
// better.
//
// ``min_memory_usage`` returns settings that will use the minimal amount of
// RAM, at the potential expense of upload and download performance. It
// adjusts the socket buffer sizes, disables the disk cache, lowers the send
// buffer watermarks so that each connection only has at most one block in
// use at any one time. It lowers the outstanding blocks send to the disk
// I/O thread so that connections only have one block waiting to be flushed
// to disk at any given time. It lowers the max number of peers in the peer
// list for torrents. It performs multiple smaller reads when it hashes
// pieces, instead of reading it all into memory before hashing.
//
// This configuration is intended to be the starting point for embedded
// devices. It will significantly reduce memory usage.
//
// ``high_performance_seed`` returns settings optimized for a seed box,
// serving many peers and that doesn't do any downloading. It has a 128 MB
// disk cache and has a limit of 400 files in its file pool. It support fast
// upload rates by allowing large send buffers.
//
// ``session_param_list`` is a list of key-value pairs that will be used to
// override the default values.
std::unique_ptr<Session> create_session(bool min_memory_usage, bool high_performance_seed,
                                        rust::Slice<const ParamPair> session_param_list,
                                        std::uint32_t save_state_flags,
                                        rust::Str session_state_path,
                                        rust::Str resume_dir, rust::Str torrent_dir,
                                        std::uint32_t log_size);

class TorrentHandle {
public:
  TorrentHandle(lt::torrent_handle lt_torrent_handle, Session* session);
  ~TorrentHandle();

  bool is_valid() const { return m_torrent_handle.is_valid(); }

  void add_tracker(rust::Str tracker_url, std::uint8_t tier) const;

  void scrape_tracker() const;
  void force_recheck() const;

  // ``force_reannounce()`` will force this torrent to do another tracker
  // request, to receive new peers. The ``seconds`` argument specifies how
  // many seconds from now to issue the tracker announces.
  //
  // If the tracker's ``min_interval`` has not passed since the last
  // announce, the forced announce will be scheduled to happen immediately
  // as the ``min_interval`` expires. This is to honor trackers minimum
  // re-announce interval settings.
  //
  // The ``tracker_index`` argument specifies which tracker to re-announce.
  // If set to -1 (which is the default), all trackers are re-announce.
  //
  // The ``flags`` argument can be used to affect the re-announce. See
  // ignore_min_interval.
  //
  // ``force_dht_announce`` will announce the torrent to the DHT
  // immediately.
  //
  // ``force_lsd_announce`` will announce the torrent on LSD
  // immediately.
  void force_reannounce() const;
  void force_dht_announce() const;
  void force_lsd_announce() const;

  void clear_error() const;

  // ``set_upload_limit`` will limit the upload bandwidth used by this
  // particular torrent to the limit you set. It is given as the number of
  // bytes per second the torrent is allowed to upload.
  // ``set_download_limit`` works the same way but for download bandwidth
  // instead of upload bandwidth. Note that setting a higher limit on a
  // torrent then the global limit
  // (``settings_pack::upload_rate_limit``) will not override the global
  // rate limit. The torrent can never upload more than the global rate
  // limit.
  //
  // ``upload_limit`` and ``download_limit`` will return the current limit
  // setting, for upload and download, respectively.
  //
  // Local peers are not rate limited by default. see peer-classes_.
  void set_upload_limit(int limit) const;
  int upload_limit() const;
  void set_download_limit(int limit) const;
  int download_limit() const;

  // This will disconnect all peers and clear the peer list for this
  // torrent. New peers will have to be acquired before resuming, from
  // trackers, DHT or local service discovery, for example.
  void clear_peers() const;

  // ``set_max_uploads()`` sets the maximum number of peers that's unchoked
  // at the same time on this torrent. If you set this to -1, there will be
  // no limit. This defaults to infinite. The primary setting controlling
  // this is the global unchoke slots limit, set by unchoke_slots_limit in
  // settings_pack.
  //
  // ``max_uploads()`` returns the current settings.
  void set_max_uploads(int max_uploads) const;
  int max_uploads() const;

  // ``set_max_connections()`` sets the maximum number of connection this
  // torrent will open. If all connections are used up, incoming
  // connections may be refused or poor connections may be closed. This
  // must be at least 2. The default is unlimited number of connections. If
  // -1 is given to the function, it means unlimited. There is also a
  // global limit of the number of connections, set by
  // ``connections_limit`` in settings_pack.
  //
  // ``max_connections()`` returns the current settings.
  void set_max_connections(int max_connections) const;
  int max_connections() const;

  // ``pause()``, and ``resume()`` will disconnect all peers and reconnect
  // all peers respectively. When a torrent is paused, it will however
  // remember all share ratios to all peers and remember all potential (not
  // connected) peers. Torrents may be paused automatically if there is a
  // file error (e.g. disk full) or something similar. See
  // file_error_alert.
  //
  // For possible values of the ``flags`` parameter, see pause_flags_t.
  //
  // To know if a torrent is paused or not, call
  // ``torrent_handle::flags()`` and check for the
  // ``torrent_status::paused`` flag.
  //
  // .. note::
  // 	Torrents that are auto-managed may be automatically resumed again. It
  // 	does not make sense to pause an auto-managed torrent without making it
  // 	not auto-managed first. Torrents are auto-managed by default when added
  //
  // 	to the session. For more information, see queuing_.
  void pause(uint8_t flags) const;
  void resume() const;

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
  std::uint64_t flags() const;
  void set_flags(std::uint64_t flags) const;
  void set_flags_with_mask(std::uint64_t flags, std::uint64_t mask) const;
  void unset_flags(std::uint64_t flags) const;

  // ``index`` must be in the range [0, number_of_files).
  //
  // ``file_priority()`` queries or sets the priority of file ``index``.
  //
  // ``prioritize_files()`` takes a vector that has at as many elements as
  // there are files in the torrent. Each entry is the priority of that
  // file. The function sets the priorities of all the pieces in the
  // torrent based on the vector.
  //
  // ``get_file_priorities()`` returns a vector with the priorities of all
  // files.
  //
  // The priority values are the same as for piece_priority(). See
  // download_priority_t.
  //
  // Whenever a file priority is changed, all other piece priorities are
  // reset to match the file priorities. In order to maintain special
  // priorities for particular pieces, piece_priority() has to be called
  // again for those pieces.
  //
  // You cannot set the file priorities on a torrent that does not yet have
  // metadata or a torrent that is a seed. ``file_priority(int, int)`` and
  // prioritize_files() are both no-ops for such torrents.
  //
  // Since changing file priorities may involve disk operations (of moving
  // files in- and out of the part file), the internal accounting of file
  // priorities happen asynchronously. i.e. setting file priorities and then
  // immediately querying them may not yield the same priorities just set.
  // To synchronize with the priorities taking effect, wait for the
  // file_prio_alert.
  //
  // When combining file- and piece priorities, the resume file will record
  // both. When loading the resume data, the file priorities will be applied
  // first, then the piece priorities.
  //
  // Moving data from a file into the part file is currently not
  // supported. If a file has its priority set to 0 *after* it has already
  // been created, it will not be moved into the partfile.
  void set_file_priority(std::int32_t index, std::uint8_t priority) const;
  std::uint8_t get_file_priority(std::int32_t index) const;
  void set_prioritize_files(rust::Slice<const std::uint8_t> const files) const;
  rust::Vec<std::uint8_t> get_file_priorities() const;

  TorrentInfo get_torrent_info() const;

  rust::Vec<PeerInfo> get_peers() const;

  rust::Vec<std::int64_t> get_file_progress(bool piece_granularity) const;

  PieceInfo get_piece_info() const;

  rust::Vec<std::int32_t> get_piece_availability() const;

  rust::Vec<AnnounceEntry> get_trackers() const;

  TorrentStatus get_torrent_status() const;

  rust::String make_magnet_uri() const;

private:
  lt::torrent_handle m_torrent_handle;
  Session* m_session;
};

} // namespace libtorrent_wrapper

#endif
