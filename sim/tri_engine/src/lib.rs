#[cfg(test)]
mod tests {
    mod modules {
        include!(concat!(env!("OUT_DIR"), "/modules.rs"));
    }

    use modules::*;

    #[test]
    fn triangle_test() {
        let mut tri_engine = TriEngine::new();

        tri_engine.reset();
        tri_engine.new_scanline = true;
        tri_engine.done_screen = false;

        tri_engine.prop();
        tri_engine.posedge_clk();
        
        for _y in 0..120 {
            for _x in 0..120 {
                tri_engine.prop();
                //println!("State: {}", tri_engine.state_debug);
                if tri_engine.pixel_write_en {
                    println!("Drawing {} {}", _x, _y);
                } 
                tri_engine.posedge_clk();

                if _x >= 117 {
                    tri_engine.new_scanline = true;
                } else {
                    tri_engine.new_scanline = false;
                }
            }
        }

    }
}
