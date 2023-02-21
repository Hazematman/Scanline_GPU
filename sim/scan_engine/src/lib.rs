#[cfg(test)]
mod tests {
    mod modules {
        include!(concat!(env!("OUT_DIR"), "/modules.rs"));
    }

    use modules::*;

    #[test]
    fn basic_raster_test() {
        let mut scan_engine = ScanEngine::new();

        // Setup edge values with values that cause a pixel to always be drawn
        scan_engine.e0 = 1u32;
        scan_engine.e1 = 1u32;
        scan_engine.e2 = 1u32;
        scan_engine.dy0 = 1u32;
        scan_engine.dy1 = 1u32;
        scan_engine.dy2 = 1u32;
        scan_engine.start_x = 0u32;
        scan_engine.end_x = 640u32;
        scan_engine.enable = false;


        scan_engine.prop();
        scan_engine.posedge_clk();

        scan_engine.enable = true;

        for i in 0..256 {
            scan_engine.prop();
            assert_eq!(true, scan_engine.pixel_write_en, "Validate write en for pixel {}", i);
            assert_eq!(i, scan_engine.pixel_addr, "Validate address is equal for pixel {}", i);
            scan_engine.posedge_clk();
        }
    }

    #[test]
    fn negative_basic_raster_test() {
        let mut scan_engine = ScanEngine::new();

        // Setup edge values with values that cause a pixel to never be drawn
        scan_engine.e0 = -1i32 as u32;
        scan_engine.e1 = -1i32 as u32;
        scan_engine.e2 = -1i32 as u32;
        scan_engine.dy0 = -1i32 as u32;
        scan_engine.dy1 = -1i32 as u32;
        scan_engine.dy2 = -1i32 as u32;
        scan_engine.start_x = 0u32;
        scan_engine.end_x = 640u32;
        scan_engine.enable = false;


        scan_engine.prop();
        scan_engine.posedge_clk();

        scan_engine.enable = true;

        for i in 0..256 {
            scan_engine.prop();
            assert_eq!(false, scan_engine.pixel_write_en, "Validate write en for pixel {}", i);
            assert_eq!(i, scan_engine.pixel_addr, "Validate address is equal for pixel {}", i);
            scan_engine.posedge_clk();
        }
    }

    #[test]
    fn tri_raster_test() {
        let mut scan_engine = ScanEngine::new();

        // Setup edge values with values of actual triangle
        // that should draw pixels from 10-100:
        // Generated with: ./tools/tri.py 10 10 60 100 100 10
        scan_engine.e0 = 0i32 as u32;
        scan_engine.e1 = -900i32 as u32;
        scan_engine.e2 = 9000i32 as u32;
        scan_engine.dy0 = 0i32 as u32;
        scan_engine.dy1 = 90i32 as u32;
        scan_engine.dy2 = -90i32 as u32;
        scan_engine.start_x = 0u32;
        scan_engine.end_x = 110u32;
        scan_engine.enable = false;


        scan_engine.prop();
        scan_engine.posedge_clk();

        scan_engine.enable = true;

        println!("Hello");

        for i in 0..256 {
            scan_engine.prop();
            let write = i >= 10 && i <= 100;
            assert_eq!(write, scan_engine.pixel_write_en, "Validate write en for pixel {}", i);
            assert_eq!(i, scan_engine.pixel_addr, "Validate address is equal for pixel {}", i);
            assert_eq!(i == 110, scan_engine.done, "Validate scan done for pixel {}", i);
            scan_engine.posedge_clk();
        }
    }
}
