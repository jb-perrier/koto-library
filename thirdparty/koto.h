#ifndef KOTO_H
#define KOTO_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>
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

typedef char* Str;
typedef char Bool;
typedef double Number;
typedef uint32_t ValueId;

typedef void* Module;
typedef void* CallContext;

// Module Builder
typedef ValueId (*ModuleBuilder_CreateStrFn)(void* module, const Str value);
typedef ValueId (*ModuleBuilder_CreateNumberFn)(void* module, Number value);
typedef ValueId (*ModuleBuilder_CreateBoolFn)(void* module, Bool value);
typedef ValueId (*ModuleBuilder_CreateMapFn)(void* module);
typedef size_t (*ModuleBuilder_MapInsertFn)(void* module, ValueId map, const Str key, ValueId value);

// CallContext
typedef uint32_t (*CallContext_ArgCountFn)(void* ctx);
typedef uint32_t (*CallContext_ArgTypeFn)(void* ctx, uint32_t index);
typedef const Str (*CallContext_ArgStringFn)(void* ctx, uint32_t index);
typedef Number (*CallContext_ArgNumberFn)(void* ctx, uint32_t index);
typedef void (*CallContext_ReturnStringFn)(void* ctx, const Str value);
typedef void (*CallContext_ReturnNumberFn)(void* ctx, Number value);

typedef struct {
    // Module Builder
    ModuleBuilder_CreateStrFn module_create_str;
    ModuleBuilder_CreateNumberFn module_create_number;
    ModuleBuilder_CreateBoolFn module_create_bool;
    ModuleBuilder_CreateMapFn module_create_map;
    ModuleBuilder_MapInsertFn module_map_insert;

    //CallContext
    // CallContext_ArgCountFn call_ctx_arg_count;
    // CallContext_ArgTypeFn call_ctx_arg_type;

    // CallContext_ArgStringFn call_ctx_arg_string;
    // CallContext_ArgNumberFn call_ctx_arg_number;

    // CallContext_ReturnStringFn call_ctx_return_string;
    // CallContext_ReturnNumberFn call_ctx_return_number;
} KotoInterface;

#ifdef __cplusplus
}
#endif

#endif // KOTO_H