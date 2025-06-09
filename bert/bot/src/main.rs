use loader::{ModuleError, ModuleLoader};

mod loader;

fn main() -> Result<(), ModuleError> {
    let mut loader: ModuleLoader = ModuleLoader::default();
    #[cfg(feature = "base")]
    loader.insert(bert_base::Base);

    // list modules
    for module in loader.modules() {
        println!(
            "Loaded module '{}' with commands: {:?}",
            module.name(),
            module
                .commands()
                .iter()
                .map(|c| c.name())
                .collect::<Vec<_>>()
        );
    }

    #[cfg(feature = "tui")]
    bert_tui::run()?;

    Ok(())
}
