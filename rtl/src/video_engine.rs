use kaze::*;


use crate::tri_engine::*;
use crate::vga::*;

pub struct VideoEngine<'a> {
    pub m: &'a Module<'a>,

    pub r: &'a Output<'a>,
    pub g: &'a Output<'a>,
    pub b: &'a Output<'a>,
    pub h_sync: &'a Output<'a>,
    pub v_sync: &'a Output<'a>,
}

impl<'a> VideoEngine<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> VideoEngine<'a> {
        const BIT_WIDTH:u32 = 16;
        const MAX_MEM_SIZE:u32 = 10; // log2(1024)
        let m = p.module(instance_name, "VideoEngine");

        let tri_engine = TriEngine::new("TriEngine", m);
        let vga = Vga::new("Vga", m);

        let lines = [
            m.mem("line0", MAX_MEM_SIZE, 16),
            m.mem("line1", MAX_MEM_SIZE, 16),
        ];

        let buf_selector = m.reg("buf_selector", 1);
        buf_selector.default_value(0u32);

        lines[0].write_port(tri_engine.pixel_addr.bits(MAX_MEM_SIZE-1, 0), tri_engine.pixel_data, 
                            tri_engine.pixel_write_en & buf_selector);
        lines[1].write_port(tri_engine.pixel_addr.bits(MAX_MEM_SIZE-1, 0), tri_engine.pixel_data, 
                            tri_engine.pixel_write_en & !buf_selector);

        let new_scanline = vga.current_column.eq(m.lit(Vga::COLUMNS - 2, BIT_WIDTH));
        let done_screen = new_scanline & vga.current_row.eq(m.lit(Vga::ROWS, BIT_WIDTH));

        let next_buf = if_(new_scanline, {
            !buf_selector
        }).else_({
            buf_selector
        });

        let pixel_data = if_(!vga.data_enable, {
            m.lit(0u32, BIT_WIDTH)
        }).else_if(!buf_selector, {
            lines[0].read_port(vga.current_column.bits(MAX_MEM_SIZE-1, 0), !buf_selector)
        }).else_({
            lines[1].read_port(vga.current_column.bits(MAX_MEM_SIZE-1, 0), buf_selector)
        });

        let r_out = m.lit(0u32, 3).concat(pixel_data.bits(15, 11));
        let g_out = m.lit(0u32, 2).concat(pixel_data.bits(10, 5));
        let b_out = m.lit(0u32, 3).concat(pixel_data.bits(4, 0));

        let r = m.output("r", r_out);
        let g = m.output("g", g_out);
        let b = m.output("b", b_out);
        let h_sync = m.output("h_sync", vga.h_sync);
        let v_sync = m.output("v_sync", vga.v_sync);

        buf_selector.drive_next(next_buf);

        tri_engine.new_scanline.drive(new_scanline);
        tri_engine.done_screen.drive(done_screen);

        VideoEngine {
            m,

            r,
            g,
            b,
            h_sync,
            v_sync,
        }
    }
}
