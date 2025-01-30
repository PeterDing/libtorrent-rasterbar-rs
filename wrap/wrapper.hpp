#ifndef LIBTORRENT_WRAPPER_HPP_
#define LIBTORRENT_WRAPPER_HPP_

#include "../libtorrent/include/libtorrent/session.hpp"
#include "../libtorrent/include/libtorrent/torrent_handle.hpp"

#include "rust/cxx.h"
#include "states.hpp"

#include <memory>

namespace libtorrent_wrapper {

// shared types
struct ParamPair;
struct DHTNode;
struct FileEntry;
struct TorrentInfo;
struct PeerInfo;
struct AnnounceInfoHash;
struct AnnounceEndpoint;
struct AnnounceEntry;

class Session {

public:
  Session(lt::session_params params, std::string session_state_path,
          std::string resume_dir, std::string torrent_dir);
  ~Session();

  void add_torrent(rust::Str torrent_path,
                   rust::Slice<const ParamPair> torrent_param_list) const;

  void add_magnet(rust::Str magnet_uri,
                  rust::Slice<const ParamPair> torrent_param_list) const;

  void remove_torrent(rust::Str info_hash_str, bool delete_files) const;

  void scrape_tracker(rust::Str info_hash_str) const;
  void force_recheck(rust::Str info_hash_str) const;
  void force_reannounce(rust::Str info_hash_str) const;
  void clear_error(rust::Str info_hash_str) const;

  void pause() const;
  void resume() const;
  bool is_paused() const;

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
  void set_flags(rust::Str info_hash_str, std::uint64_t flags) const;
  void set_flags_with_mask(rust::Str info_hash_str, std::uint64_t flags,
                           std::uint64_t mask) const;
  void unset_flags(rust::Str info_hash_str, std::uint64_t flags) const;

  rust::Vec<TorrentInfo> get_torrents() const;

  TorrentInfo get_torrent_info(rust::Str info_hash_str) const;

  rust::Vec<PeerInfo> get_peers(rust::Str info_hash_str);

  rust::Vec<std::int64_t> get_file_progress(rust::Str info_hash_str,
                                            bool piece_granularity);

  void get_piece_info(rust::Str info_hash_str);

  rust::Vec<std::int32_t> get_piece_availability(rust::Str info_hash_str);

  rust::Vec<AnnounceEntry> get_trackers(rust::Str info_hash_str);

  void poll_alerts();

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

  void handle_alert(lt::alert* a);

  lt::torrent_handle find_torrent_handle(rust::Str info_hash_str) const;

  bool m_running;
  std::shared_ptr<std::thread> m_thread;
};

std::unique_ptr<Session> create_session(rust::Slice<const ParamPair> session_param_list,
                                        std::uint32_t save_state_flags,
                                        rust::Str session_state_path,
                                        rust::Str resume_dir, rust::Str torrent_dir);

} // namespace libtorrent_wrapper

#endif
