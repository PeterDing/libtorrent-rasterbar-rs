#ifndef LIBTORRENT_WRAPPER_UTILS_HPP_
#define LIBTORRENT_WRAPPER_UTILS_HPP_

#include "../libtorrent/include/libtorrent/sha1_hash.hpp"
#include "../libtorrent/include/libtorrent/socket.hpp"

#include "rust/cxx.h"
#include <string>
#include <vector>

namespace libtorrent_wrapper {

// load a file into a vector
bool load_file(std::string const& filename, std::vector<char>& v, int limit = 8000000);

// save buf to a file
bool save_file(std::string const& filename, std::vector<char> const& v);

// convert rust::Str to std::string
std::string rust_str_to_string(rust::Str s);

// convert lt::sha1_hash to hex
std::string to_hex(lt::sha1_hash const& s);

// convert lt::sha256_hash to hex
std::string to_hex(lt::sha256_hash const& s);

// convert hex to lt::sha1_hash
lt::sha1_hash from_hex(std::string const& hex);

// convert lt::tcp::endpoint to string as
// ipv4:port or [ipv6]:port
std::string endpoint_to_string(lt::tcp::endpoint const& ep);

// list files in a directory
std::vector<std::string> list_dir(const std::string& dir, bool recursive = false);

} // namespace libtorrent_wrapper
#endif
