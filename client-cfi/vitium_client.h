#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Conj {
  uint16_t gotta;
  const char *resp;
};

extern "C" {

Conj get(const char *url);

Conj post(const char *url, const char *mes);

} // extern "C"
