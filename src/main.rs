use std::env;

use anyhow::{bail, Result};
use webscript_core::Environment;

mod engine;
mod runtime;

fn main() -> Result<()> {
    // Enable better panic.
    better_panic::install();

    // Get the path to the file.
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => bail!("Usage: webscript <path>"),
    };

    let env = Environment::new();
    let mut engine = engine::Engine::create(env)?;

    engine.load_main(path)?;
    Ok(())
}
