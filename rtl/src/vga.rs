use kaze::*;

pub struct Vga<'a> {
    pub m: &'a Module<'a>,

    pub h_sync: &'a Output<'a>,
    pub v_sync: &'a Output<'a>,
    pub data_enable: &'a Output<'a>,
    pub current_column: &'a Output<'a>,
    pub current_row: &'a Output<'a>,
}

impl<'a> Vga<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Vga<'a> {
        const BIT_WIDTH: u32 = 16;
        const COLUMNS:u32 = 800;
        const COLUMNS_VISIBLE:u32 = 640;
        const ROWS:u32 = 525;
        const ROWS_VISIBLE:u32 = 480;
        const HORIZONTAL_FRONT_PORCH:u32 = 656;
        const HORIZONTAL_BACK_PORCH:u32 = 752;
        const VERTICAL_FRONT_PORCH:u32 = 490;
        const VERTICAL_BACK_PORCH:u32 = 492;
        let m = p.module(instance_name, "Vga");
        
        let column = m.reg("column", BIT_WIDTH);
        let row = m.reg("row", BIT_WIDTH);
        column.default_value(0u32);
        row.default_value(0u32);

        let end_of_line = column.eq(m.lit(COLUMNS-1u32, BIT_WIDTH));
        let end_of_screen = column.eq(m.lit(ROWS-1u32, BIT_WIDTH));

        let (next_column, next_row) = if_(end_of_line & end_of_screen, {
            (m.lit(0u32, BIT_WIDTH), m.lit(0u32, BIT_WIDTH))
        }).else_if(end_of_screen, {
            (m.lit(0u32, BIT_WIDTH), row + m.lit(1u32, BIT_WIDTH))
        }).else_({
            (column + m.lit(1u32, BIT_WIDTH), row)
        });

        column.drive_next(next_column);
        row.drive_next(next_row);

        let h_sync = m.output("h_sync", column.lt(m.lit(HORIZONTAL_FRONT_PORCH, BIT_WIDTH))
                                        | column.ge(m.lit(HORIZONTAL_BACK_PORCH, BIT_WIDTH)));

        let v_sync = m.output("v_sync", row.lt(m.lit(VERTICAL_FRONT_PORCH, BIT_WIDTH))
                                        | row.ge(m.lit(VERTICAL_BACK_PORCH, BIT_WIDTH)));
        let data_enable = m.output("data_enable", column.lt(m.lit(COLUMNS_VISIBLE, BIT_WIDTH))
                                                  & row.lt(m.lit(ROWS_VISIBLE, BIT_WIDTH)));
        let current_column = m.output("current_column", column);
        let current_row = m.output("current_row", row);
        
        Vga {
            m,
            h_sync,
            v_sync,
            data_enable,
            current_column,
            current_row,
        }
    }
}
