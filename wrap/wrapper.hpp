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
struct PeerInfo;
struct PartialPieceInfo;
struct BlockInfo;
struct PieceInfo;
struct AnnounceInfoHash;
struct AnnounceEndpoint;
struct AnnounceEntry;
struct Log;

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

  void pause() const;
  void resume() const;
  bool is_paused() const;

  rust::Vec<TorrentInfo> get_torrents() const;

  void poll_alerts();

  rust::Vec<Log> get_logs();

private:
  void add_torrent_from_parmas(lt::add_torrent_params atp,
                               rust::Slice<const ParamPair> torrent_param_list) const;

  std::string m_session_state_path;
  std::string m_resume_dir;
  std::string m_torrent_dir;

  std::shared_ptr<lt::session> lt_session;

  std::string get_resume_file_path(lt::sha1_hash info_hash) const;

  SessionStats m_session_stats;
  TorrentState m_torrent_state;
  DHTStats m_dht_stats;

  PeerState m_peer_state;
  FileProgressState m_file_progress_state;
  PieceInfoState m_piece_info_state;
  PieceAvailabilityState m_piece_availability_state;
  TrackerState m_tracker_state;

  std::mutex m_pop_alerts_mutex; // protects pop_alerts
  void pop_alerts();

  bool handle_alert(lt::alert* a);

  lt::torrent_handle find_torrent_handle(rust::Str info_hash_str) const;

  bool m_running;
  std::shared_ptr<std::thread> m_thread;

  std::uint32_t m_log_size;
  std::deque<std::pair<lt::time_point, std::string>> m_events; // for log
};

std::unique_ptr<Session> create_session(rust::Slice<const ParamPair> session_param_list,
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
  void force_reannounce() const;
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
  void set_flags(std::uint64_t flags) const;
  void set_flags_with_mask(std::uint64_t flags, std::uint64_t mask) const;
  void unset_flags(std::uint64_t flags) const;

  TorrentInfo get_torrent_info() const;

  rust::Vec<PeerInfo> get_peers() const;

  rust::Vec<std::int64_t> get_file_progress(bool piece_granularity) const;

  PieceInfo get_piece_info() const;

  rust::Vec<std::int32_t> get_piece_availability() const;

  rust::Vec<AnnounceEntry> get_trackers() const;

private:
  lt::torrent_handle m_torrent_handle;
  Session* m_session;
};

} // namespace libtorrent_wrapper

#endif
