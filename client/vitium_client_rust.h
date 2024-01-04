#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct TestStruct {
  int32_t a;
};

struct TestEnum {
  enum class Tag {
    A,
  };

  struct A_Body {
    TestStruct _0;
  };

  Tag tag;
  union {
    A_Body a;
  };
};

extern "C" {

TestEnum test_fn(int32_t a, int32_t b);

} // extern "C"
