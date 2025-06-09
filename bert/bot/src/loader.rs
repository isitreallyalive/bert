use bert::Module;
use libloading::{Library, Symbol};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, thiserror::Error)]
pub enum ModuleError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Library(#[from] libloading::Error),

    #[error("Module '{0}' not found")]
    ModuleNotFound(String),
    #[error("Module name mismatch: expected '{expected}', got '{actual}")]
    NameMismatch { expected: String, actual: String },
    #[error("Module '{0}' is a static module and can not be reloaded")]
    StaticModule(String),
}

struct LoadedModule {
    module: Box<dyn Module>,

    _lib: Option<Library>,
    path: Option<PathBuf>,
    temp_path: Option<PathBuf>,
}

impl Drop for LoadedModule {
    fn drop(&mut self) {
        // clean up temp path
        if let Some(temp_path) = &self.temp_path {
            let _ = fs::remove_file(temp_path);
        }
    }
}

#[derive(Default)]
pub struct ModuleLoader {
    modules: HashMap<String, LoadedModule>,
}

impl ModuleLoader {
    /// Load a module from a given path.
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<&Box<dyn Module>, ModuleError> {
        let (name, loaded_module) = self.load_module_from_path(path.as_ref())?;
        self.modules.insert(name.clone(), loaded_module);
        Ok(&self.modules[&name].module)
    }

    pub fn insert<M: Module + 'static>(&mut self, module: M) -> &Box<dyn Module> {
        let name = module.name().to_string();
        self.modules.insert(
            name.clone(),
            LoadedModule {
                module: Box::new(module),

                _lib: None,
                path: None,
                temp_path: None,
            },
        );

        &self.modules[&name].module
    }

    /// Reload a module by name.
    pub fn reload(&mut self, name: &str) -> Result<String, ModuleError> {
        // check if module exists first
        let path = self
            .modules
            .get(name)
            .map(|m| m.path.clone())
            .ok_or_else(|| ModuleError::ModuleNotFound(name.to_string()))?
            .ok_or_else(|| ModuleError::StaticModule(name.to_string()))?;

        // keep the original module until we successfully load the new one
        let original = self.modules.remove(name).unwrap();

        // try to load the new module
        match self.load_module_from_path(&path) {
            Ok((loaded_name, loaded_module)) => {
                if loaded_name != name {
                    // name mismatch - restore original and return error
                    self.modules.insert(name.to_string(), original);
                    return Err(ModuleError::NameMismatch {
                        expected: name.to_string(),
                        actual: loaded_name,
                    });
                }

                // success - insert the new module
                self.modules.insert(loaded_name.clone(), loaded_module);
                println!("Successfully reloaded module: {}", loaded_name);
                Ok(loaded_name)
            }
            Err(e) => {
                // loading failed - restore the original module
                self.modules.insert(name.to_string(), original);
                Err(e)
            }
        }
    }

    fn load_module_from_path(&self, path: &Path) -> Result<(String, LoadedModule), ModuleError> {
        // force reload by creating a temporary copy with a unique name
        let temp_path = self.create_temp_copy(path)?;

        // load the constructor
        let lib = unsafe { Library::new(&temp_path)? };
        let create_module: Symbol<unsafe extern "C" fn() -> *mut dyn Module> =
            unsafe { lib.get(b"create_module")? };

        // construct the module
        let module_ptr = unsafe { create_module() };
        let module = unsafe { Box::from_raw(module_ptr) };
        let name = module.name().to_string();

        Ok((
            name,
            LoadedModule {
                module,

                _lib: Some(lib),
                path: Some(path.to_path_buf()),
                temp_path: Some(temp_path),
            },
        ))
    }

    fn create_temp_copy(&self, original_path: &Path) -> Result<PathBuf, ModuleError> {
        let temp_dir = std::env::temp_dir();
        let file_name = original_path.file_name().ok_or_else(|| {
            ModuleError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid file path",
            ))
        })?;

        // create unique temp file name using pid and timestamp
        let temp_name = format!(
            "{}_{}_{}",
            file_name.to_string_lossy(),
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );

        let temp_path = temp_dir.join(temp_name);
        fs::copy(original_path, &temp_path)?;
        Ok(temp_path)
    }
}
