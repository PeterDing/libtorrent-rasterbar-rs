#include "wrapper.hpp"

#include "../libtorrent/include/libtorrent/add_torrent_params.hpp"
#include "../libtorrent/include/libtorrent/alert.hpp"
#include "../libtorrent/include/libtorrent/alert_types.hpp"
#include "../libtorrent/include/libtorrent/announce_entry.hpp"
#include "../libtorrent/include/libtorrent/aux_/path.hpp"
#include "../libtorrent/include/libtorrent/download_priority.hpp"
#include "../libtorrent/include/libtorrent/error_code.hpp"
#include "../libtorrent/include/libtorrent/load_torrent.hpp"
#include "../libtorrent/include/libtorrent/magnet_uri.hpp"
#include "../libtorrent/include/libtorrent/read_resume_data.hpp"
#include "../libtorrent/include/libtorrent/session_types.hpp"
#include "../libtorrent/include/libtorrent/string_view.hpp"
#include "../libtorrent/include/libtorrent/time.hpp"
#include "../libtorrent/include/libtorrent/torrent_flags.hpp"
#include "../libtorrent/include/libtorrent/torrent_handle.hpp"
#include "../libtorrent/include/libtorrent/write_resume_data.hpp"

#include "libtorrent-rasterbar-sys/src/lib.rs.h"
#include "states.hpp"
#include "units.hpp"
#include "utils.hpp"

#include <boost/algorithm/string.hpp>
#include <boost/algorithm/string/split.hpp>
#include <cstdio>
#include <memory>
#include <string>
#include <system_error>
#include <thread>
#include <utility>
#include <vector>

namespace libtorrent_wrapper {

TorrentStatus cast_torrent_status(lt::torrent_status const& ts) {
  TorrentStatus ret;

  ret.errc = ts.errc.message();
  ret.error_file = static_cast<uint8_t>(ts.error_file);
  ret.save_path = ts.save_path;
  ret.name = ts.name;
  ret.next_announce = ts.next_announce.count();
  ret.current_tracker = ts.current_tracker;
  ret.total_download = ts.total_download;
  ret.total_upload = ts.total_upload;
  ret.total_payload_download = ts.total_payload_download;
  ret.total_payload_upload = ts.total_payload_upload;
  ret.total_failed_bytes = ts.total_failed_bytes;
  ret.total_redundant_bytes = ts.total_redundant_bytes;
  for (auto i : ts.pieces.range()) {
    bool have = ts.pieces[i];
    ret.pieces.push_back(have);
  }
  for (auto i : ts.verified_pieces.range()) {
    bool have = ts.pieces[i];
    ret.verified_pieces.push_back(have);
  }
  ret.total_done = ts.total_done;
  ret.total = ts.total;
  ret.total_wanted_done = ts.total_wanted_done;
  ret.total_wanted = ts.total_wanted;
  ret.all_time_upload = ts.all_time_upload;
  ret.all_time_download = ts.all_time_download;
  ret.added_time = static_cast<std::int64_t>(ts.added_time);
  ret.completed_time = static_cast<std::int64_t>(ts.completed_time);
  ret.last_seen_complete = static_cast<std::int64_t>(ts.last_seen_complete);
  ret.storage_mode = ts.storage_mode;
  ret.progress = ts.progress;
  ret.progress_ppm = ts.progress_ppm;
  ret.queue_position = static_cast<std::int32_t>(ts.queue_position);
  ret.download_rate = ts.download_rate;
  ret.upload_rate = ts.upload_rate;
  ret.download_payload_rate = ts.download_payload_rate;
  ret.upload_payload_rate = ts.upload_payload_rate;
  ret.num_seeds = ts.num_seeds;
  ret.num_peers = ts.num_peers;
  ret.num_complete = ts.num_complete;
  ret.num_incomplete = ts.num_incomplete;
  ret.list_seeds = ts.list_seeds;
  ret.list_peers = ts.list_peers;
  ret.connect_candidates = ts.connect_candidates;
  ret.num_pieces = ts.num_pieces;
  ret.distributed_full_copies = ts.distributed_full_copies;
  ret.distributed_fraction = ts.distributed_fraction;
  ret.distributed_copies = ts.distributed_copies;
  ret.block_size = ts.block_size;
  ret.num_uploads = ts.num_uploads;
  ret.num_connections = ts.num_connections;
  ret.uploads_limit = ts.uploads_limit;
  ret.connections_limit = ts.connections_limit;
  ret.up_bandwidth_queue = ts.up_bandwidth_queue;
  ret.down_bandwidth_queue = ts.down_bandwidth_queue;
  ret.seed_rank = ts.seed_rank;
  ret.state = static_cast<std::uint8_t>(ts.state);
  ret.need_save_resume = ts.need_save_resume;
  ret.is_seeding = ts.is_seeding;
  ret.is_finished = ts.is_finished;
  ret.has_metadata = ts.has_metadata;
  ret.has_incoming = ts.has_incoming;
  ret.moving_storage = ts.moving_storage;
  ret.announcing_to_trackers = ts.announcing_to_trackers;
  ret.announcing_to_lsd = ts.announcing_to_lsd;
  ret.announcing_to_dht = ts.announcing_to_dht;
  ret.info_hash = to_hex(ts.info_hashes.get_best());
  ret.last_upload = ts.last_upload.time_since_epoch().count() / 1000000; // milliseconds
  ret.last_download =
      ts.last_download.time_since_epoch().count() / 1000000; // milliseconds
  ret.active_duration = ts.active_duration.count();
  ret.finished_duration = ts.finished_duration.count();
  ret.seeding_duration = ts.seeding_duration.count();
  ret.flags = ts.flags;

  return ret;
}

TorrentInfo cast_torrent_info(const lt::torrent_info& lt_ti) {
  TorrentInfo ti;

  // fill files
  lt::file_storage fs = lt_ti.files();
  for (auto file_index : fs.file_range()) {
    FileEntry fe;
    fe.file_path = rust::String::lossy(fs.file_path(file_index).data());
    fe.file_name = rust::String::lossy(fs.file_name(file_index).to_string());
    fe.file_size = static_cast<std::uint64_t>(fs.file_size(file_index));
    ti.files.push_back(fe);
  }

  for (auto& t : lt_ti.trackers()) {
    ti.trackers.push_back(rust::String::lossy(t.url));
  }

  // These two functions are related to `BEP 38`_ (mutable torrents). The
  // vectors returned from these correspond to the "similar" and
  // "collections" keys in the .torrent file. Both info-hashes and
  // collections from within the info-dict and from outside of it are
  // included.
  for (auto& sh : lt_ti.similar_torrents()) {
    ti.similar_torrents.push_back(to_hex(sh));
  }
  // fill collections
  for (auto& c : lt_ti.collections()) {
    ti.collections.push_back(rust::String::lossy(c));
  }

  // fill web_seeds
  for (auto& ws : lt_ti.web_seeds()) {
    ti.web_seeds.push_back(rust::String::lossy(ws.url));
  }
  // fill nodes
  // TODO: diff ipv4 and ipv6
  for (auto& n : lt_ti.nodes()) {
    ti.nodes.push_back(rust::String::lossy(n.first + ":" + std::to_string(n.second)));
  }
  // fill total size
  ti.total_size = lt_ti.total_size();
  // fill piece length
  ti.piece_length = lt_ti.piece_length();
  // fill number of pieces
  ti.num_pieces = lt_ti.num_pieces();
  // fill blocks per piece
  ti.blocks_per_piece = lt_ti.blocks_per_piece();
  // fill info-hash
  // TODO: use lt_ti.info_hashs()
  ti.info_hash = rust::String::lossy(to_hex(lt_ti.info_hash()));
  // file num_files
  ti.num_files = static_cast<std::uint32_t>(lt_ti.num_files());
  // fill name
  ti.name = rust::String::lossy(lt_ti.name());
  // fill creation date
  ti.creation_date = static_cast<int64_t>(lt_ti.creation_date());
  // fill creator
  ti.creator = rust::String::lossy(lt_ti.creator());
  // fill ssl cert
  ti.ssl_cert = rust::String::lossy(lt_ti.ssl_cert().to_string());
  // fill is private
  ti.is_private = lt_ti.priv();
  // fill is i2p
  ti.is_i2p = lt_ti.is_i2p();

  return ti;
}

Session::Session(lt::session_params params, std::string session_state_path,
                 std::string resume_dir, std::string torrent_dir, std::uint32_t log_size)
    : m_session_state_path(session_state_path), m_resume_dir(resume_dir),
      m_torrent_dir(torrent_dir), m_log_size(log_size) {
  lt_session = std::make_shared<lt::session>(lt::session(std::move(params)));
  m_running = true;

  m_thread = std::make_shared<std::thread>([=] { poll_alerts(); });

  // load all resume data
  load_all_resume_data();
}

Session::~Session() {
  m_running = false;

  printf("Session destructor 1\n");

  if (m_thread) {
    m_thread->join();
  }

  printf("Session destructor 2\n");

  save_all_resume();

  printf("Session destructor 3\n");

  lt_session->abort(); // asynchronous deconstruction
  lt_session.reset();

  printf("Session destructor 4\n");
}

void Session::save_all_resume() const {
  int outstanding_resume_data = 0; // counter of outstanding resume data
  printf("outstanding_resume_data 1: %d\n", outstanding_resume_data);
  std::vector<lt::torrent_handle> handles = lt_session->get_torrents();
  for (lt::torrent_handle const& h : handles)
    try {
      h.save_resume_data(lt::torrent_handle::only_if_modified);
      ++outstanding_resume_data;
    } catch (lt::system_error const& e) {
      // the handle was invalid, ignore this one and move to the next
    }
  printf("outstanding_resume_data 2: %d\n", outstanding_resume_data);

  while (outstanding_resume_data > 0) {
    printf("outstanding_resume_data 3: %d\n", outstanding_resume_data);
    lt::alert const* a = lt_session->wait_for_alert(lt::seconds(30));

    // if we don't get an alert within 30 seconds, abort
    if (a == nullptr)
      break;

    std::vector<lt::alert*> alerts;
    lt_session->pop_alerts(&alerts);

    for (lt::alert* i : alerts) {
      if (lt::alert_cast<lt::save_resume_data_failed_alert>(i)) {
        std::fprintf(stderr, "fails to save resume data");
        --outstanding_resume_data;
        continue;
      }

      lt::save_resume_data_alert const* p = lt::alert_cast<lt::save_resume_data_alert>(i);
      if (p == nullptr) {
        continue;
      }

      auto const buf = lt::write_resume_data_buf(p->params);
      auto resume_file = get_resume_file_path(p->params.info_hashes.get_best());

      bool ok = save_file(resume_file, buf);

      if (!ok) {
        std::fprintf(stderr, "failed to save resume data: %s\n", resume_file.c_str());
      } else {
        std::fprintf(stderr, "saved resume data: %s\n", resume_file.c_str());
      }

      --outstanding_resume_data;
    }
  }
}

void assign_session_setting(lt::settings_pack& settings, std::string const& key,
                            std::string const& value) {

  printf("set %s to %s\n", key.data(), value.data());

  int const sett_name = lt::setting_by_name(key);
  if (sett_name < 0) {
    throw std::runtime_error("unknown setting: \"" + key + "\"");
  }

  using lt::settings_pack;
  using namespace lt::literals;

  switch (sett_name & settings_pack::type_mask) {
  case settings_pack::string_type_base:
    settings.set_str(sett_name, value);
    break;
  case settings_pack::bool_type_base:
    if (value == "1"_sv || value == "on"_sv || value == "true"_sv) {
      settings.set_bool(sett_name, true);
    } else if (value == "0"_sv || value == "off"_sv || value == "false"_sv) {
      settings.set_bool(sett_name, false);
    } else {
      throw std::runtime_error("invalid value for \"" + key + "\". expected 0 or 1");
    }
    break;
  case settings_pack::int_type_base:
    static std::map<lt::string_view, int> const enums = {
        {"no_piece_suggestions"_sv, settings_pack::no_piece_suggestions},
        {"suggest_read_cache"_sv, settings_pack::suggest_read_cache},
        {"fixed_slots_choker"_sv, settings_pack::fixed_slots_choker},
        {"rate_based_choker"_sv, settings_pack::rate_based_choker},
        {"round_robin"_sv, settings_pack::round_robin},
        {"fastest_upload"_sv, settings_pack::fastest_upload},
        {"anti_leech"_sv, settings_pack::anti_leech},
        {"enable_os_cache"_sv, settings_pack::enable_os_cache},
        {"disable_os_cache"_sv, settings_pack::disable_os_cache},
        {"write_through"_sv, settings_pack::write_through},
        {"prefer_tcp"_sv, settings_pack::prefer_tcp},
        {"peer_proportional"_sv, settings_pack::peer_proportional},
        {"pe_forced"_sv, settings_pack::pe_forced},
        {"pe_enabled"_sv, settings_pack::pe_enabled},
        {"pe_disabled"_sv, settings_pack::pe_disabled},
        {"pe_plaintext"_sv, settings_pack::pe_plaintext},
        {"pe_rc4"_sv, settings_pack::pe_rc4},
        {"pe_both"_sv, settings_pack::pe_both},
        {"none"_sv, settings_pack::none},
        {"socks4"_sv, settings_pack::socks4},
        {"socks5"_sv, settings_pack::socks5},
        {"socks5_pw"_sv, settings_pack::socks5_pw},
        {"http"_sv, settings_pack::http},
        {"http_pw"_sv, settings_pack::http_pw},
    };

    {
      auto const it = enums.find(lt::string_view(value));
      if (it != enums.end()) {
        settings.set_int(sett_name, it->second);
        break;
      }
    }

    if (key.c_str() == "alert_mask"_sv) {
      static std::map<lt::string_view, lt::alert_category_t> const alert_categories = {
          {"error"_sv, lt::alert_category::error},
          {"peer"_sv, lt::alert_category::peer},
          {"port_mapping"_sv, lt::alert_category::port_mapping},
          {"storage"_sv, lt::alert_category::storage},
          {"tracker"_sv, lt::alert_category::tracker},
          {"connect"_sv, lt::alert_category::connect},
          {"status"_sv, lt::alert_category::status},
          {"ip_block"_sv, lt::alert_category::ip_block},
          {"performance_warning"_sv, lt::alert_category::performance_warning},
          {"dht"_sv, lt::alert_category::dht},
          {"stats"_sv, lt::alert_category::stats},
          {"session_log"_sv, lt::alert_category::session_log},
          {"torrent_log"_sv, lt::alert_category::torrent_log},
          {"peer_log"_sv, lt::alert_category::peer_log},
          {"incoming_request"_sv, lt::alert_category::incoming_request},
          {"dht_log"_sv, lt::alert_category::dht_log},
          {"dht_operation"_sv, lt::alert_category::dht_operation},
          {"port_mapping_log"_sv, lt::alert_category::port_mapping_log},
          {"picker_log"_sv, lt::alert_category::picker_log},
          {"file_progress"_sv, lt::alert_category::file_progress},
          {"piece_progress"_sv, lt::alert_category::piece_progress},
          {"upload"_sv, lt::alert_category::upload},
          {"block_progress"_sv, lt::alert_category::block_progress},
          {"all"_sv, lt::alert_category::all},
      };

      // TODO: use boost
      std::stringstream flags(value);
      std::string f;
      lt::alert_category_t val;
      while (std::getline(flags, f, ',')) {
        auto const it = alert_categories.find(f);
        printf("alert mask: %s\n  ==> %s\n", f.c_str(), it->first.data());
        if (it == alert_categories.end())
          val |= lt::alert_category_t{unsigned(std::stoi(f))};
        else
          val |= it->second;

        // throw std::invalid_argument if it doesn't parse
        //     "invalid value for \"%s\". expected integer or enum value\n",
        //     key.c_str());
      }
      settings.set_int(sett_name, val);
      break;
    }

    // set number value
    settings.set_int(sett_name, std::stoi(value));
    break;
  }
}

std::unique_ptr<Session> create_session(bool min_memory_usage, bool high_performance_seed,
                                        rust::Slice<const ParamPair> session_param_list,
                                        std::uint32_t save_state_flags,
                                        rust::Str session_state_path,
                                        rust::Str resume_dir, rust::Str torrent_dir,
                                        std::uint32_t log_size) {
  std::string ssp = rust_str_to_string(session_state_path);
  std::string rd = rust_str_to_string(resume_dir);
  std::string td = rust_str_to_string(torrent_dir);

  lt::session_params params;

  lt::save_state_flags_t flags(save_state_flags);
  std::vector<char> in;
  if (load_file(ssp, in)) {
    params = read_session_params(in, flags);
  }

  if (min_memory_usage)
    params.settings = lt::min_memory_usage();
  if (high_performance_seed)
    params.settings = lt::high_performance_seed();

  // make parent directories
  lt::error_code ec;
  if (lt::has_parent_path(ssp)) {
    lt::create_directories(lt::parent_path(ssp), ec);
    if (ec)
      throw std::runtime_error(ec.message());
  }

  ec.clear();
  if (!lt::exists(rd, ec)) {
    ec.clear();
    lt::create_directories(rd, ec);
    if (ec)
      throw std::runtime_error(ec.message());
  }

  ec.clear();
  if (!lt::exists(td, ec)) {
    ec.clear();
    lt::create_directories(td, ec);
    if (ec)
      throw std::runtime_error(ec.message());
  }

  auto& settings = params.settings;
  for (ParamPair const& sp : session_param_list) {
    assign_session_setting(settings, rust_str_to_string(sp.key),
                           rust_str_to_string(sp.value));
  }

  return std::make_unique<Session>(std::move(params), ssp, rd, td, log_size);
}

std::string Session::get_resume_file_path(lt::sha1_hash info_hash) const {
  std::string info_hash_str = to_hex(info_hash);
  std::string resume_file(m_resume_dir);
  lt::append_path(resume_file, info_hash_str + ".resume");
  return resume_file;
}

std::string Session::get_torrent_file_path(lt::sha1_hash info_hash) const {
  std::string info_hash_str = to_hex(info_hash);
  std::string torrent_file(m_torrent_dir);
  lt::append_path(torrent_file, info_hash_str + ".torrent");
  return torrent_file;
}

void Session::load_all_resume_data() const {
  std::vector<std::string> files = list_dir(m_resume_dir);
  for (auto& f : files) {
    if (!(f.size() > 7 && f.substr(f.size() - 7) == ".resume"))
      continue;

    std::vector<char> resume_data;
    if (!load_file(f, resume_data)) {
      continue;
    }

    lt::error_code ec;
    lt::add_torrent_params p = lt::read_resume_data(resume_data, ec);
    if (ec) {
      continue;
    }

    printf("load resume data: %s\n", f.c_str());

    lt_session->async_add_torrent(std::move(p));
  }
}

void assign_torrent_setting(lt::add_torrent_params& atp, std::string const& key,
                            std::string const& value) {
  using namespace lt::literals;

  printf("torrent set %s = %s\n", key.data(), value.data());

  if (key == "trackers"_sv) {
    std::vector<std::string> trackers;
    boost::split(trackers, value, boost::is_any_of(","));
    for (auto& t : trackers) {
      t = boost::trim_copy(t);
      if (t.empty())
        continue;
      auto it = std::find(atp.trackers.begin(), atp.trackers.end(), t);
      if (it != atp.trackers.end())
        continue;
      printf("add trackers: %s\n", t.c_str());
      atp.trackers.emplace_back(t);
    }
  }

  if (key == "dht_nodes"_sv) {
    std::vector<std::pair<std::string, int>> node_pairs;
    std::vector<std::string> nodes;
    boost::split(nodes, value, boost::is_any_of(","));
    for (auto& n : nodes) {
      n = boost::trim_copy(n);
      if (n.empty())
        continue;

      std::vector<std::string> items;
      boost::split(items, n, boost::is_any_of(":"));
      if (items.size() != 2)
        continue;
      std::pair<std::string, int> node = std::make_pair(items[0], atoi(items[1].c_str()));

      auto it = std::find(atp.dht_nodes.begin(), atp.dht_nodes.end(), node);
      if (it != atp.dht_nodes.end())
        continue;
      atp.dht_nodes.emplace_back(std::move(node));
    }
  }

  if (key == "name"_sv) {
    atp.name = value;
  }

  if (key == "save_path"_sv) {
    atp.save_path = lt::canonicalize_path(value);
  }

  if (key == "storage_mode"_sv) {
    if (value == "storage_mode_sparse"_sv) {
      atp.storage_mode = lt::storage_mode_t::storage_mode_sparse;
    } else if (value == "storage_mode_allocate"_sv) {
      atp.storage_mode = lt::storage_mode_t::storage_mode_allocate;
    } else {
      printf("unknown storage mode: %s\n", value.data());
      atp.storage_mode = lt::storage_mode_t::storage_mode_sparse;
    }
  }

  if (key == "flags"_sv) {
    atp.flags = static_cast<lt::torrent_flags_t>(atoi(value.data()));
  }

  if (key == "max_uploads") {
    atp.max_uploads = atoi(value.data());
  }

  if (key == "max_connections") {
    atp.max_connections = atoi(value.data());
  }

  if (key == "upload_limit") {
    atp.upload_limit = atoi(value.data());
  }

  if (key == "download_limit") {
    atp.download_limit = atoi(value.data());
  }
}

void Session::add_torrent_from_parmas(
    lt::add_torrent_params atp, rust::Slice<const ParamPair> torrent_param_list) const {
  using lt::storage_mode_t;

  for (auto& p : torrent_param_list) {
    assign_torrent_setting(atp, rust_str_to_string(p.key), rust_str_to_string(p.value));
  }

  lt_session->async_add_torrent(std::move(atp));
}

// add torrent to session
// - torrent_path: path to torrent file
// - torrent_param_list: list of key-value pairs see: libtorrent/add_torrent_params.hpp
void Session::add_torrent(rust::Str torrent_path,
                          rust::Slice<const ParamPair> torrent_param_list) const {
  std::string tp = rust_str_to_string(torrent_path);
  std::printf("Add %s\n", tp.data());

  lt::add_torrent_params atp = lt::load_torrent_file(tp);
  std::vector<char> resume_data;
  if (load_file(get_resume_file_path(atp.info_hashes.get_best()), resume_data)) {
    lt::error_code ec;
    lt::add_torrent_params rd = lt::read_resume_data(resume_data, ec);
    if (ec)
      // TODO: add to log
      std::printf("  failed to load resume data: %s\n", ec.message().c_str());
    else
      atp = rd;
  }

  add_torrent_from_parmas(atp, torrent_param_list);
}

void Session::add_magnet(rust::Str magnet_uri,
                         rust::Slice<const ParamPair> torrent_param_list) const {
  std::string mu = rust_str_to_string(magnet_uri);
  std::printf("Add %s\n", mu.data());

  lt::error_code ec;
  lt::add_torrent_params atp = lt::parse_magnet_uri(mu, ec);
  if (ec) {
    throw std::system_error(ec);
  }

  ec.clear();
  std::vector<char> resume_data;
  if (load_file(get_resume_file_path(atp.info_hashes.get_best()), resume_data)) {
    lt::add_torrent_params rd = lt::read_resume_data(resume_data, ec);
    if (ec)
      // TODO: add to log
      std::printf("  failed to load resume data: %s\n", ec.message().c_str());
    else
      atp = rd;
  }

  add_torrent_from_parmas(atp, torrent_param_list);
}

std::unique_ptr<TorrentHandle>
Session::get_torrent_handle(rust::Str info_hash_str) const {
  lt::torrent_handle h = find_torrent_handle(info_hash_str);
  return std::make_unique<TorrentHandle>(h, const_cast<Session*>(this));
}

void Session::remove_torrent(rust::Str info_hash_str, bool delete_files) const {
  lt::torrent_handle h = find_torrent_handle(info_hash_str);

  if (!h.is_valid()) {
    return;
  }

  // remove resume data
  lt::error_code ec;
  auto resume_file = get_resume_file_path(h.info_hashes().get_best());
  if (lt::exists(resume_file, ec)) {
    ec.clear();
    lt::remove(resume_file, ec);
  }

  // remove torrent file
  ec.clear();
  auto torrent_file = get_torrent_file_path(h.info_hashes().get_best());
  if (lt::exists(torrent_file, ec)) {
    ec.clear();
    lt::remove(torrent_file, ec);
  }

  lt_session->remove_torrent(h, delete_files ? lt::session::delete_files
                                             : lt::session::delete_partfile);
}

TwoSessionStats Session::get_stats() const {
  TwoSessionStats tss;

  for (auto& s : m_session_stats.stats()) {
    tss.stats.push_back(s);
  }
  tss.timestamp = m_session_stats.timestamp().time_since_epoch().count() / 1000000;

  for (auto& s : m_session_stats.prev_stats()) {
    tss.prev_stats.push_back(s);
  }
  tss.prev_timestamp =
      m_session_stats.prev_timestamp().time_since_epoch().count() / 1000000;

  return tss;
}

rust::Vec<TorrentInfo> Session::get_torrents() const {
  auto ses = lt_session;
  std::vector<lt::torrent_handle> handles = ses->get_torrents();
  rust::Vec<TorrentInfo> ret;
  for (auto& h : handles) {
    std::shared_ptr<const lt::torrent_info> tf = h.torrent_file();
    ret.push_back(cast_torrent_info(*tf));
  }
  return ret;
}

rust::Vec<TorrentStatus> Session::get_all_torrent_status() const {
  rust::Vec<TorrentStatus> ret;
  ret.reserve(m_torrent_state.m_all_torrents.size());
  for (auto& h : m_torrent_state.m_all_torrents) {
    ret.push_back(cast_torrent_status(h.second));
  }
  return ret;
}

void Session::pause() const { lt_session->pause(); }
void Session::resume() const { lt_session->resume(); }
bool Session::is_paused() const { return lt_session->is_paused(); }

// Handle an alert
// Note: only called from Session::pop_alerts
bool Session::handle_alert(lt::alert* a) {
  using namespace lt;

  // don't log every peer we try to connect to
  if (alert_cast<lt::peer_connect_alert>(a))
    return true;

  if (session_stats_alert* p = alert_cast<session_stats_alert>(a)) {
    m_session_stats.update_counters(p);
    return true;
  }

  if (state_update_alert* p = alert_cast<state_update_alert>(a)) {
    m_torrent_state.update_torrents(p);
    return true;
  }

  if (dht_stats_alert* p = alert_cast<dht_stats_alert>(a)) {
    m_dht_stats.update_dht_stats(p);
    return true;
  }

  if (auto* p = alert_cast<peer_info_alert>(a)) {
    m_peer_state.update_peers(p);
    return true;
  }

  if (auto* p = alert_cast<file_progress_alert>(a)) {
    m_file_progress_state.update_file_progress(p);
    return true;
  }

  if (auto* p = alert_cast<piece_info_alert>(a)) {
    m_piece_info_state.update_piece_info(p);
    return true;
  }

  if (auto* p = alert_cast<piece_availability_alert>(a)) {
    m_piece_availability_state.update_piece_availability(p);
    return true;
  }

  if (auto* p = alert_cast<tracker_list_alert>(a)) {
    m_tracker_state.update_trackers(p);
    return true;
  }

  if (metadata_received_alert* p = alert_cast<metadata_received_alert>(a)) {
    torrent_handle h = p->handle;
    h.save_resume_data(torrent_handle::save_info_dict);
    return false;
  }

  if (add_torrent_alert* p = alert_cast<add_torrent_alert>(a)) {
    if (p->error) {
      // TODO: handle the error
      std::fprintf(stderr, "failed to add torrent: %s %s\n",
                   p->params.ti ? p->params.ti->name().c_str() : p->params.name.c_str(),
                   p->error.message().c_str());
    } else {
      torrent_handle h = p->handle;
      h.save_resume_data(torrent_handle::save_info_dict |
                         torrent_handle::if_metadata_changed);
    }
    return false;
  }

  if (torrent_finished_alert* p = alert_cast<torrent_finished_alert>(a)) {
    // write resume data for the finished torrent
    // the alert handler for save_resume_data_alert
    // will save it to disk
    torrent_handle h = p->handle;
    h.save_resume_data(torrent_handle::save_info_dict |
                       torrent_handle::if_download_progress);
    return false;
  }

  if (save_resume_data_alert* p = alert_cast<save_resume_data_alert>(a)) {
    auto const buf = lt::write_resume_data_buf(p->params);
    auto resume_file = get_resume_file_path(p->params.info_hashes.get_best());

    bool ok = save_file(resume_file, buf);

    if (!ok) {
      std::fprintf(stderr, "failed to save resume data: %s\n", resume_file.c_str());
      return false;
    }

    // save torrent file
    lt::error_code ec;
    auto torrent_file = get_torrent_file_path(p->params.info_hashes.get_best());
    if (!lt::exists(torrent_file, ec)) {
      try {
        lt::entry e =
            lt::write_torrent_file(p->params, lt::write_flags::allow_missing_piece_layer);

        printf("Saving torrent file: %s\n", torrent_file.c_str());

        std::vector<char> torrent;
        lt::bencode(std::back_inserter(torrent), e);

        ok = save_file(torrent_file, torrent);

        if (!ok) {
          std::fprintf(stderr, "failed to save torrent file 1: %s\n",
                       torrent_file.c_str());
          return false;
        }
      } catch (lt::system_error const& e) {
        std::fprintf(stderr, "failed to save torrent file 2: %s\n", torrent_file.c_str());
        return false;
      }
    }

    return false;
  }

  // TODO: handle the error
  // if (save_resume_data_failed_alert* p =
  // alert_cast<save_resume_data_failed_alert>(a))
  // {
  // 	--num_outstanding_resume_data;
  // 	// don't print the error if it was just that we didn't need to save
  // resume
  // 	// data. Returning true means "handled" and not printed to the log
  // 	return p->error == lt::errors::resume_data_not_modified;
  // }

  if (torrent_paused_alert* p = alert_cast<torrent_paused_alert>(a)) {
    // write resume data for the finished torrent
    // the alert handler for save_resume_data_alert
    // will save it to disk
    torrent_handle h = p->handle;
    h.save_resume_data(torrent_handle::save_info_dict);
    return false;
  }

  if (torrent_removed_alert* p = alert_cast<torrent_removed_alert>(a)) {
    m_torrent_state.remove(p->handle);
    m_peer_state.remove(p->handle);
    m_file_progress_state.remove(p->handle);
    m_piece_info_state.remove(p->handle);
    m_piece_availability_state.remove(p->handle);
    m_tracker_state.remove(p->handle);
    return false;
  }

  return false;
}

void Session::pop_alerts() {
  // add lock
  std::lock_guard<std::mutex> lock(m_pop_alerts_mutex);

  std::vector<lt::alert*> alerts;
  lt_session->pop_alerts(&alerts);
  for (auto a : alerts) {
    if (handle_alert(a)) {
      continue;
    }

    // if we didn't handle the alert, print it to the log
    std::pair<lt::time_point, std::string> event =
        std::make_pair(a->timestamp(), a->message());
    m_events.push_back(event);
    if (m_events.size() >= m_log_size)
      m_events.pop_front();
  }
}

void Session::poll_alerts() {
  auto ses = lt_session;
  while (true) {
    if (!ses->is_valid() || !m_running) {
      printf("Session::poll_alerts exit\n");
      break;
    }

    ses->post_session_stats();
    ses->post_torrent_updates();
    ses->post_dht_stats();

    pop_alerts();

    std::this_thread::sleep_for(std::chrono::milliseconds(500));
  }
}

lt::torrent_handle Session::find_torrent_handle(rust::Str info_hash_str) const {
  lt::sha1_hash info_hash = from_hex(rust_str_to_string(info_hash_str));
  auto ses = lt_session;
  return ses->find_torrent(info_hash);
}

rust::Vec<Log> Session::get_logs() {
  rust::Vec<Log> ret;
  ret.reserve(m_events.size());
  while (!m_events.empty()) {
    auto e = m_events.front();
    m_events.pop_front();
    Log l;
    l.timestamp = e.first.time_since_epoch().count() / 1000000; // milliseconds
    l.message = e.second;
    ret.push_back(l);
  }
  return ret;
}

TorrentHandle::TorrentHandle(lt::torrent_handle h, Session* ses)
    : m_torrent_handle(h), m_session(ses) {}
TorrentHandle::~TorrentHandle() {}

void TorrentHandle::add_tracker(rust::Str tracker_url, std::uint8_t tier) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  lt::announce_entry ae(tracker_url.data());
  ae.tier = tier;
  h.add_tracker(ae);
}

void TorrentHandle::scrape_tracker() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.scrape_tracker();
}

void TorrentHandle::force_recheck() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.force_recheck();
}

void TorrentHandle::force_reannounce() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.force_reannounce();
}

void TorrentHandle::force_dht_announce() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.force_dht_announce();
}

void TorrentHandle::force_lsd_announce() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.force_lsd_announce();
}

void TorrentHandle::clear_error() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.clear_error();
}

void TorrentHandle::set_upload_limit(int limit) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.set_upload_limit(limit);
}

int TorrentHandle::upload_limit() const {
  lt::torrent_handle h = m_torrent_handle;

  return h.upload_limit();
}

void TorrentHandle::set_download_limit(int limit) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.set_download_limit(limit);
}

int TorrentHandle::download_limit() const {
  lt::torrent_handle h = m_torrent_handle;

  return h.download_limit();
}

void TorrentHandle::clear_peers() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.clear_peers();
}

void TorrentHandle::set_max_uploads(int max_uploads) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.set_max_uploads(max_uploads);
}

int TorrentHandle::max_uploads() const {
  lt::torrent_handle h = m_torrent_handle;

  return h.max_uploads();
}

void TorrentHandle::set_max_connections(int max_connections) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.set_max_connections(max_connections);
}

int TorrentHandle::max_connections() const {
  lt::torrent_handle h = m_torrent_handle;

  return h.max_connections();
}

void TorrentHandle::pause(uint8_t flags) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.pause(lt::pause_flags_t(flags));
}

void TorrentHandle::resume() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.resume();
}

std::uint64_t TorrentHandle::flags() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return 0;
  }

  return static_cast<std::uint64_t>(h.flags());
}

void TorrentHandle::set_flags(std::uint64_t flags) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.set_flags(lt::torrent_flags_t(flags));
}

void TorrentHandle::set_flags_with_mask(std::uint64_t flags, std::uint64_t mask) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.set_flags(lt::torrent_flags_t(flags), lt::torrent_flags_t(mask));
}

void TorrentHandle::unset_flags(std::uint64_t flags) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.unset_flags(lt::torrent_flags_t(flags));
}

void TorrentHandle::set_file_priority(std::int32_t index, std::uint8_t priority) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  h.file_priority(static_cast<lt::file_index_t>(index),
                  static_cast<lt::download_priority_t>(priority));
}

std::uint8_t TorrentHandle::get_file_priority(std::int32_t index) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return 0;
  }

  return static_cast<std::uint8_t>(h.file_priority(static_cast<lt::file_index_t>(index)));
}

void TorrentHandle::set_prioritize_files(
    rust::Slice<const std::uint8_t> const files) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return;
  }

  std::vector<lt::download_priority_t> priorities;
  for (auto p : files) {
    priorities.push_back(static_cast<lt::download_priority_t>(p));
  }

  h.prioritize_files(priorities);
}

rust::Vec<std::uint8_t> TorrentHandle::get_file_priorities() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return rust::Vec<std::uint8_t>();
  }

  rust::Vec<std::uint8_t> ret;
  std::vector<lt::download_priority_t> priorities = h.get_file_priorities();
  for (auto p : priorities) {
    ret.push_back(static_cast<std::uint8_t>(p));
  }

  return ret;
}

TorrentInfo TorrentHandle::get_torrent_info() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return TorrentInfo();
  }

  std::shared_ptr<const lt::torrent_info> tf = h.torrent_file();
  return cast_torrent_info(*tf);
}

rust::Vec<PeerInfo> TorrentHandle::get_peers() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return rust::Vec<PeerInfo>();
  }

  h.post_peer_info();
  m_session->pop_alerts();

  rust::Vec<PeerInfo> ret;
  std::vector<lt::peer_info> peers = m_session->m_peer_state.get_peers(h);
  for (auto& peer : peers) {
    PeerInfo pi;
    pi.client = peer.client;

    for (auto i : peer.pieces.range()) {
      bool have = peer.pieces[i];
      pi.pieces.push_back(have);
    }

    pi.total_download = peer.total_download;
    pi.total_upload = peer.total_upload;

    pi.last_request = peer.last_request.count();
    pi.last_active = peer.last_active.count();

    pi.download_queue_time = peer.download_queue_time.count();

    pi.flags = static_cast<uint32_t>(peer.flags);

    pi.source = static_cast<uint8_t>(peer.source);
    pi.up_speed = peer.up_speed;
    pi.down_speed = peer.down_speed;
    pi.payload_up_speed = peer.payload_up_speed;
    pi.payload_down_speed = peer.payload_down_speed;
    pi.pid = to_hex(peer.pid);
    pi.queue_bytes = peer.queue_bytes;
    pi.request_timeout = peer.request_timeout;
    pi.send_buffer_size = peer.send_buffer_size;
    pi.used_send_buffer = peer.used_send_buffer;
    pi.receive_buffer_size = peer.receive_buffer_size;
    pi.used_receive_buffer = peer.used_receive_buffer;
    pi.receive_buffer_watermark = peer.receive_buffer_watermark;
    pi.num_hashfails = peer.num_hashfails;
    pi.download_queue_length = peer.download_queue_length;
    pi.timed_out_requests = peer.timed_out_requests;
    pi.busy_requests = peer.busy_requests;
    pi.requests_in_buffer = peer.requests_in_buffer;
    pi.target_dl_queue_length = peer.target_dl_queue_length;
    pi.upload_queue_length = peer.upload_queue_length;
    pi.failcount = peer.failcount;
    pi.downloading_piece_index = static_cast<int32_t>(peer.downloading_piece_index);
    pi.downloading_block_index = peer.downloading_block_index;
    pi.downloading_progress = peer.downloading_progress;
    pi.downloading_total = peer.downloading_total;
    pi.connection_type = static_cast<uint8_t>(peer.connection_type);
    pi.pending_disk_bytes = peer.pending_disk_bytes;
    pi.pending_disk_read_bytes = peer.pending_disk_read_bytes;
    pi.send_quota = peer.send_quota;
    pi.receive_quota = peer.receive_quota;
    pi.rtt = peer.rtt;
    pi.num_pieces = peer.num_pieces;
    pi.download_rate_peak = peer.download_rate_peak;
    pi.upload_rate_peak = peer.upload_rate_peak;
    pi.progress = peer.progress;
    pi.progress_ppm = peer.progress_ppm;
    pi.ip = peer.ip.address().to_string();
    pi.local_endpoint = peer.local_endpoint.address().to_string();
    pi.read_state = static_cast<uint8_t>(peer.read_state);
    pi.write_state = static_cast<uint8_t>(peer.write_state);
    pi.i2p_destination = to_hex(peer.i2p_destination());

    ret.push_back(std::move(pi));
  }

  return ret;
}

rust::Vec<std::int64_t> TorrentHandle::get_file_progress(bool piece_granularity) const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return rust::Vec<std::int64_t>();
  }

  if (piece_granularity) {
    h.post_file_progress(lt::torrent_handle::piece_granularity);
  } else {
    h.post_file_progress({});
  }
  m_session->pop_alerts();

  std::vector<std::int64_t> progress =
      m_session->m_file_progress_state.get_file_progress(h);
  rust::Vec<std::int64_t> ret;
  ret.reserve(progress.size());
  for (auto& p : progress) {
    ret.push_back(p);
  }
  return ret;
}

PieceInfo TorrentHandle::get_piece_info() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return PieceInfo();
  }

  h.post_download_queue();
  m_session->pop_alerts();

  PieceInfo ret;
  auto pair = m_session->m_piece_info_state.get_piece_info(h);

  for (auto& p : pair.first) {
    PartialPieceInfo ppi;
    ppi.piece_index = static_cast<int32_t>(p.piece_index);
    ppi.blocks_in_piece = p.blocks_in_piece;
    ppi.finished = p.finished;
    ppi.writing = p.writing;
    ppi.requested = p.requested;

    lt::block_info const* bs = p.blocks;
    rust::Vec<BlockInfo> blocks;
    for (auto i = 0; i < p.blocks_in_piece; i++) {
      BlockInfo bi;
      bi.bytes_progress = bs->bytes_progress;
      bi.block_size = bs->block_size;
      bi.state = static_cast<uint8_t>(bs->state);
      bi.num_peers = bs->num_peers;
      blocks.push_back(std::move(bi));
      bs++;
    }
    ppi.blocks = std::move(blocks);

    ret.partial_pieces.push_back(std::move(ppi));
  }

  for (auto& p : pair.second) {
    BlockInfo bi;
    bi.bytes_progress = p.bytes_progress;
    bi.block_size = p.block_size;
    bi.state = static_cast<uint8_t>(p.state);
    bi.num_peers = p.num_peers;
    ret.blocks.push_back(std::move(bi));
  }

  return ret;
}

rust::Vec<std::int32_t> TorrentHandle::get_piece_availability() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return rust::Vec<std::int32_t>();
  }

  h.post_piece_availability();
  m_session->pop_alerts();

  std::vector<std::int32_t> availability =
      m_session->m_piece_availability_state.get_piece_availability(h);
  rust::Vec<std::int32_t> ret;
  ret.reserve(availability.size());
  for (auto& p : availability) {
    ret.push_back(p);
  }
  return ret;
}

rust::Vec<AnnounceEntry> TorrentHandle::get_trackers() const {
  lt::torrent_handle h = m_torrent_handle;

  if (!h.is_valid()) {
    return rust::Vec<AnnounceEntry>();
  }

  h.post_trackers();
  m_session->pop_alerts();

  std::vector<lt::announce_entry> trackers = m_session->m_tracker_state.get_trackers(h);
  rust::Vec<AnnounceEntry> ret;
  ret.reserve(trackers.size());
  for (auto& p : trackers) {
    AnnounceEntry ae;
    ae.url = p.url;
    ae.trackerid = p.trackerid;
    ae.tier = p.tier;
    ae.fail_limit = p.fail_limit;
    ae.source = p.source;
    ae.verified = p.verified;

    rust::Vec<AnnounceEndpoint> endpoints;
    for (auto& ep : p.endpoints) {
      AnnounceEndpoint endpoint;
      endpoint.local_endpoint = endpoint_to_string(ep.local_endpoint);

      rust::Vec<AnnounceInfoHash> info_hashes;
      for (lt::protocol_version const v :
           {lt::protocol_version::V1, lt::protocol_version::V2}) {
        AnnounceInfoHash info_hash;
        auto const& av = ep.info_hashes[v];
        info_hash.message = av.message;
        info_hash.last_error = av.last_error.message();
        info_hash.next_announce =
            av.next_announce.time_since_epoch().count() / 1000000; // milliseconds
        info_hash.min_announce =
            av.min_announce.time_since_epoch().count() / 1000000; // milliseconds

        info_hash.scrape_complete = av.scrape_complete;
        info_hash.scrape_incomplete = av.scrape_incomplete;
        info_hash.scrape_downloaded = av.scrape_downloaded;

        info_hash.fails = av.fails;
        info_hash.updating = av.updating;
        info_hash.start_sent = av.start_sent;
        info_hash.complete_sent = av.complete_sent;
        info_hash.triggered_manually = av.triggered_manually;

        info_hashes.push_back(info_hash);
      }
      endpoint.info_hashes = info_hashes;

      endpoints.push_back(endpoint);
    }
    ae.endpoints = endpoints;

    ret.push_back(ae);
  }

  return ret;
}

TorrentStatus TorrentHandle::get_torrent_status() const {
  lt::torrent_handle h = m_torrent_handle;

  lt::torrent_status ts = m_session->m_torrent_state.get_torrent_status(h);

  return cast_torrent_status(ts);
}

} // namespace libtorrent_wrapper
