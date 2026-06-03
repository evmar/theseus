use runtime::{CPU, Context, EXEData, Mappings, Memory};

/// DOSBox-X loads com files into this segment.
pub const DOSBOX_SEG: u16 = 0x813;

pub fn load(exe: &EXEData) -> Context {
    logger::init();

    let memory_size = 1 << 20;
    let memory = Memory::leak_new(memory_size as usize);

    let mut ctx = Context {
        cpu: CPU::default(),
        thread_handle: 0,
        thread_id: 1,
        memory,
        blocks: exe.blocks,
        recent: [Context::return_from_x86; 4],
    };
    ctx.cpu.real_mode = true;

    let mut mappings = Mappings::default();
    (exe.init_memory)(&mut ctx, &mut mappings);

    // initial register values copied to match dosbox

    ctx.cpu.regs.cs = DOSBOX_SEG;
    ctx.cpu.regs.ds = DOSBOX_SEG;
    ctx.cpu.regs.es = DOSBOX_SEG;
    ctx.cpu.regs.ss = DOSBOX_SEG;

    // initial cx: https://stackoverflow.com/questions/79440940/why-cx-register-already-has-a-non-zero-value-on-startup-of-a-dos-program-unlike
    ctx.cpu.regs.ecx = 0xff;
    ctx.cpu.regs.esp = 0xfffe;
    ctx
}

pub fn start(ctx: &mut Context, exe: &EXEData) {
    assert!(ctx.cpu.real_mode);
    ctx.cpu_loop(exe.entry_point, 0);
    panic!();
}

pub fn run(exe: &EXEData) {
    let mut ctx = load(exe);
    start(&mut ctx, exe);
}

pub fn int10(ctx: &mut Context) {
    let func = ctx.cpu.regs.get_ah();
    match func {
        0x0 => {
            let mode = ctx.cpu.regs.get_al();
            log::warn!("TODO: set video mode {mode:02x}");
        }
        _ => log::error!("TODO: int 10h (video) call {func:02x}"),
    }
}

pub fn int21(ctx: &mut Context) {
    let func = ctx.cpu.regs.get_ah();
    match func {
        0x25 => {
            let int = ctx.cpu.regs.get_al();
            let (seg, ofs) = (ctx.cpu.regs.get_ds(), ctx.cpu.regs.get_dx());
            log::warn!("TODO: set interrupt handler {int:02x} to {seg:04x}:{ofs:04x}");
        }
        0x35 => {
            let int = ctx.cpu.regs.get_al();
            log::warn!("TODO: get interrupt handler {int:02x}, returning 0");
            let (seg, ofs) = (0, 0);
            ctx.cpu.regs.set_es(seg);
            ctx.cpu.regs.set_bx(ofs);
        }
        _ => log::error!("TODO: dos int 21h ({func:02x})"),
    }
}

pub fn out(_ctx: &mut Context, port: u16, data: u8) {
    match port {
        0x40..=0x42 => {
            // https://wiki.osdev.org/Programmable_Interval_Timer
            log::warn!("TODO: out({:#x}, {:#x}): PIT channel", port, data);
        }
        0x43 => {
            // https://wiki.osdev.org/Programmable_Interval_Timer
            log::warn!("TODO: out({:#x}, {:#x}): PIT control", port, data);
        }

        0x3C0..=0x3DF => {
            // http://www.osdever.net/FreeVGA/vga/portidx.htm
            match port {
                0x3c8 => log::warn!("TODO: out({:#x}, {:#x}): DAC write address", port, data),
                0x3c9 => log::warn!("TODO: out({:#x}, {:#x}): DAC write data", port, data),
                _ => log::error!("TODO: out({:#x}, {:#x}): graphics control", port, data),
            }
        }
        _ => {
            log::error!("TODO: out({:#x}, {:#x})", port, data);
        }
    }
}
