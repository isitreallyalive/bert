pub mod prelude {
    pub use crate::{Module, create_module};
}

/// Module trait that dynamically loaded modules must implement.
pub trait Module {
    /// Instantiate the module.
    fn new() -> Self
    where
        Self: Sized;

    /// The name of the module.
    fn name(&self) -> &str;
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
