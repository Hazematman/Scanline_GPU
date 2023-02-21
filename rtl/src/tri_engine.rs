use kaze::*;

use crate::scan_engine::*;

pub struct TriEngine<'a> {
    pub m: &'a Module<'a>,

    pub new_scanline: &'a Input<'a>,
    pub done_screen: &'a Input<'a>,

    pub pixel_addr: &'a Output<'a>,
    pub pixel_data: &'a Output<'a>,
    pub pixel_write_en: &'a Output<'a>,
}

enum TriState {
    Start,
    Wait,
    Draw,
    PostDraw,
    NumStates,
}

const NUM_STATE_BITS: u32 = u32::BITS - (TriState::NumStates as u32).leading_zeros();

impl<'a> TriEngine<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> TriEngine<'a> {
        const BIT_WIDTH: u32 = 16;
        let m = p.module(instance_name, "TriEngine");

        let scan_engine = ScanEngine::new("scan_engine", m);

        let new_scanline = m.input("new_scanline", 1);
        let done_screen = m.input("done_screen", 1);

        let params = [
                m.reg("e0", BIT_WIDTH),
                m.reg("e1", BIT_WIDTH),
                m.reg("e2", BIT_WIDTH),
                m.reg("dy0", BIT_WIDTH),
                m.reg("dy1", BIT_WIDTH),
                m.reg("dy2", BIT_WIDTH),
                m.reg("dx0", BIT_WIDTH),
                m.reg("dx1", BIT_WIDTH),
                m.reg("dx2", BIT_WIDTH),
                m.reg("start_x", BIT_WIDTH),
                m.reg("start_y", BIT_WIDTH),
                m.reg("end_x", BIT_WIDTH),
                m.reg("end_y", BIT_WIDTH),
            ];

        params[0].default_value(-900i32 as u16);
        params[1].default_value(-400i32 as u16);
        params[2].default_value(9400i32 as u16);
        params[3].default_value(0i32 as u16);
        params[4].default_value(90i32 as u16);
        params[5].default_value(-90i32 as u16);
        params[6].default_value(-90i32 as u16);
        params[7].default_value(50i32 as u16);
        params[8].default_value(40i32 as u16);
        params[9].default_value(0i32 as u16);
        params[10].default_value(0i32 as u16);
        params[11].default_value(100i32 as u16);
        params[12].default_value(100i32 as u16);

        let state = m.reg("state", NUM_STATE_BITS);
        state.default_value(TriState::Start as u32);

        scan_engine.edges[0].drive(params[0]);
        scan_engine.edges[1].drive(params[1]);
        scan_engine.edges[2].drive(params[2]);

        scan_engine.dy[0].drive(params[3]);
        scan_engine.dy[1].drive(params[4]);
        scan_engine.dy[2].drive(params[5]);

        scan_engine.start_x.drive(params[6]);
        scan_engine.end_x.drive(params[11]);

        let next_state = if_(state.eq(m.lit(TriState::Start as u32, NUM_STATE_BITS)), {
            m.lit(TriState::Wait as u32, NUM_STATE_BITS)
        }).else_if(state.eq(m.lit(TriState::Wait as u32, NUM_STATE_BITS)), {
            if_(done_screen, {
                m.lit(TriState::Start as u32, NUM_STATE_BITS)
            }).else_if(new_scanline, {
                m.lit(TriState::Draw as u32, NUM_STATE_BITS)
            }).else_({
                m.lit(TriState::Wait as u32, NUM_STATE_BITS)
            })
        }).else_if(state.eq(m.lit(TriState::Draw as u32, NUM_STATE_BITS)), {
            if_(scan_engine.done | new_scanline, {
                m.lit(TriState::PostDraw as u32, NUM_STATE_BITS)
            }).else_({
                m.lit(TriState::Draw as u32, NUM_STATE_BITS)
            })
        }).else_if(state.eq(m.lit(TriState::PostDraw as u32, NUM_STATE_BITS)), {
            m.lit(TriState::Wait as u32, NUM_STATE_BITS)
        }).else_({
            m.lit(TriState::Start as u32, NUM_STATE_BITS)
        });

        scan_engine.enable.drive(next_state.eq(m.lit(TriState::Draw as u32, NUM_STATE_BITS)));

        let (e0, e1, e2) = if_(state.eq(m.lit(TriState::Start as u32, NUM_STATE_BITS)), {
            (m.lit(-900i32 as u16, BIT_WIDTH), m.lit(-400i32 as u16, BIT_WIDTH), m.lit(9400i32 as u16, BIT_WIDTH))
        }).else_if(state.eq(m.lit(TriState::PostDraw as u32, NUM_STATE_BITS)), {
            (params[0] - params[6], params[1] - params[7], params[2] - params[8])
        }).else_({
            (params[0], params[1], params[2])
        });

        state.drive_next(next_state);
        params[0].drive_next(e0);
        params[1].drive_next(e1);
        params[2].drive_next(e2);
        for i in 3..13 {
            params[i].drive_next(params[i])
        }

        let pixel_addr = m.output("pixel_addr", scan_engine.pixel_addr);
        let pixel_data = m.output("pixel_data", scan_engine.pixel_data);
        let pixel_write_en = m.output("pixel_write_en", scan_engine.pixel_write_en);

        m.output("state_debug", state);
        m.output("done_line", scan_engine.done);

        TriEngine {
            m: m,
            
            new_scanline,
            done_screen,

            pixel_addr,
            pixel_data,
            pixel_write_en,
        }
    }
}
