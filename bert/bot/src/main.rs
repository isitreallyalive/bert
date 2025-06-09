use loader::{ModuleError, ModuleLoader};
use std::io::{self, Write};

mod loader;

fn main() -> Result<(), ModuleError> {
    let mut loader = ModuleLoader::default();

    // load base module
    let name = loader.load("target/debug/libbert_base.so")?;
    println!("{:?}", name);

    println!("Press Enter to continue...");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let name2 = loader.reload(&name)?;
    println!("{:?}", name2);

    Ok(())
}
