use crate::{Flags, Machine};

pub fn stc(m: &mut Machine) {
    m.flags.insert(Flags::CF);
}

pub fn clc(m: &mut Machine) {
    m.flags.remove(Flags::CF);
}

pub fn std(m: &mut Machine) {
    m.flags.insert(Flags::DF);
}

pub fn cld(m: &mut Machine) {
    m.flags.remove(Flags::DF);
}

pub fn sahf(m: &mut Machine) {
    // This constructs flags from the AH register, but only specific flags.
    let flags = Flags::from_bits(m.regs.get_ah() as u32).unwrap();
    m.flags.set(Flags::CF, flags.contains(Flags::CF));
    m.flags.set(Flags::PF, flags.contains(Flags::PF));
    // m.flags.set(Flags::AF, flags.contains(Flags::AF));
    m.flags.set(Flags::ZF, flags.contains(Flags::ZF));
    m.flags.set(Flags::OF, flags.contains(Flags::OF));
}
