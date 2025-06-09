pub mod prelude {
    pub use crate::{Command, Module, create_module};
}

/// A command that can be executed.
pub trait Command {
    /// The command's name.
    fn name(&self) -> &str;
}

/// Module trait that dynamically loaded modules must implement.
pub trait Module {
    /// Instantiate the module.
    fn new() -> Self
    where
        Self: Sized;

    /// The name of the module.
    fn name(&self) -> &str;

    /// Get all commands provided by this module.
    fn commands(&self) -> Vec<Box<dyn Command>>;
}

#[macro_export]
macro_rules! create_module {
    ($module_type:ty) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn create_module() -> *mut dyn $crate::Module {
            let module = <$module_type>::new();
            Box::into_raw(Box::new(module)) as *mut dyn $crate::Module
        }
    };
}
