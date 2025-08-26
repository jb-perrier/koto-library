# koto-library

POC for loading dynamic library in the Koto runtime using a C interface.

Require **Clang**.

## Usage

1. Run the compile script:
	```
	./compile.ps1
	```
	or
	```
	./compile.sh
	```
2. Execute with test file:
	```
	cargo run --bin host -- test.koto
	```

### Limitations
- For practical reasons we just `std::mem::forget` the loaded dynamic library, outside a POC we should store it somewhere.
- Unsafe grounds ... especially aliasing rules, needs to be reviewed carefully.