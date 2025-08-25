#include "koto.h"
#include <stdio.h>

ResultCode add(const KotoInterface *koto, CallContext *ctx) {
  	if (koto->call.arg_count(ctx) != 2) {
		  return FAILURE;
  	}

  	if (koto->call.arg_type(ctx, 0) != KOTO_TYPE_NUMBER
		|| koto->call.arg_type(ctx, 1) != KOTO_TYPE_NUMBER) {
		  return FAILURE;
  	}

  	Number a = koto->call.arg_number(ctx, 0);
  	Number b = koto->call.arg_number(ctx, 1);
  	Number result = a + b;
  	koto->call.return_number(ctx, result);
  	return SUCCESS;
}

KOTO_API ValueId koto_load(const KotoInterface *koto, Module *module) {
  	ValueId map = koto->module.create_map(module);
  	ValueId str_value = koto->module.create_str(module, "Hello from Koto!");
  	ValueId num_value = koto->module.create_number(module, 42.5);
  	koto->module.map_insert(module, map, "greeting", str_value);
  	koto->module.map_insert(module, map, "answer", num_value);

  	ValueId add_func = koto->module.create_native_function(module, add);
  	koto->module.map_insert(module, map, "add", add_func);

  	ValueId inner_map = koto->module.create_map(module);
  	ValueId inner_str = koto->module.create_str(module, "Inner value");
  	koto->module.map_insert(module, inner_map, "inner_key", inner_str);
  	ValueId inner_bool = koto->module.create_bool(module, 1);
  	koto->module.map_insert(module, inner_map, "inner_bool", inner_bool);
  	koto->module.map_insert(module, map, "inner_map", inner_map);
  	return map;
}
