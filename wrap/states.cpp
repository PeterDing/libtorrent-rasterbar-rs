#include "states.hpp"
#include "../libtorrent/include/libtorrent/session_stats.hpp"
#include "../libtorrent/include/libtorrent/span.hpp"
#include "../libtorrent/include/libtorrent/time.hpp"
#include "../libtorrent/include/libtorrent/torrent_status.hpp"

#include <cstdint>
#include <vector>

namespace libtorrent_wrapper {

// SessionStats
SessionStats::SessionStats() {
  m_stats_metrics = lt::session_stats_metrics();
  m_cnt[1].resize(m_stats_metrics.size(), 0);
  m_cnt[0].resize(m_stats_metrics.size(), 0);
}

SessionStats::~SessionStats() {}

std::vector<lt::stats_metric> SessionStats::stats_metrics() { return m_stats_metrics; }

void SessionStats::update_counters(lt::session_stats_alert* a) {
  lt::span<std::int64_t const> stats_counters = a->counters();
  lt::clock_type::time_point t = a->timestamp();

  // only update the previous counters if there's been enough
  // time since it was last updated
  if (t - m_timestamp[1] > lt::seconds(2)) {
    m_cnt[1].swap(m_cnt[0]);
    m_timestamp[1] = m_timestamp[0];
  }

  m_cnt[0].assign(stats_counters.begin(), stats_counters.end());
  m_timestamp[0] = t;
}

std::vector<std::int64_t> SessionStats::stats() const { return m_cnt[0]; }

std::vector<std::int64_t> SessionStats::prev_stats() const { return m_cnt[1]; }

std::int64_t SessionStats::value(int idx) const {
  if (idx < 0)
    return 0;
  return m_cnt[0][std::size_t(idx)];
}

std::int64_t SessionStats::prev_value(int idx) const {
  if (idx < 0)
    return 0;
  return m_cnt[1][std::size_t(idx)];
}

// TorrentState
TorrentState::TorrentState() {}
TorrentState::~TorrentState() {}

void TorrentState::update_torrents(lt::state_update_alert* a) {
  std::vector<lt::torrent_status> st = std::move(a->status);
  for (lt::torrent_status& t : st) {
    auto j = m_all_torrents.find(t.handle);
    if (j == m_all_torrents.end()) {
      auto handle = t.handle;
      j = m_all_torrents.emplace(handle, std::move(t)).first;
    } else {
      j->second = std::move(t);
    }
  }
}

void TorrentState::remove(lt::torrent_handle h) {
  auto i = m_all_torrents.find(h);
  if (i == m_all_torrents.end())
    return;
  m_all_torrents.erase(i);
}

// DHTStats
DHTStats::DHTStats() {}
DHTStats::~DHTStats() {}

void DHTStats::update_dht_stats(lt::dht_stats_alert* a) {
  active_requests = std::move(a->active_requests);
  routing_table = std::move(a->routing_table);
  nid = a->nid;
  local_endpoint = std::move(a->local_endpoint);
}

// PeerState
PeerState::PeerState() {}
PeerState::~PeerState() {}

void PeerState::update_peers(lt::peer_info_alert* a) {
  printf("!!!!update_peers\n");
  auto h = a->handle;
  auto peers = a->peer_info;
  auto j = m_all_peers.find(h);
  if (j == m_all_peers.end()) {
    j = m_all_peers.emplace(h, std::move(peers)).first;
  } else {
    j->second = std::move(peers);
  }
}

std::vector<lt::peer_info> PeerState::get_peers(lt::torrent_handle h) {
  auto i = m_all_peers.find(h);
  if (i == m_all_peers.end())
    return std::vector<lt::peer_info>();
  printf("!!!!get_peers\n");
  return i->second;
}

void PeerState::remove(lt::torrent_handle h) {
  auto i = m_all_peers.find(h);
  if (i == m_all_peers.end())
    return;
  m_all_peers.erase(i);
}

// FileProgressState
FileProgressState::FileProgressState() {}
FileProgressState::~FileProgressState() {}

void FileProgressState::update_file_progress(lt::file_progress_alert* a) {
  auto h = a->handle;
  auto file_progress = a->files;
  auto j = m_all_file_progress.find(h);
  if (j == m_all_file_progress.end()) {
    j = m_all_file_progress.emplace(h, std::move(file_progress)).first;
  } else {
    j->second = std::move(file_progress);
  }
}

std::vector<std::int64_t> FileProgressState::get_file_progress(lt::torrent_handle h) {
  auto i = m_all_file_progress.find(h);
  if (i == m_all_file_progress.end())
    return std::vector<std::int64_t>();
  return i->second;
}

void FileProgressState::remove(lt::torrent_handle h) {
  auto i = m_all_file_progress.find(h);
  if (i == m_all_file_progress.end())
    return;
  m_all_file_progress.erase(i);
}

// PieceInfoState
PieceInfoState::PieceInfoState() {}
PieceInfoState::~PieceInfoState() {}

void PieceInfoState::update_piece_info(lt::piece_info_alert* a) {
  auto h = a->handle;
  auto download_queue = a->piece_info;
  auto download_queue_block_info = a->block_data;
  auto j = m_all_piece_info.find(h);
  if (j == m_all_piece_info.end()) {
    j = m_all_piece_info
            .emplace(h, std::make_pair(std::move(download_queue),
                                       std::move(download_queue_block_info)))
            .first;
  } else {
    j->second.first = std::move(download_queue);
    j->second.second = std::move(download_queue_block_info);
  }
}

// std::pair<std::vector<lt::partial_piece_info>, std::vector<lt::block_info>>
// PieceInfoState::get_piece_info(lt::torrent_handle h) {
//   auto i = m_all_piece_info.find(h);
//   if (i == m_all_piece_info.end())
//     return std::make_pair(std::vector<lt::partial_piece_info>(),
//                           std::vector<lt::block_info>());
//   return i->second;
// }

void PieceInfoState::remove(lt::torrent_handle h) {
  auto i = m_all_piece_info.find(h);
  if (i == m_all_piece_info.end())
    return;
  m_all_piece_info.erase(i);
}

// PieceAvailabilityState
PieceAvailabilityState::PieceAvailabilityState() {}
PieceAvailabilityState::~PieceAvailabilityState() {}

void PieceAvailabilityState::update_piece_availability(lt::piece_availability_alert* a) {
  auto h = a->handle;
  auto piece_availability = a->piece_availability;
  auto j = m_all_piece_availability.find(h);
  if (j == m_all_piece_availability.end()) {
    j = m_all_piece_availability.emplace(h, std::move(piece_availability)).first;
  } else {
    j->second = std::move(piece_availability);
  }
}

std::vector<int> PieceAvailabilityState::get_piece_availability(lt::torrent_handle h) {
  auto i = m_all_piece_availability.find(h);
  if (i == m_all_piece_availability.end())
    return std::vector<int>();
  return i->second;
}

void PieceAvailabilityState::remove(lt::torrent_handle h) {
  auto i = m_all_piece_availability.find(h);
  if (i == m_all_piece_availability.end())
    return;
  m_all_piece_availability.erase(i);
}

// TrackerState
TrackerState::TrackerState() {}
TrackerState::~TrackerState() {}

void TrackerState::update_trackers(lt::tracker_list_alert* a) {
  auto h = a->handle;
  auto trackers = a->trackers;
  auto j = m_all_trackers.find(h);
  if (j == m_all_trackers.end()) {
    j = m_all_trackers.emplace(h, std::move(trackers)).first;
  } else {
    j->second = std::move(trackers);
  }
}

std::vector<lt::announce_entry> TrackerState::get_trackers(lt::torrent_handle h) {
  auto i = m_all_trackers.find(h);
  if (i == m_all_trackers.end())
    return std::vector<lt::announce_entry>();
  return i->second;
}

void TrackerState::remove(lt::torrent_handle h) {
  auto i = m_all_trackers.find(h);
  if (i == m_all_trackers.end())
    return;
  m_all_trackers.erase(i);
}

} // namespace libtorrent_wrapper
