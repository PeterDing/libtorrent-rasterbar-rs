#include "utils.hpp"
#include "rust/cxx.h"

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

std::string rust_str_to_string(rust::Str s) { return std::string(s.data(), s.length()); }

std::string to_hex(lt::sha1_hash const& s) {
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

} // namespace libtorrent_wrapper
