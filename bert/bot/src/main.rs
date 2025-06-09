use loader::{ModuleError, ModuleLoader};

mod loader;

fn main() -> Result<(), ModuleError> {
    let mut loader = ModuleLoader::default();

    // load base module
    // todo: module registry
    // let base = loader.load("target/debug/libbert_base.so")?;
    let base = loader.insert(bert_base::Base);
    println!(
        "{:?} {:?}",
        base.name(),
        base.commands().iter().map(|c| c.name()).collect::<Vec<_>>()
    );

    Ok(())
}
