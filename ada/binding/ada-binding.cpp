#include "ada-binding.hpp"

extern "C"
{
  static CppStringView to_cpp_string_view(std::string_view sv)
  {
    return CppStringView{sv.data(), sv.length()};
  }

  bool parse(char* url, size_t len, AdaUrlObject* url_object)
  {
    auto cpp_url = std::string_view(url, len);
    ada::result<ada::url_aggregator> result = ada::parse(cpp_url);
    if (result) {
      auto cpp_url_ptr = new ada::url_aggregator(result.value());
      url_object->href = to_cpp_string_view(cpp_url_ptr->get_href());
      url_object->origin = to_cpp_string_view(cpp_url_ptr->get_origin());
      url_object->protocol = to_cpp_string_view(cpp_url_ptr->get_protocol());
      url_object->username = to_cpp_string_view(cpp_url_ptr->get_username());
      url_object->password = to_cpp_string_view(cpp_url_ptr->get_password());
      url_object->hostname = to_cpp_string_view(cpp_url_ptr->get_hostname());
      url_object->port = to_cpp_string_view(cpp_url_ptr->get_port());
      url_object->pathname = to_cpp_string_view(cpp_url_ptr->get_pathname());
      url_object->search = to_cpp_string_view(cpp_url_ptr->get_search());
      url_object->hash = to_cpp_string_view(cpp_url_ptr->get_hash());
      url_object->url = reinterpret_cast<url_aggregator*>(cpp_url_ptr);
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
