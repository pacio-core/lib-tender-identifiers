#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const uint8_t *bytes;
  size_t len;
} RustByteSlice;

RustByteSlice get_string_from_rust(void);

char *hello(const char *to);

void hello_release(char *s);

void utf8_bytes_to_rust(const uint8_t *bytes, size_t len);
