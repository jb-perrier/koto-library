use builder::KotoInterface;
use builder::LoadFunc;
use builder::ModuleBuilder;
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

        let load_func = unsafe {
            lib.get::<LoadFunc>(b"koto_load")
        };
        let Ok(load_func) = load_func else {
            return Err(format!(
                "The library '{path}' does not contain a 'koto_load' function"
            )
            .into());
        };

        let mut builder = ModuleBuilder::new();
        let koto_interface = KotoInterface::new();
        let return_value = unsafe {
            let koto_interface = &koto_interface as *const KotoInterface as *mut KotoInterface;
            let module_builder = &mut builder as *mut ModuleBuilder;
            load_func(koto_interface, module_builder)
        };
        let Some(module_map) = builder.take_value(return_value) else {
            return Err("Failed to retrieve the module map".into());
        };
        Ok(module_map)
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
