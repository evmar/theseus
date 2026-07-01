mod timer;
mod vga;

use std::{
    cell::{RefCell, RefMut},
    sync::LazyLock,
};

use host::SingleThreader;
use runtime::{CPU, Context, EXEData, Mappings, Memory, segofs};

use crate::{timer::PIT, vga::VGA};

/// DOSBox-X loads com files into this segment.
pub const DOSBOX_SEG: u16 = 0x813;

#[repr(C)]
#[derive(zerocopy::IntoBytes, zerocopy::Immutable)]
struct PSP {
    int20: [u8; 2],
    memory_top: u16,
}

pub fn load(exe: &EXEData) -> Context {
    host::init();

    let memory_size = 1 << 20;
    let mut memory = Memory::leak_new(memory_size as usize);

    memory.write(
        exe.image_base,
        PSP {
            int20: [0xcd, 0x20],
            memory_top: 0x9fff, // from dosbox
        },
    );

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
    (exe.init)(&mut ctx, &mut mappings);

    ctx
}

pub fn start(ctx: &mut Context, exe: &EXEData) {
    assert!(ctx.cpu.real_mode);

    let mut f = exe.entry_point;
    let mut i = 0;
    loop {
        if i % 0x2000 == 0 {
            state().check_interrupts(ctx);
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

struct State {
    pit: PIT,
    // IVT; TODO: this actually lives in in memory at 0000:0000, not sure if anything depends on that
    interrupt_handlers: [(u16, u16); 0x30],
    vga: Option<VGA>,
}

impl State {
    fn new() -> Self {
        State {
            pit: PIT::default(),
            interrupt_handlers: [(0, 0); 0x30],
            vga: None,
        }
    }
}

fn state() -> RefMut<'static, State> {
    STATE.get().borrow_mut()
}

/// int10 is graphics calls.
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

/// int21 is used for system calls, like file i/o and exiting.
fn int21(ctx: &mut Context) {
    let func = ctx.cpu.regs.get_ah();
    match func {
        // write to interrupt table
        0x25 => {
            let int = ctx.cpu.regs.get_al();
            let (seg, ofs) = (ctx.cpu.regs.get_ds(), ctx.cpu.regs.get_dx());
            state().interrupt_handlers[int as usize] = (seg, ofs);
        }
        // get DOS version
        0x30 => {
            // these values match dosbox
            ctx.cpu.regs.set_ax(5);
            ctx.cpu.regs.set_bx(0xff00);
            ctx.cpu.regs.set_cx(0);
        }
        // read from interrupt table
        0x35 => {
            let int = ctx.cpu.regs.get_al();
            let (seg, ofs) = state().interrupt_handlers[int as usize];
            ctx.cpu.regs.set_es(seg);
            ctx.cpu.regs.set_bx(ofs);
        }
        // write to file
        0x40 => {
            use std::io::Write;
            let handle = ctx.cpu.regs.get_bx();
            let len = ctx.cpu.regs.get_cx();
            let addr = segofs(ctx.cpu.regs.get_ds(), ctx.cpu.regs.get_dx());
            let buf = &ctx.memory[addr..][..len as usize];
            match handle {
                1 => std::io::stdout().lock().write_all(buf).unwrap(),
                2 => std::io::stderr().lock().write_all(buf).unwrap(),
                _ => log::error!("TODO: dos write to file {handle} {buf:?}"),
            }
            ctx.cpu.regs.set_ax(len); // bytes written
            ctx.cpu.flags.remove(runtime::Flags::CF); // no error
        }
        // resize memory block
        0x4a => {
            let size = ctx.cpu.regs.get_bx() << 4;
            let seg = ctx.cpu.regs.es;
            log::warn!("TODO: resize memory seg {seg:x} to {size:x}");
            ctx.cpu.flags.remove(runtime::Flags::CF);
            let avail = 0xffff;
            ctx.cpu.regs.set_bx(avail);
        }
        // error exit
        0x4c => {
            let code = ctx.cpu.regs.get_al();
            std::process::exit(code as i32);
        }
        _ => log::error!("TODO: dos int 21h ({func:02x})"),
    }
}

pub fn int(ctx: &mut Context, interrupt: u8) {
    // https://en.wikibooks.org/wiki/First_steps_towards_system_programming_under_MS-DOS_7/Selected_interrupt_handlers
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

pub fn out(ctx: &mut Context, port: u16, data: u8) {
    match port {
        0x20 => { /* end of interrupt, ignore */ }
        0x40..=0x43 => state().pit.out(ctx, port, data),
        0x3C0..=0x3DF => state().vga.as_mut().unwrap().io_out(port, data),
        _ => log::error!("TODO: out({:#x}, {:#x})", port, data),
    }
}

pub fn dump_com(ctx: &mut Context) -> &[u8] {
    let data = &ctx.memory[segofs(DOSBOX_SEG, 0x100)..];
    let end = data.iter().rposition(|&x| x != 0);
    let data = &data[..end.unwrap() + 1];
    data
}

impl State {
    fn check_interrupts(&mut self, ctx: &mut Context) {
        self.pit.check_timer(ctx, self.interrupt_handlers[8]);
        if let Some(vga) = &mut self.vga {
            vga.update_screen(ctx);
        }
    }
}

/// Handler for address 0; what happens if the entry point returns.
pub fn exit(_ctx: &mut Context) -> runtime::Cont {
    std::process::exit(0);
}
