#include "ada-binding.hpp"

extern "C"
{
  bool parse(char* url, size_t len, url_aggregator** ada_url)
  {
    auto cpp_url = std::string_view(url, len);
    ada::result<ada::url_aggregator> result = ada::parse(cpp_url);
    if (result) {
      auto cpp_url_ptr = new ada::url_aggregator(result.value());
      *ada_url = reinterpret_cast<url_aggregator*>(cpp_url_ptr);
      return true;
    } else {
      return false;
    }
  }

  void delete_url(url_aggregator* ada_url)
  {
    auto cpp_url = reinterpret_cast<ada::url_aggregator*>(ada_url);
    delete cpp_url;
  }
}
