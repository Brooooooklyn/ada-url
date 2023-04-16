#ifndef ADA_CAPI_H
#define ADA_CAPI_H

#include "ada.hpp"

typedef struct url_aggregator url_aggregator;

extern "C"
{
  bool parse(char* url, size_t len, url_aggregator** ada_url);
}

#endif // ADA_CAPI_H