#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  const uint8_t *start;
  size_t len;
} RustByteSlice;

RustByteSlice keypair_from_phrase(RustByteSlice phrase_utf8);

RustByteSlice pubKey_from_pair_bytes(RustByteSlice keypair);

RustByteSlice sign(RustByteSlice message, RustByteSlice keypair);

bool verify(RustByteSlice message, RustByteSlice pubKey, RustByteSlice sig);
