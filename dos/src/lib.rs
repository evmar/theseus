use std::{
    cell::{RefCell, RefMut},
    sync::LazyLock,
};

use host::SingleThreader;
use runtime::{CPU, Cont, Context, EXEData, Mappings, Memory, segofs};

/// DOSBox-X loads com files into this segment.
pub const DOSBOX_SEG: u16 = 0x813;

pub fn load(exe: &EXEData) -> Context {
    host::init();

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

    let mut f = exe.entry_point;
    let mut i = 0;
    loop {
        if i % 0x2000 == 0 {
            if let Some(int) = check_interrupts(ctx) {
                f = int;
            }
        }
        f = f.0(ctx);
        i += 1;
    }
}

pub fn run(exe: &EXEData) {
    let mut ctx = load(exe);
    start(&mut ctx, exe);
}

static STATE: LazyLock<SingleThreader<RefCell<State>>> =
    LazyLock::new(|| SingleThreader::new(RefCell::new(State::new())));

struct VGA {
    window: host::Window,
    surface: host::Surface,
    pixels32: Vec<u8>,
    palette: [[u8; 3]; 256],
    palette_index: (u8, u8),
}

impl VGA {
    fn new() -> Self {
        let mut window = host::host().create_window("VGA", 320, 200);
        let surface = window.create_surface(320, 200);
        let mut pixels32 = vec![];
        pixels32.resize(320 * 200 * 4, 0);
        VGA {
            window,
            surface,
            pixels32,
            palette: [[0; 3]; 256],
            palette_index: (0, 0),
        }
    }
}

struct State {
    pit_divisor: u16,
    pit_lobyte: Option<u8>,
    interrupt_handlers: [(u16, u16); 16],
    vga: Option<VGA>,
}

impl State {
    const fn new() -> Self {
        State {
            pit_divisor: 0,
            pit_lobyte: None,
            interrupt_handlers: [(0, 0); 16],
            vga: None,
        }
    }
}

fn state() -> RefMut<'static, State> {
    STATE.get().borrow_mut()
}

fn int10(ctx: &mut Context) {
    let func = ctx.cpu.regs.get_ah();
    match func {
        0x0 => {
            let mode = ctx.cpu.regs.get_al();
            assert_eq!(mode, 0x13);
            state().vga = Some(VGA::new());
        }
        _ => log::error!("TODO: int 10h (video) call {func:02x}"),
    }
}

fn int21(ctx: &mut Context) {
    let func = ctx.cpu.regs.get_ah();
    match func {
        0x25 => {
            let int = ctx.cpu.regs.get_al();
            let (seg, ofs) = (ctx.cpu.regs.get_ds(), ctx.cpu.regs.get_dx());
            state().interrupt_handlers[int as usize] = (seg, ofs);
        }
        0x35 => {
            let int = ctx.cpu.regs.get_al();
            let (seg, ofs) = state().interrupt_handlers[int as usize];
            ctx.cpu.regs.set_es(seg);
            ctx.cpu.regs.set_bx(ofs);
        }
        _ => log::error!("TODO: dos int 21h ({func:02x})"),
    }
}

pub fn int(ctx: &mut Context, interrupt: u8) {
    match interrupt {
        0x10 => int10(ctx),
        0x16 => {
            // TODO: dos int 0x16, keyboard?
            ctx.cpu.flags.insert(runtime::Flags::ZF);
        }
        0x21 => int21(ctx),
        _ => log::error!("TODO: dos int {interrupt:x}h"),
    }
}

/// Handle an `out` instruction that writes to a Programmable Interval Timer (PIT) port.
fn out_pit(_ctx: &mut Context, port: u16, data: u8) {
    // https://wiki.osdev.org/Programmable_Interval_Timer
    match port {
        0x40..=0x42 => {
            assert_eq!(port, 0x40); // timer interrupt
            let mut state = state();
            match state.pit_lobyte {
                Some(lo) => {
                    state.pit_lobyte = None;
                    state.pit_divisor = (data as u16) << 8 | (lo as u16);
                    log::info!("PIT divisor set to {:#x}", state.pit_divisor);
                }
                None => state.pit_lobyte = Some(data),
            }
        }
        0x43 => {
            let channel = data >> 6;
            let access_mode = (data >> 4) & 0b11;
            let operating_mode = (data >> 1) & 0b11;
            let bcd_mode = data & 0b1;
            assert_eq!(channel, 0); // timer interrupt
            assert_eq!(access_mode, 0b11); // lo/hi byte
            assert_eq!(operating_mode, 0b11); // square wave
            assert_eq!(bcd_mode, 0); // binary mode
        }
        _ => unreachable!(),
    }
}

/// Handle an `out` instruction that writes to a VGA port.
fn out_vga(_ctx: &mut Context, port: u16, data: u8) {
    let mut state = state();
    let vga = state.vga.as_mut().unwrap();
    // http://www.osdever.net/FreeVGA/vga/portidx.htm
    match port {
        0x3c8 => vga.palette_index = (data, 0),
        0x3c9 => {
            let (mut index, mut color) = vga.palette_index;
            vga.palette[index as usize][color as usize] = data;
            color += 1;
            if color == 3 {
                color = 0;
                index = index.wrapping_add(1);
            }
            vga.palette_index = (index, color);
        }
        _ => log::error!("TODO: out({:#x}, {:#x}): graphics control", port, data),
    }
}

pub fn out(ctx: &mut Context, port: u16, data: u8) {
    match port {
        0x40..=0x43 => out_pit(ctx, port, data),
        0x3C0..=0x3DF => out_vga(ctx, port, data),
        _ => log::error!("TODO: out({:#x}, {:#x})", port, data),
    }
}

pub fn dump_com(ctx: &mut Context) -> &[u8] {
    let data = &ctx.memory[segofs(DOSBOX_SEG, 0x100)..];
    let end = data.iter().rposition(|&x| x != 0);
    let data = &data[..end.unwrap() + 1];
    data
}

fn check_interrupts(ctx: &mut Context) -> Option<Cont> {
    let mut state = state();

    let now = host::host().time();
    // TODO: actually use time value to judge whether to invoke timer

    let timer = state.interrupt_handlers[8];
    if timer.0 != 0 {
        let (seg, ofs) = timer;
        log::info!("timer @{now} {seg:x}:{ofs:x}");

        assert_eq!(ctx.cpu.regs.cs, seg);
        let esp = ctx.cpu.regs.esp;
        ctx.push16(ctx.cpu.flags.bits() as u16);
        ctx.push16(seg);
        ctx.push16(ofs);

        let mut f = ctx.indirect16(ofs);
        while ctx.cpu.regs.esp != esp {
            // don't check interrupts while running interrupt handler
            f = f.0(ctx);
        }
    }

    if let Some(vga) = &mut state.vga {
        vga.update_screen(ctx);
    }

    None
}

impl VGA {
    fn update_screen(&mut self, ctx: &mut Context) {
        host::host().poll(); // pump msg loop

        let pixels_seg = 0xa000;
        let pixels8 = &ctx.memory[segofs(pixels_seg, 0)..][..(320 * 200)];

        for (p32, &p8) in self.pixels32.chunks_exact_mut(4).zip(pixels8) {
            p32[0] = p8;
            p32[1] = p8;
            p32[2] = p8;
            p32[3] = 0xff;
        }

        self.surface.set_pixels(&self.pixels32, 320 * 4);
        self.window.render(&mut self.surface);
    }
}
