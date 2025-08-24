# koto-library

POC for loading dynamic library in the Koto runtime using a C interface.

Require **Clang**

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