#ifndef ADA_CAPI_H
#define ADA_CAPI_H

#include "ada.hpp"

typedef struct url_aggregator url_aggregator;

struct CppStringView
{
  const char* data;
  size_t length;
};

struct AdaUrlObject
{
  CppStringView href;
  CppStringView origin;
  CppStringView protocol;
  CppStringView username;
  CppStringView password;
  CppStringView hostname;
  CppStringView port;
  CppStringView pathname;
  CppStringView search;
  CppStringView hash;
  url_aggregator* url;
};

extern "C"
{
  bool parse(char* url, size_t len, AdaUrlObject* url_object);
  void delete_url(url_aggregator* ada_url);
}

#endif // ADA_CAPI_H