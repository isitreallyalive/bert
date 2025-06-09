use loader::{ModuleError, ModuleLoader};

#[macro_use]
extern crate tracing;

mod loader;

fn main() -> Result<(), ModuleError> {
    tracing_subscriber::fmt::init();

    let mut loader: ModuleLoader = ModuleLoader::default();
    #[cfg(feature = "base")]
    loader.insert(bert_base::Base);

    // list modules
    for module in loader.modules() {
        info!(
            "Loaded module '{}' with commands: {:?}",
            module.name(),
            module
                .commands()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>()
        );
    }

    Ok(())
}
