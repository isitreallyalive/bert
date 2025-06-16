#[cfg(feature = "tui")]
use bert_tui::TuiError;
use loader::{ModuleError, ModuleLoader};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[macro_use]
extern crate tracing;

mod loader;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Module(#[from] ModuleError),
    #[cfg(feature = "tui")]
    #[error(transparent)]
    Tui(#[from] TuiError),
}

fn main() -> Result<(), Error> {
    {
        let registry = tracing_subscriber::registry();
        #[cfg(feature = "tui")]
        let registry = registry.with(bert_tui::TuiTracingSubscriberLayer);
        registry.init();
        #[cfg(feature = "tui")]
        bert_tui::init_logger(bert_tui::LevelFilter::Trace).map_err(TuiError::from)?;
    }

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

    #[cfg(feature = "tui")]
    bert_tui::run()?;

    Ok(())
}
