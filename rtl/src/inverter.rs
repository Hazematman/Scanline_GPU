use kaze::*;

pub struct Inverter<'a> {
    pub m: &'a Module<'a>,
}

impl<'a> Inverter<'a> {
    pub fn new(instance_name: impl Into<String>, p: &'a impl ModuleParent<'a>) -> Inverter<'a> {
        let m = p.module(instance_name, "Inverter");
        let i = m.input("i", 1);

        m.output("o", !i);

        Inverter {
            m,
        }
    }
}
