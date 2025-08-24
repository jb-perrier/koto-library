#include "koto.h"

KOTO_API int koto_load(void* module, size_t map) {
    ValueId str_value = koto_create_str(module, "Hello from Koto!");
    ValueId num_value = koto_create_number(module, 42.5);
    koto_map_insert(module, map, "greeting", str_value);
    koto_map_insert(module, map, "answer", num_value);

    ValueId inner_map = koto_create_map(module);
    ValueId inner_str = koto_create_str(module, "Inner value");
    koto_map_insert(module, inner_map, "inner_key", inner_str);
    koto_map_insert(module, map, "inner_map", inner_map);
    return 0;
}