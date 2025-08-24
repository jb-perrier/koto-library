#include "koto.h"

KOTO_API ValueId koto_load(KotoInterface koto, Module module) {
    ValueId map = koto.module_create_map(module);
    ValueId str_value = koto.module_create_str(module, "Hello from Koto!");
    ValueId num_value = koto.module_create_number(module, 42.5);
    koto.module_map_insert(module, map, "greeting", str_value);
    koto.module_map_insert(module, map, "answer", num_value);

    ValueId inner_map = koto.module_create_map(module);
    ValueId inner_str = koto.module_create_str(module, "Inner value");
    koto.module_map_insert(module, inner_map, "inner_key", inner_str);
    ValueId inner_bool = koto.module_create_bool(module, 1);
    koto.module_map_insert(module, inner_map, "inner_bool", inner_bool);
    koto.module_map_insert(module, map, "inner_map", inner_map);
    return map;
}