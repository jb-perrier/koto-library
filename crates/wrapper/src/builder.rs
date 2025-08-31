use koto::runtime::{KValue, KMap, KNativeFunction, KotoFunction};
use koto::parser::KString;
use koto::runtime::KNumber;

/// Builder for creating Koto libraries in pure Rust
/// This provides an alternative to the C interface for creating loadable content
pub struct LibraryBuilder {
    root_map: KMap,
}

impl LibraryBuilder {
    /// Create a new library builder
    pub fn new() -> Self {
        Self {
            root_map: KMap::default(),
        }
    }

    /// Add a string value to the library
    pub fn add_string<K, V>(self, key: K, value: V) -> Self 
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut new_self = self;
        new_self.root_map.insert(key.as_ref(), KValue::Str(KString::from(value.as_ref())));
        new_self
    }

    /// Add a number value to the library
    pub fn add_number<K>(self, key: K, value: f64) -> Self 
    where
        K: AsRef<str>,
    {
        let mut new_self = self;
        new_self.root_map.insert(key.as_ref(), KValue::Number(KNumber::F64(value)));
        new_self
    }

    /// Add a boolean value to the library
    pub fn add_bool<K>(self, key: K, value: bool) -> Self 
    where
        K: AsRef<str>,
    {
        let mut new_self = self;
        new_self.root_map.insert(key.as_ref(), KValue::Bool(value));
        new_self
    }

    /// Add a native function to the library
    pub fn add_function<K, F>(self, key: K, func: F) -> Self 
    where
        K: AsRef<str>,
        F: KotoFunction + 'static,
    {
        let mut new_self = self;
        let native_fn = KNativeFunction::new(func);
        new_self.root_map.insert(key.as_ref(), KValue::NativeFunction(native_fn));
        new_self
    }

    /// Add a nested map to the library
    pub fn add_map<K>(self, key: K, map: KMap) -> Self 
    where
        K: AsRef<str>,
    {
        let mut new_self = self;
        new_self.root_map.insert(key.as_ref(), KValue::Map(map));
        new_self
    }

    /// Add any KValue to the library
    pub fn add_value<K>(self, key: K, value: KValue) -> Self 
    where
        K: AsRef<str>,
    {
        let mut new_self = self;
        new_self.root_map.insert(key.as_ref(), value);
        new_self
    }

    /// Build the final KValue that represents the library
    pub fn build(self) -> KValue {
        KValue::Map(self.root_map)
    }
}

impl Default for LibraryBuilder {
    fn default() -> Self {
        Self::new()
    }
}