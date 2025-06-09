use loader::ModuleLoader;

use crate::loader::ModuleError;

mod loader;

fn main() -> Result<(), ModuleError> {
    let mut loader = ModuleLoader::default();

    // load base module
    let name = loader.load("target/debug/libbert_base.so")?;
    println!("{:?}", name);

    Ok(())
}
