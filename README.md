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

### Limitations
- The **builder** crate is linked statically with the **third party library**, can be an issue if the **host** as not be compiled with the same rust version / flags than the **builder**.
Can be solved by passing a struct of function to **koto_load** so it directly comes from the **host**.
- Implement an interface for accessing **CallingContext** in order to use native function.