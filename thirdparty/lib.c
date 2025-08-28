#include "koto.h"

CallResult add(const KotoInterface *koto, CallContext *ctx, Values *values) {
  if (koto->call.arg_count(ctx) != 2) {
    return CALL_FAILURE;
  }

  if (koto->call.arg_type(ctx, 0) != KOTO_TYPE_NUMBER ||
      koto->call.arg_type(ctx, 1) != KOTO_TYPE_NUMBER) {
    return CALL_FAILURE;
  }

  Number a = koto->call.arg_number(ctx, 0);
  Number b = koto->call.arg_number(ctx, 1);
  Number result = a + b;
  ValueId return_value = koto->values.create_number(values, result);
  return CALL_SUCCESS(return_value);
}

KOTO_API CallResult koto_load(const KotoInterface *koto, Values *values) {
  ValueId map = koto->values.create_map(values);
  ValueId str_value = koto->values.create_str(values, "Hello from Koto!");
  ValueId num_value = koto->values.create_number(values, 42.5);
  koto->values.map_insert(values, map, "greeting", str_value);
  koto->values.map_insert(values, map, "answer", num_value);

  ValueId add_func = koto->values.create_native_function(values, add);
  koto->values.map_insert(values, map, "add", add_func);

  ValueId inner_map = koto->values.create_map(values);
  ValueId inner_str = koto->values.create_str(values, "Inner value");
  koto->values.map_insert(values, inner_map, "inner_key", inner_str);
  ValueId inner_bool = koto->values.create_bool(values, 1);
  koto->values.map_insert(values, inner_map, "inner_bool", inner_bool);
  koto->values.map_insert(values, map, "inner_map", inner_map);
  return CALL_SUCCESS(map);
}
