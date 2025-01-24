#ifndef LIBTORRENT_WRAPPER_STATES_HPP_
#define LIBTORRENT_WRAPPER_STATES_HPP_

#include "../libtorrent/include/libtorrent/alert_types.hpp"
#include "../libtorrent/include/libtorrent/announce_entry.hpp"
#include "../libtorrent/include/libtorrent/peer_info.hpp"
#include "../libtorrent/include/libtorrent/session_stats.hpp"
#include "../libtorrent/include/libtorrent/time.hpp"
#include "../libtorrent/include/libtorrent/torrent_handle.hpp"
#include "../libtorrent/include/libtorrent/torrent_status.hpp"

#include <cstdint>
#include <unordered_map>
#include <vector>

namespace libtorrent_wrapper {

class SessionStats {
public:
  SessionStats();
  ~SessionStats();

  std::vector<lt::stats_metric> stats_metrics();

  void update_counters(lt::session_stats_alert* a);

  std::vector<std::int64_t> stats() const;
  std::vector<std::int64_t> prev_stats() const;

  std::int64_t value(int idx) const;
  std::int64_t prev_value(int idx) const;

private:
  // the session_stats_metrics = lt::session_stats_metrics();
  std::vector<lt::stats_metric> m_stats_metrics;

  // there are two sets of counters. the current one and the last one. This
  // is used to calculate rates
  std::vector<std::int64_t> m_cnt[2];

  // the timestamps of the counters in m_cnt[0] and m_cnt[1]
  // respectively.
  lt::clock_type::time_point m_timestamp[2];
};

class TorrentState {
public:
  TorrentState();
  ~TorrentState();

  void update_torrents(lt::state_update_alert* a);
  void remove(lt::torrent_handle h);

private:
  // torrent handle -> torrent status
  std::unordered_map<lt::torrent_handle, lt::torrent_status> m_all_torrents;
};

class DHTStats {
public:
  DHTStats();
  ~DHTStats();

  void update_dht_stats(lt::dht_stats_alert* a);

private:
  // a vector of the currently running DHT lookups.
  std::vector<lt::dht_lookup> active_requests;

  // contains information about every bucket in the DHT routing
  // table.
  std::vector<lt::dht_routing_bucket> routing_table;

  // the node ID of the DHT node instance
  lt::sha1_hash nid;

  // the local socket this DHT node is running on
  lt::aux::noexcept_movable<lt::udp::endpoint> local_endpoint;
};

class PeerState {
public:
  PeerState();
  ~PeerState();

  void update_peers(lt::peer_info_alert* a);
  void remove(lt::torrent_handle h);

private:
  // torrent handle -> list of peer info
  std::unordered_map<lt::torrent_handle, std::vector<lt::peer_info>> m_all_peers;
};

class FileProgressState {
public:
  FileProgressState();
  ~FileProgressState();

  void update_file_progress(lt::file_progress_alert* a);
  void remove(lt::torrent_handle h);

private:
  // torrent handle -> list of file progress which is the number of bytes
  // downloaded of each file in this torrent
  std::unordered_map<lt::torrent_handle, std::vector<std::int64_t>> m_all_file_progress;
};

class PieceInfoState {
public:
  PieceInfoState();
  ~PieceInfoState();

  void update_piece_info(lt::piece_info_alert* a);
  void remove(lt::torrent_handle h);

private:
  // torrent handle -> list of piece info
  std::unordered_map<lt::torrent_handle,
                     std::pair<std::vector<lt::partial_piece_info>, // download_queue
                               std::vector<lt::block_info> // download_queue_block_info
                               >>
      m_all_piece_info;
};

class PieceAvailabilityState {
public:
  PieceAvailabilityState();
  ~PieceAvailabilityState();

  void update_piece_availability(lt::piece_availability_alert* a);
  void remove(lt::torrent_handle h);

private:
  // torrent handle -> list of piece availability
  std::unordered_map<lt::torrent_handle, std::vector<int>> m_all_piece_availability;
};

class TrackerState {
public:
  TrackerState();
  ~TrackerState();

  void update_trackers(lt::tracker_list_alert* a);
  void remove(lt::torrent_handle h);

private:
  // torrent handle -> list of tracker status
  std::unordered_map<lt::torrent_handle, std::vector<lt::announce_entry>> m_all_trackers;
};

} // namespace libtorrent_wrapper

#endif
