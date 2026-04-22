use runtime::{Cont, Context};

use crate::do_unpack;

pub fn x004085dd(_ctx: &mut Context) -> Cont {
    do_unpack();
    std::process::exit(0);
}
