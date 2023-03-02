use kaze::*;

pub struct ScanEngine<'a> {
    pub m: &'a Module<'a>,

    pub edges: [&'a Input<'a>; 3],
    pub dy: [&'a Input<'a>; 3],
    pub start_x: &'a Input<'a>,
    pub end_x: &'a Input<'a>,
    pub enable: &'a Input<'a>,

    pub pixel_addr: &'a Output<'a>,
    pub pixel_data: &'a Output<'a>,
    pub pixel_write_en: &'a Output<'a>,
    pub done: &'a Output<'a>,
}

impl<'a> ScanEngine<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> ScanEngine<'a> {
        const BIT_WIDTH: u32 = 16;
        let m = p.module(instance_name, "ScanEngine");

        let e0 = m.input("e0", BIT_WIDTH);
        let e1 = m.input("e1", BIT_WIDTH);
        let e2 = m.input("e2", BIT_WIDTH);
        let dy0 = m.input("dy0", BIT_WIDTH);
        let dy1 = m.input("dy1", BIT_WIDTH);
        let dy2 = m.input("dy2", BIT_WIDTH);
        let enable = m.input("enable", 1);
        let start_x = m.input("start_x", BIT_WIDTH);
        let end_x = m.input("end_x", BIT_WIDTH);

        let index = m.reg("index", BIT_WIDTH);
        let c_e0 = m.reg("c_e0", BIT_WIDTH);
        let c_e1 = m.reg("c_e1", BIT_WIDTH);
        let c_e2 = m.reg("c_e2", BIT_WIDTH);

        let inc_e0 = c_e0 + dy0;
        let inc_e1 = c_e1 + dy1;
        let inc_e2 = c_e2 + dy2;

        // Check if current edge functions are valid by reading
        // the sign bit of the edge function. If the sign bit is
        // zero then the edge function is valid.
        let e0_valid = !c_e0.bit(BIT_WIDTH - 1);
        let e1_valid = !c_e1.bit(BIT_WIDTH - 1);
        let e2_valid = !c_e2.bit(BIT_WIDTH - 1);

        let pixel_value = m.lit(0xF800u32, BIT_WIDTH);
        let done_internal = index.eq(end_x);
        let running = enable & !done_internal;

        let pixel_addr = m.output("pixel_addr", index);
        let pixel_data = m.output("pixel_data", pixel_value);
        let pixel_write_en = m.output("pixel_write_en", running & e0_valid & e1_valid & e2_valid);
        let done = m.output("done", done_internal);

        //let c_e0_out = m.output("c_e0_out", c_e0);
        //let c_e1_out = m.output("c_e1_out", c_e1);
        //let c_e2_out = m.output("c_e2_out", c_e2);

        let next_index = if_(enable, {
            index + m.lit(1u32, BIT_WIDTH)
        }).else_({
            start_x
        });

        index.drive_next(next_index);

        let (next_c_e0, next_c_e1, next_c_e2) = if_(enable, {
            (inc_e0, inc_e1, inc_e2)
        }).else_({
            (e0, e1, e2)
        });

        c_e0.drive_next(next_c_e0);
        c_e1.drive_next(next_c_e1);
        c_e2.drive_next(next_c_e2);

        // Return created scan engine
        ScanEngine {
            m: m,
            edges: [e0, e1, e2],
            dy: [dy0, dy1, dy2],
            start_x: start_x,
            end_x: end_x,
            enable: enable,
            pixel_addr: pixel_addr,
            pixel_data: pixel_data,
            pixel_write_en: pixel_write_en,
            done: done,
        }
    }
}
