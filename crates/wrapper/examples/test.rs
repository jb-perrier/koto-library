use koto::prelude::*;
use wrapper::{Library, LibraryBuilder};

fn main() {
    println!("=== Koto Rust Wrapper Demo ===\n");

    // Test 1: Load existing C library using wrapper with ergonomic API
    println!("1. Loading C library through Rust wrapper:");
    match Library::load("./target/debug/libthirdparty") {
        Ok(lib) => {
            println!("✓ Library loaded successfully\n");
            
            // Ergonomic typed access
            if let Some(greeting) = lib.get_string("greeting") {
                println!("  Greeting: \"{}\"", greeting);
            }
            if let Some(answer) = lib.get_number("answer") {
                println!("  Answer: {}", answer);
            }
            
            // Access nested map
            if let Some(inner_map) = lib.get_map_value("inner_map") {
                if let KValue::Map(map) = inner_map {
                    if let Some(KValue::Str(inner_key)) = map.get("inner_key") {
                        println!("  Inner key: \"{}\"", inner_key);
                    }
                    if let Some(KValue::Bool(inner_bool)) = map.get("inner_bool") {
                        println!("  Inner bool: {}", inner_bool);
                    }
                }
            }
            
            // Use the loaded library in Koto
            let mut koto = Koto::default();
            koto.prelude().insert("c_lib", lib.into_root_value());
            
            println!("\n  Testing C library function:");
            match koto.compile_and_run("print c_lib.add(10, 20)") {
                Ok(_) => println!("  ✓ C function call successful"),
                Err(e) => eprintln!("  ✗ C function call failed: {}", e),
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to load library: {}", e);
        }
    }

    println!("\n{}", "=".repeat(50));
    println!("2. Creating library in pure Rust (no C interface):");
    
    // Pure Rust approach - much cleaner!
    let rust_lib = LibraryBuilder::new()
        .add_string("name", "Pure Rust Library")
        .add_number("version", 2.0)
        .add_bool("rust_native", true)
        .add_function("multiply", |ctx| {
            match (ctx.args().get(0), ctx.args().get(1)) {
                (Some(KValue::Number(a)), Some(KValue::Number(b))) => {
                    let result = match (a, b) {
                        (KNumber::F64(x), KNumber::F64(y)) => KNumber::F64(x * y),
                        (KNumber::I64(x), KNumber::I64(y)) => KNumber::I64(x * y),
                        (KNumber::F64(x), KNumber::I64(y)) => KNumber::F64(x * (*y as f64)),
                        (KNumber::I64(x), KNumber::F64(y)) => KNumber::F64((*x as f64) * y),
                    };
                    Ok(KValue::Number(result))
                }
                _ => Err("Expected two numbers".into())
            }
        })
        .add_function("fibonacci", |ctx| {
            if let Some(KValue::Number(n)) = ctx.args().first() {
                let n: i64 = n.into();
                if n < 0 { return Err("Fibonacci input must be non-negative".into()); }
                
                let mut a = 0i64;
                let mut b = 1i64;
                for _ in 0..n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                Ok(KValue::Number(KNumber::I64(a)))
            } else {
                Err("Expected a number".into())
            }
        })
        .build();
    
    println!("✓ Pure Rust library created");
    
    // Test the Rust library
    let mut koto = Koto::default();
    koto.prelude().insert("rust_lib", rust_lib);
    
    println!("\n  Testing Rust library functions:");
    
    let tests = [
        ("rust_lib.multiply(6, 7)", "Multiplication result"),
        ("rust_lib.fibonacci(10)", "Fibonacci result"),
        ("rust_lib.name", "Library name"),
        ("rust_lib.rust_native", "Native flag"),
    ];
    
    for (code, desc) in tests {
        print!("  {}: ", desc);
        match koto.compile_and_run(code) {
            Ok(_) => {},
            Err(e) => eprintln!("✗ {} failed: {}", desc, e),
        }
    }
    
    println!("\n=== Comparison Summary ===");
    println!("• C Interface: Requires C knowledge, manual memory management, ValueId indirection");
    println!("• Rust Wrapper: Type-safe, ergonomic API, automatic memory management");  
    println!("• Pure Rust: No FFI overhead, full Rust ecosystem integration");
}