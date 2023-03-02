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
        const FIRST_LINE:u32 = 168;
        const LAST_LINE:u32 = 312;
        const FIRST_COLUMN:u32 = 192;
        const LAST_COLUMN:u32 = 448;
        const END_COLUMN:u32 = 800;
        let m = p.module(instance_name, "VideoEngine");

        let tri_engine = TriEngine::new("TriEngine", m);
        let vga = Vga::new("Vga", m);

        let clear_index = m.reg("clear_index", BIT_WIDTH);
        clear_index.default_value(0u32);

        let lines = [
            m.mem("line0", MAX_MEM_SIZE, 16),
            m.mem("line1", MAX_MEM_SIZE, 16),
        ];

        let buf_selector = m.reg("buf_selector", 1);
        buf_selector.default_value(0u32);

        let visible_line = vga.current_row.ge(m.lit(FIRST_LINE-1, BIT_WIDTH))
            & vga.current_row.lt(m.lit(LAST_LINE, BIT_WIDTH));
        let new_scanline = vga.current_column.ge(m.lit(255u32, BIT_WIDTH))
            & vga.current_column.le(m.lit(Vga::COLUMNS - 2, BIT_WIDTH));
        let done_screen = !visible_line;

        let next_buf = if_(new_scanline, {
            !buf_selector
        }).else_({
            buf_selector
        });

        let end_of_line = vga.current_column.gt(m.lit(LAST_COLUMN, BIT_WIDTH))
            & vga.current_column.le(m.lit(END_COLUMN, BIT_WIDTH));

        let next_clear_index = if_(end_of_line, {
            clear_index + m.lit(1u32, BIT_WIDTH)
        }).else_({
            m.lit(0u32, BIT_WIDTH)
        });
        clear_index.drive_next(next_clear_index);

        for (i, line) in lines.iter().enumerate() {
            let clear_enable = buf_selector.ne(m.lit(i as u32, 1)) & end_of_line;
            let draw_enable = buf_selector.eq(m.lit(i as u32, 1)) & tri_engine.pixel_write_en;
            let (index, data) = if_(clear_enable, {
                (clear_index, m.lit(0u32, BIT_WIDTH))
            }).else_({
                (tri_engine.pixel_addr, tri_engine.pixel_data)
            });

            line.write_port(index.bits(MAX_MEM_SIZE-1, 0), data, clear_enable | draw_enable);
        }

        let visible_column = vga.current_column.ge(m.lit(FIRST_COLUMN-1, BIT_WIDTH))
            & vga.current_column.lt(m.lit(FIRST_COLUMN + 255u32, BIT_WIDTH));
        let visible_index = vga.current_column - m.lit(FIRST_COLUMN-1, BIT_WIDTH);

        let pixel_data = if_(!vga.data_enable | !visible_column | !visible_line, {
            m.lit(0u32, BIT_WIDTH)
        }).else_if(!buf_selector, {
            lines[0].read_port(visible_index.bits(MAX_MEM_SIZE-1, 0), !buf_selector)
        }).else_({
            lines[1].read_port(visible_index.bits(MAX_MEM_SIZE-1, 0), buf_selector)
        });

        let r_out = pixel_data.bits(15, 11).concat(m.lit(0u32, 3));
        let g_out = pixel_data.bits(10, 5).concat(m.lit(0u32, 2));
        let b_out = pixel_data.bits(4, 0).concat(m.lit(0u32, 3));

        m.output("debug_pixel", tri_engine.pixel_data);
        m.output("debug_index", tri_engine.pixel_addr);
        m.output("debug_write", tri_engine.pixel_write_en);
        m.output("debug_visible_index", visible_index);
        m.output("debug_clear_index", clear_index);

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
