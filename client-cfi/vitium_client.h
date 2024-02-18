#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Conj {
  int32_t gotta;
  const char *resp;
} Conj;

struct Conj get(const char *url);

struct Conj post(const char *url, const char *mes);
