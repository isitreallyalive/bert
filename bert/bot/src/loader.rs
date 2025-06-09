use std::{collections::HashMap, path::Path};

use bert::Module;
use libloading::{Library, Symbol};

#[derive(Debug, thiserror::Error)]
pub enum ModuleError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Library(#[from] libloading::Error),
}

struct LoadedModule {
    module: Box<dyn Module>,
    _lib: Library,
}

#[derive(Default)]
pub struct ModuleLoader {
    modules: HashMap<String, LoadedModule>,
}

impl ModuleLoader {
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<String, ModuleError> {
        // load the constructor
        let lib = unsafe { Library::new(path.as_ref())? };
        let create_module: Symbol<unsafe extern "C" fn() -> *mut dyn Module> =
            unsafe { lib.get(b"create_module")? };

        // construct the module
        let module_ptr = unsafe { create_module() };
        let module = unsafe { Box::from_raw(module_ptr) };

        // store the mdoule
        let name = module.name().to_string();
        self.modules
            .insert(name.clone(), LoadedModule { module, _lib: lib });

        Ok(name)
    }
}
