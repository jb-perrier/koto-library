#ifndef KOTO_H
#define KOTO_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


#ifdef _WIN32
#define KOTO_API __declspec(dllexport)
#else
#ifdef __GNUC__
#define KOTO_API __attribute__((visibility("default")))
#else
#define KOTO_API
#endif
#endif

typedef double Number;
typedef int ValueId;
typedef char *Str;

typedef char Bool;
#define TRUE 1
#define FALSE 0

typedef void Module;
typedef void CallContext;

typedef int ResultCode;
#define SUCCESS 1
#define FAILURE 0

enum KotoValueType {
  KOTO_TYPE_NULL = 0,
  KOTO_TYPE_BOOL = 1,
  KOTO_TYPE_NUMBER = 2,
  KOTO_TYPE_STR = 7
};

// Forward declarations
typedef struct ModuleBuilderInterface ModuleBuilderInterface;
typedef struct CallContextInterface CallContextInterface;
struct _KotoInterface;
typedef struct _KotoInterface KotoInterface;

// Koto Native Function
typedef ResultCode (*ForeignNativeFunction)(const KotoInterface* koto, CallContext* ctx);

// Module Builder function types
typedef ValueId (*ModuleBuilder_CreateStrFn)(void *module, const Str value);
typedef ValueId (*ModuleBuilder_CreateNumberFn)(void *module, Number value);
typedef ValueId (*ModuleBuilder_CreateBoolFn)(void *module, Bool value);
typedef ValueId (*ModuleBuilder_CreateMapFn)(void *module);
typedef ValueId (*ModuleBuilder_CreateNativeFunctionFn)(void *module,
                                                        ForeignNativeFunction func);
typedef ResultCode (*ModuleBuilder_MapInsertFn)(void *module, ValueId map,
                                            const Str key, ValueId value);

// CallContext function types
typedef unsigned int (*CallContext_ArgCountFn)(void *ctx);
typedef unsigned int (*CallContext_ArgTypeFn)(void *ctx, unsigned int index);
typedef const Str (*CallContext_ArgStringFn)(void *ctx, unsigned int index);
typedef Number (*CallContext_ArgNumberFn)(void *ctx, unsigned int index);
typedef void (*CallContext_ReturnStringFn)(void *ctx, const Str value);
typedef void (*CallContext_ReturnNumberFn)(void *ctx, Number value);

struct ModuleBuilderInterface {
  ModuleBuilder_CreateStrFn create_str;
  ModuleBuilder_CreateNumberFn create_number;
  ModuleBuilder_CreateBoolFn create_bool;
  ModuleBuilder_CreateMapFn create_map;
  ModuleBuilder_CreateNativeFunctionFn create_native_function;
  ModuleBuilder_MapInsertFn map_insert;
};

struct CallContextInterface {
  CallContext_ArgCountFn arg_count;
  CallContext_ArgTypeFn arg_type;
//   CallContext_ArgStringFn arg_string;
  CallContext_ArgNumberFn arg_number;
//   CallContext_ReturnStringFn return_string;
  CallContext_ReturnNumberFn return_number;
};

// Main interface with nested modules
typedef struct _KotoInterface{
  ModuleBuilderInterface module;
  CallContextInterface call;
} KotoInterface;

#ifdef __cplusplus
}
#endif

#endif // KOTO_H