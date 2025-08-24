#include "koto.h"

KOTO_API int koto_load(void* module, size_t map) {
    ValueId str_value = koto_create_str(module, "Hello from Koto!");
    ValueId num_value = koto_create_number(module, 42.5);
    koto_map_insert(module, map, "greeting", str_value);
    koto_map_insert(module, map, "answer", num_value);

    return 0;
}