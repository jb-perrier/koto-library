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

typedef void Values;
typedef void CallContext;

typedef int ResultCode;
#define SUCCESS 1
#define FAILURE 0

typedef struct CallResult {
  ResultCode code;
  ValueId value;
} CallResult;
#define CALL_SUCCESS(var_value) ((CallResult){.code = SUCCESS, .value = var_value})
#define CALL_FAILURE ((CallResult){.code = FAILURE, .value = -1})

enum KotoValueType {
  KOTO_TYPE_NULL = 0,
  KOTO_TYPE_BOOL = 1,
  KOTO_TYPE_NUMBER = 2,
  KOTO_TYPE_STR = 7
};

// Forward declarations
typedef struct ValuesInterface ValuesInterface;
typedef struct CallContextInterface CallContextInterface;
struct _KotoInterface;
typedef struct _KotoInterface KotoInterface;

// Koto Native Function
typedef CallResult (*ForeignNativeFunction)(const KotoInterface *koto,
                                            CallContext *ctx, Values* values);

// Values function types
typedef ValueId (*Values_CreateStrFn)(void *values, const Str value);
typedef ValueId (*Values_CreateNumberFn)(void *values, Number value);
typedef ValueId (*Values_CreateBoolFn)(void *values, Bool value);
typedef ValueId (*Values_CreateMapFn)(void *values);
typedef ValueId (*Values_CreateNativeFunctionFn)(void *values,
                                                 ForeignNativeFunction func);
typedef ResultCode (*Values_MapInsertFn)(void *values, ValueId map,
                                         const Str key, ValueId value);

// CallContext function types
typedef unsigned int (*CallContext_ArgCountFn)(void *ctx);
typedef unsigned int (*CallContext_ArgTypeFn)(void *ctx, unsigned int index);
typedef const Str (*CallContext_ArgStringFn)(void *ctx, unsigned int index);
typedef Number (*CallContext_ArgNumberFn)(void *ctx, unsigned int index);
typedef void (*CallContext_ReturnStringFn)(void *ctx, const Str value);
typedef void (*CallContext_ReturnNumberFn)(void *ctx, Number value);

struct ValuesInterface {
  Values_CreateStrFn create_str;
  Values_CreateNumberFn create_number;
  Values_CreateBoolFn create_bool;
  Values_CreateMapFn create_map;
  Values_CreateNativeFunctionFn create_native_function;
  Values_MapInsertFn map_insert;
};

struct CallContextInterface {
  CallContext_ArgCountFn arg_count;
  CallContext_ArgTypeFn arg_type;
  //   CallContext_ArgStringFn arg_string;
  CallContext_ArgNumberFn arg_number;
  //   CallContext_ReturnStringFn return_string;
  CallContext_ReturnNumberFn return_number;
};

// Main interface with nested valuess
typedef struct _KotoInterface {
  ValuesInterface values;
  CallContextInterface call;
} KotoInterface;

#ifdef __cplusplus
}
#endif

#endif // KOTO_H