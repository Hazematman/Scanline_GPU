use kaze::*;
use rtl::tri_engine::*;

use std::env;
use std::fs::File;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("modules.rs");
    let file = File::create(&dest_path).unwrap();

    let c = Context::new();

    sim::generate(TriEngine::new("TriEngine", &c).m, sim::GenerationOptions::default(), file)?;

    Ok(())
}
