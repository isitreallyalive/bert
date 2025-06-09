pub mod prelude {
    pub use crate::{Command, Module, create_module};
}

pub use pastey::paste;

/// A command that can be executed.
pub trait Command {
    /// The command's name.
    fn name(&self) -> &str;
}

/// Module trait that dynamically loaded modules must implement.
pub trait Module {
    /// The name of the module.
    fn name(&self) -> &str;

    /// Get all commands provided by this module.
    fn commands(&self) -> Vec<Box<dyn Command>>;
}

#[macro_export]
macro_rules! create_module {
    (
        $name:ident;
        $(cmd: $($cmd:expr),*)?
    ) => {
        pub struct $name;

        impl Module for $name {
            fn name(&self) -> &str {
                $crate::paste! { stringify!([<$name:lower>]) }
            }

            fn commands(&self) -> Vec<Box<dyn Command>> {
                vec![
                    $(
                        $(
                            Box::new($cmd)
                        ),+
                    )?
                ]
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn create_module() -> *mut dyn $crate::Module {
            Box::into_raw(Box::new($name {})) as *mut dyn $crate::Module
        }
    };
}
