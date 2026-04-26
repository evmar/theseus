use runtime::{Cont, Context};

use crate::do_unpack;

pub fn x41f079(ctx: &mut Context) -> Cont {
    do_unpack(ctx);
    std::process::exit(0);
}
