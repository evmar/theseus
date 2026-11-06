//! Implementation of the BLASTER.DRV API.
//! We swap this in in place of loading that program.

use runtime::{Cont, Context};
use std::sync::Mutex;

pub struct Audio {
    pub buf: Vec<u8>,
}
pub static AUDIO: Mutex<Audio> = Mutex::new(Audio { buf: vec![] });

pub mod api {
    use super::*;

    pub fn init(ctx: &mut Context) -> Cont {
        ctx.cpu.regs.set_ax(0); // success
        ctx.cpu.regs.set_di(0); // buffer id
        ctx.retf16(0)
    }

    pub fn get_buffer(ctx: &mut Context) -> Cont {
        // let audio = AUDIO.lock().unwrap();
        ctx.cpu.regs.set_di(0); // buffer id
        ctx.retf16(0)
    }

    pub fn put_sample(ctx: &mut Context) -> Cont {
        let sample = ctx.cpu.regs.get_al();
        let mut audio = AUDIO.lock().unwrap();
        audio.buf.push(sample);
        ctx.cpu.regs.set_di(0);
        ctx.retf16(0)
    }

    pub fn noop(ctx: &mut Context) -> Cont {
        ctx.retf16(0)
    }

    pub fn flush(ctx: &mut Context) -> Cont {
        ctx.cpu.regs.set_di(0);
        ctx.retf16(0)
    }
}
