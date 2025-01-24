#ifndef LIBTORRENT_WRAPPER_UTILS_HPP_
#define LIBTORRENT_WRAPPER_UTILS_HPP_

#include "../libtorrent/include/libtorrent/sha1_hash.hpp"

#include "rust/cxx.h"
#include <string>
#include <vector>

namespace libtorrent_wrapper {

// load a file into a vector
bool load_file(std::string const& filename, std::vector<char>& v, int limit = 8000000);

// convert rust::Str to std::string
std::string rust_str_to_string(rust::Str s);

// convert lt::sha1_hash to hex
std::string to_hex(lt::sha1_hash const& s);

// convert hex to lt::sha1_hash
lt::sha1_hash from_hex(std::string const& hex);

} // namespace libtorrent_wrapper
#endif
