# Wrapper Crate

This crate provides a Rust-friendly wrapper around the existing C interface for loading Koto dynamic libraries.

## Features

- **Library**: Load C-based dynamic libraries with ergonomic API
- **LibraryBuilder**: Create libraries in pure Rust without C interface
- Type-safe value accessors (`get_string`, `get_number`, `get_bool`)
- Automatic memory management and library lifetime handling
- Builder pattern for fluent API design

## Usage

```rust
use wrapper::{Library, LibraryBuilder};

// Load C library
let lib = Library::load("./target/debug/mylibrary")?;
let greeting = lib.get_string("greeting").unwrap();

// Create pure Rust library  
let lib = LibraryBuilder::new()
    .add_string("name", "My Library")
    .add_function("double", |ctx| { /* ... */ })
    .build();
```

## Benefits over C interface

- Type safety: No ValueId indirection
- Memory safety: Automatic lifetime management  
- Ergonomics: Builder patterns and typed accessors
- Pure Rust option: No FFI overhead for Rust-only libraries