#[cfg(test)]
mod tests {
    mod modules {
        include!(concat!(env!("OUT_DIR"), "/modules.rs"));
    }

    use modules::*;

    #[test]
    fn basic_vga_test() {
        // vga parameters for 640x480
        const COLUMNS:u32 = 800;
        const COLUMNS_VISIBLE:u32 = 640;
        const ROWS:u32 = 525;
        const ROWS_VISIBLE:u32 = 480;
        const HORIZONTAL_FRONT_PORCH:u32 = 656;
        const HORIZONTAL_BACK_PORCH:u32 = 752;
        const VERTICAL_FRONT_PORCH:u32 = 490;
        const VERTICAL_BACK_PORCH:u32 = 492;
        let mut vga = Vga::new();

        vga.reset();
        vga.prop();

        for y in 0..525 {
            for x in 0..800 {
                vga.prop();

                println!("Testing {}, {}", x, y);
                assert_eq!(x, vga.current_column, "Validate column match");
                assert_eq!(y, vga.current_row, "Validate row match");

                assert_eq!(x < HORIZONTAL_FRONT_PORCH || x >= HORIZONTAL_BACK_PORCH,
                           vga.h_sync, "Validate h_sync is correct");
                assert_eq!(y < VERTICAL_FRONT_PORCH || y >= VERTICAL_BACK_PORCH,
                           vga.v_sync, "Validate v_sync is correct");
                assert_eq!(x < COLUMNS_VISIBLE && y < ROWS_VISIBLE,
                           vga.data_enable, "Validate data_enable is correct");
                vga.posedge_clk();
            }
        }
    }
}
