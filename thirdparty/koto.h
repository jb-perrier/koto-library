#ifndef KOTO_H
#define KOTO_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdbool.h>

#ifdef _WIN32
    #define KOTO_API __declspec(dllexport)
#else
    #ifdef __GNUC__
        #define KOTO_API __attribute__((visibility("default")))
    #else
        #define KOTO_API
    #endif
#endif

typedef size_t ValueId;
typedef void* Module;

KOTO_API ValueId koto_create_str(Module module, const char* value);
KOTO_API ValueId koto_create_number(Module module, double value);
KOTO_API ValueId koto_create_map(Module module);
KOTO_API size_t koto_map_insert(Module module, ValueId map, const char* key, ValueId value);

#ifdef __cplusplus
}
#endif

#endif // KOTO_H