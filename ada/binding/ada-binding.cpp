#include "ada-binding.hpp"

extern "C"
{
  bool parse(char* url, size_t len, url_aggregator** ada_url) {
    auto cpp_url = std::string_view(url, len);
    auto result = ada::parse(cpp_url);
    if (result) {
      *ada_url = reinterpret_cast<url_aggregator*>(&result.value());
      return true;
    } else {
      return false;
    }
  }
}
