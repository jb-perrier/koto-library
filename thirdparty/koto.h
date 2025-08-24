#ifndef KOTO_H
#define KOTO_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdbool.h>

// Cross-platform function export macros
#ifdef _WIN32
    #define KOTO_API __declspec(dllexport)
#else
    #ifdef __GNUC__
        #define KOTO_API __attribute__((visibility("default")))
    #else
        #define KOTO_API
    #endif
#endif

// Type definitions
typedef size_t ValueId;

// Function declarations
/**
 * Creates a string value in the Koto module
 * @param module Pointer to the ModuleBuilder instance
 * @param value C string to convert to Koto string
 * @return ValueId of the created string, or SIZE_MAX on error
 */
KOTO_API ValueId koto_create_str(void* module, const char* value);

/**
 * Creates a number value in the Koto module
 * @param module Pointer to the ModuleBuilder instance
 * @param value Double value to convert to Koto number
 * @return ValueId of the created number, or SIZE_MAX on error
 */
KOTO_API ValueId koto_create_number(void* module, double value);

/**
 * Creates a map value in the Koto module
 * @param module Pointer to the ModuleBuilder instance
 * @return ValueId of the created map, or SIZE_MAX on error
 */
KOTO_API ValueId koto_create_map(void* module);

/**
 * Inserts a key-value pair into a Koto map
 * @param module Pointer to the ModuleBuilder instance
 * @param map ValueId of the target map
 * @param key C string key
 * @param value ValueId of the value to insert
 * @return true on success, false on error
 */
KOTO_API bool koto_map_insert(void* module, ValueId map, const char* key, ValueId value);

#ifdef __cplusplus
}
#endif

#endif // KOTO_H