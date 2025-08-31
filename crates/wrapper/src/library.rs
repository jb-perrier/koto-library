use crate::{Result, WrapperError};
use builder::{KotoInterface, LoadFunc, Values};
use koto::runtime::KValue;
use libloading::Library as DynLib;
use std::path::PathBuf;

/// A loaded Koto dynamic library with ergonomic Rust interface
pub struct Library {
    lib: DynLib, // Changed from _lib to lib to indicate we need to keep it alive
    root_value: KValue,
}

impl Library {
    /// Load a dynamic library and call its koto_load function
    pub fn load<P: AsRef<str>>(path: P) -> Result<Self> {
        let mut lib_path = PathBuf::from(path.as_ref());
        
        // Add platform-specific extension
        #[cfg(target_os = "windows")]
        lib_path.set_extension("dll");
        #[cfg(not(target_os = "windows"))]
        lib_path.set_extension("so");

        let path_str = lib_path.to_string_lossy().to_string();
        let lib = unsafe {
            DynLib::new(&path_str)
                .map_err(|e| WrapperError::LibraryLoad(format!("Failed to load '{}': {}", path_str, e)))?
        };

        let load_func = unsafe { 
            lib.get::<LoadFunc>(b"koto_load")
                .map_err(|_| WrapperError::LibraryLoad(
                    format!("Library '{}' does not contain a 'koto_load' function", path_str)
                ))?
        };

        let mut values = Values::new();
        let koto_interface = KotoInterface::new();
        
        let result = unsafe {
            let koto_interface_ptr = &koto_interface as *const KotoInterface as *mut KotoInterface;
            let values_ptr = &mut values as *mut Values;
            load_func(koto_interface_ptr, values_ptr)
        };

        if result.code == builder::FAILURE {
            return Err(WrapperError::LibraryLoad("Library load function failed".into()));
        }

        let root_value = values.take_value(result.value)
            .ok_or_else(|| WrapperError::LibraryLoad("Failed to retrieve root value".into()))?;

        Ok(Library {
            lib, // Keep the library alive to prevent segfault on function calls
            root_value,
        })
    }

    /// Get the root value returned by the library's koto_load function
    pub fn root_value(&self) -> &KValue {
        &self.root_value
    }

    /// Take ownership of the root value, but note that the library handle will be dropped
    /// This means any native functions from the library will become invalid!
    pub fn into_root_value(self) -> KValue {
        // Forget the library to prevent crashes from function pointers becoming invalid
        // This is the same approach used in the original host implementation
        std::mem::forget(self.lib);
        self.root_value
    }

    /// Helper method to access map values if root is a map
    pub fn get_map_value(&self, key: &str) -> Option<KValue> {
        match &self.root_value {
            KValue::Map(map) => map.get(key),
            _ => None,
        }
    }

    /// Get a string value from the root map
    pub fn get_string(&self, key: &str) -> Option<String> {
        match self.get_map_value(key)? {
            KValue::Str(s) => Some(s.to_string()),
            _ => None,
        }
    }

    /// Get a number value from the root map
    pub fn get_number(&self, key: &str) -> Option<f64> {
        match self.get_map_value(key)? {
            KValue::Number(n) => Some(n.into()),
            _ => None,
        }
    }

    /// Get a boolean value from the root map
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get_map_value(key)? {
            KValue::Bool(b) => Some(b),
            _ => None,
        }
    }
}