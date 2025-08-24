# koto-library

POC loading dynamic lirbary in the Koto runtime using a C interface.

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