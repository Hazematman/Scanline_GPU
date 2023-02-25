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

        tri_engine.prop();
        tri_engine.posedge_clk();

        tri_engine.new_scanline = false;

        tri_engine.prop();
        tri_engine.posedge_clk();
        
        for _y in 0..120 {
            for _x in 0..120 {
                tri_engine.prop();
                println!("State({},{}): {}, {}", _x, _y, tri_engine.state_debug, tri_engine.pixel_addr);
                if _y >= 10 {
                    let width = 100 - 2*(_y - 10);
                    let last_pixel = _y + width;
                    if _x >= _y && _x <= last_pixel {
                        println!("width, last pixel {} {}", width, last_pixel);
                        assert_eq!(true, tri_engine.pixel_write_en, "Validate triangle drawing");
                    }
                }
                //println!("State({},{}): {}, {}", _x, _y, tri_engine.state_debug, tri_engine.pixel_addr);
                //if tri_engine.pixel_write_en {
                //    println!("Drawing {} {}", _x, _y);
                //}
                tri_engine.posedge_clk();

                // Set new scanline for the third last pixel on the scanline
                // disabling it on the last pixel
                if _x == 117 {
                    tri_engine.new_scanline = true;
                } else {
                    tri_engine.new_scanline = false;
                }
            }
        }

    }
}
