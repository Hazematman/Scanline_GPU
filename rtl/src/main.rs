//mod inverter;
mod scan_engine;
mod tri_engine;
mod vga;
mod video_engine;

use kaze::*;
use video_engine::*;

fn main() -> std::io::Result<()> {
    let c = Context::new();
    let video_engine = VideoEngine::new("VideoEngine", &c);

    // Generate Verilog code
    verilog::generate(video_engine.m, std::io::stdout())?;

    Ok(())
}
