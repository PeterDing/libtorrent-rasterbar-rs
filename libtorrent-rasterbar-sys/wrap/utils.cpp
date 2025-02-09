#include "utils.hpp"
#include "rust/cxx.h"

#include "../libtorrent/include/libtorrent/address.hpp"
#include "../libtorrent/include/libtorrent/socket.hpp"

#include <boost/filesystem.hpp>
#include <fstream>
#include <ios>
#include <sstream>
#include <string>
#include <vector>

namespace libtorrent_wrapper {

bool load_file(std::string const& filename, std::vector<char>& v, int limit) {
  std::fstream f(filename, std::ios_base::in | std::ios_base::binary);
  f.seekg(0, std::ios_base::end);
  auto const s = f.tellg();
  if (s > limit || s < 0)
    return false;
  f.seekg(0, std::ios_base::beg);
  v.resize(static_cast<std::size_t>(s));
  if (s == std::fstream::pos_type(0))
    return !f.fail();
  f.read(v.data(), int(v.size()));
  return !f.fail();
}

bool save_file(std::string const& filename, std::vector<char> const& v) {
  std::fstream f(filename,
                 std::ios_base::trunc | std::ios_base::out | std::ios_base::binary);
  f.write(v.data(), int(v.size()));
  return !f.fail();
}

std::string rust_str_to_string(rust::Str s) { return std::string(s.data(), s.length()); }

std::string to_hex(lt::sha1_hash const& s) {
  std::stringstream ret;
  ret << s;
  return ret.str();
}

std::string to_hex(lt::sha256_hash const& s) {
  std::stringstream ret;
  ret << s;
  return ret.str();
}

lt::sha1_hash from_hex(std::string const& hex) {
  if (hex.length() != 40) {
    throw std::invalid_argument("Invalid SHA1 hash length");
  }

  std::array<char, 20> bytes;
  for (size_t i = 0; i < 20; i++) {
    std::string byte = hex.substr(i * 2, 2);
    bytes[i] = static_cast<char>(std::stoi(byte, nullptr, 16));
  }

  return lt::sha1_hash(bytes);
}

std::string endpoint_to_string(lt::tcp::endpoint const& ep) {
  char buf[200];
  lt::address const& addr = ep.address();
  if (addr.is_v6())
    std::snprintf(buf, sizeof(buf), "[%s]:%d", addr.to_string().c_str(), ep.port());
  else
    std::snprintf(buf, sizeof(buf), "%s:%d", addr.to_string().c_str(), ep.port());
  return buf;
}

std::vector<std::string> list_dir(const std::string& dir, bool recursive) {
  std::vector<std::string> files;

  namespace fs = boost::filesystem;

  try {
    // Check if directory exists
    if (!fs::exists(dir)) {
      return files;
    }

    // Check if path is a directory
    if (!fs::is_directory(dir)) {
      return files;
    }

    // Iterate through the directory
    if (recursive) {
      // Recursive directory iterator (includes subdirectories)
      for (const auto& entry : fs::recursive_directory_iterator(dir)) {
        if (fs::is_regular_file(entry.path())) {
          files.push_back(entry.path().string());
        }
      }
    } else {
      // Regular directory iterator (only current directory)
      for (const auto& entry : fs::directory_iterator(dir)) {
        if (fs::is_regular_file(entry.path())) {
          files.push_back(entry.path().string());
        }
      }
    }
  } catch (const std::exception& e) {
  }

  return files;
}

} // namespace libtorrent_wrapper
