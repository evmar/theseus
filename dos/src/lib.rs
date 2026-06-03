use runtime::{CPU, Context, EXEData, Mappings, Memory, segofs};

/// DOSBox-X loads com files into this segment.
pub const DOSBOX_SEG: u16 = 0x813;

pub fn load(exe: &EXEData) -> Context {
    logger::init();

    let memory_size = segofs(DOSBOX_SEG, 0) + (64 << 10);
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

pub fn int10() {
    log::warn!("TODO: int 10h (video)");
}

pub fn int21() {
    log::warn!("TODO: int 21h (dos)");
}
