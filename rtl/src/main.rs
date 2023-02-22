//mod inverter;
mod scan_engine;
mod tri_engine;
mod vga;

use kaze::*;
use tri_engine::*;
use scan_engine::*;
use vga::*;

fn main() -> std::io::Result<()> {
    let c = Context::new();
    //let inverter = Inverter::new("Inverter", &c);
    let scan_engine = ScanEngine::new("ScanEngine", &c);
    let tri_engine = TriEngine::new("TriEngine", &c);
    let vga = Vga::new("Vga", &c);

    // Generate Verilog code
    verilog::generate(scan_engine.m, std::io::stdout())?;
    verilog::generate(tri_engine.m, std::io::stdout())?;
    verilog::generate(vga.m, std::io::stdout())?;

    Ok(())
}
