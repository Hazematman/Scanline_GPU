//mod inverter;
mod scan_engine;
mod tri_engine;

use kaze::*;
use tri_engine::*;
use scan_engine::*;

fn main() -> std::io::Result<()> {
    let c = Context::new();
    //let inverter = Inverter::new("Inverter", &c);
    let scan_engine = ScanEngine::new("ScanEngine", &c);
    let tri_engine = TriEngine::new("TriEngine", &c);

    // Generate Verilog code
    verilog::generate(scan_engine.m, std::io::stdout())?;
    verilog::generate(tri_engine.m, std::io::stdout())?;

    Ok(())
}
