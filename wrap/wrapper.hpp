#ifndef LIBTORRENT_WRAPPER_HPP_
#define LIBTORRENT_WRAPPER_HPP_

#include "../libtorrent/include/libtorrent/session.hpp"
#include "../libtorrent/include/libtorrent/torrent_handle.hpp"

#include "rust/cxx.h"
#include "states.hpp"

#include <memory>

namespace libtorrent_wrapper {

// shared types
struct DHTNode;
struct FileEntry;
struct TorrentInfo;
struct ParamPair;
struct PeerInfo;

class Session {

public:
  Session(lt::session_params params, std::string session_state_path,
          std::string resume_dir, std::string torrent_dir);
  ~Session();

  void add_torrent_from_parmas(lt::add_torrent_params atp,
                               rust::Slice<const ParamPair> torrent_param_list) const;

  void add_torrent(rust::Str torrent_path,
                   rust::Slice<const ParamPair> torrent_param_list) const;

  void add_magnet(rust::Str magnet_uri,
                  rust::Slice<const ParamPair> torrent_param_list) const;

  rust::Vec<TorrentInfo> get_torrents() const;

  TorrentInfo get_torrent_info(rust::Str info_hash_str) const;

  void get_peers(rust::Str info_hash_str);

  void get_file_progress(rust::Str info_hash_str, bool piece_granularity);

  void get_piece_info(rust::Str info_hash_str);

  void get_piece_availability(rust::Str info_hash_str);

  void get_trackers(rust::Str info_hash_str);

  void scrape_tracker(rust::Str info_hash_str);

private:
  std::string m_session_state_path;
  std::string m_resume_dir;
  std::string m_torrent_dir;

  std::shared_ptr<lt::session> lt_session;

  std::string get_resume_file_path(std::string info_hash_str) const;

  SessionStats m_session_stats;
  TorrentState m_torrent_state;
  DHTStats m_dht_stats;

  PeerState m_peer_state;
  FileProgressState m_file_progress_state;
  PieceInfoState m_piece_info_state;
  PieceAvailabilityState m_piece_availability_state;
  TrackerState m_tracker_state;

  void poll_alerts();

  std::mutex m_pop_alerts_mutex; // protects pop_alerts
  void pop_alerts();

  void handle_alert(lt::alert* a);

  lt::torrent_handle find_torrent_handle(rust::Str info_hash_str) const;
};

std::unique_ptr<Session> create_session(rust::Slice<const ParamPair> session_param_list,
                                        std::uint32_t save_state_flags,
                                        rust::Str session_state_path,
                                        rust::Str resume_dir, rust::Str torrent_dir);

} // namespace libtorrent_wrapper

#endif
