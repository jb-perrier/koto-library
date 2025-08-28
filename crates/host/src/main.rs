use builder::KotoInterface;
use builder::LoadFunc;
use builder::Values;
use koto::prelude::*;
use libloading::Library;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <koto_file>", args[0]);
        eprintln!("Example: {} script.koto", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    let script_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{file_path}': {e}");
            process::exit(1);
        }
    };

    let mut koto = Koto::default();
    let prelude = koto.prelude();
    prelude.add_fn("load_library", |ctx| {
        let Some(KValue::Str(path)) = ctx.args().first() else {
            return Err("Expected a file path as the first argument".into());
        };

        let mut lib_path = std::path::PathBuf::from(path.as_str());

        #[cfg(target_os = "windows")]
        lib_path.set_extension("dll");
        #[cfg(not(target_os = "windows"))]
        lib_path.set_extension("so");

        let path = lib_path.to_string_lossy().to_string();
        let lib = unsafe {
            match Library::new(path.as_str()) {
                Ok(lib) => lib,
                Err(e) => return Err(format!("Failed to load library '{path}': {e}").into()),
            }
        };

        let load_func = unsafe { lib.get::<LoadFunc>(b"koto_load") };

        let Ok(load_func) = load_func else {
            return Err(
                format!("The library '{path}' does not contain a 'koto_load' function").into(),
            );
        };

        let mut values = Values::new();
        let koto_interface = KotoInterface::new();
        let result = unsafe {
            let koto_interface = &koto_interface as *const KotoInterface as *mut KotoInterface;
            let values = &mut values as *mut Values;
            load_func(koto_interface, values)
        };

        if result.code == builder::FAILURE {
            return Err("The library failed to load".into());
        }

        let Some(value) = values.take_value(result.value) else {
            return Err("Failed to retrieve the return value".into());
        };

        // DO NOT REMOVE, otherwise the library will be dropped and calling function pointers from this lib will crash
        std::mem::forget(lib);
        Ok(value)
    });

    match koto.compile_and_run(&script_content) {
        Ok(result) => {
            if let Err(e) = koto.value_to_string(result.clone()) {
                eprintln!("Error converting result to string: {e}");
                println!("{result:?}");
            }
        }
        Err(e) => {
            eprintln!("Error executing Koto script: {e}");
            process::exit(1);
        }
    }
}
