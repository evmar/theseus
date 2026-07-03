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
    /// TODO: other fields, pad out for now
    padding1: [u8; 0x28],
    /// environment segment
    environment: u16,
    padding2: [u8; 0x52],
    args_len: u8,
    args: [u8; 0x7f],
}

impl PSP {
    fn new() -> Self {
        PSP {
            int20: [0xcd, 0x20],
            memory_top: 0,
            padding1: [0; 0x28],
            environment: 0,
            padding2: [0; 0x52],
            args_len: 0,
            args: [0; 0x7f],
        }
    }

    fn set_args(&mut self, args: &str) {
        assert!(args.len() <= 0x7e);
        self.args_len = args.len() as u8;
        self.args[..args.len()].copy_from_slice(args.as_bytes());
        self.args[args.len()] = b'\r';
    }
}

use zerocopy::byteorder::little_endian::U16;

/// Memory Control Block
/// Note that owner/size are unaligned, so we use zerocopy's unaligned U16 not u16.
#[repr(C)]
#[derive(zerocopy::IntoBytes, zerocopy::Immutable)]
struct MCB {
    /// 'M': more in chain, 'Z': last MCB
    typ: u8,
    /// PSP segment of block owner
    owner: U16,
    /// Size of block in paragraphs
    size: U16,
    reserved: [u8; 3],
    owner_name: [u8; 8],
}

pub fn load(exe: &EXEData, command_line: Option<&str>) -> Context {
    host::init();

    let memory_size = 1 << 20;
    let mut memory = Memory::leak_new(memory_size as usize);
    // programs can write to memory address zero to overwrite the IVT
    memory.null_page = false;

    // MCB goes in the paragraph before the PSP.
    // TODO: values copied from dosbox
    let mcb = MCB {
        typ: b'M',
        owner: U16::new(0x813),
        size: U16::new(0x2cb1),
        reserved: [0; 3],
        owner_name: [0; 8],
    };
    memory.write(exe.image_base - 0x10, mcb);

    // from dosbox
    let environment = [
        b"COMSPEC=Z:\\COMMAND.COM".as_slice(),
        b"PATH=Z:\\;Z:\\SYSTEM;Z:\\BIN;Z:\\DOS;Z:\\4DOS;Z:\\DEBUG;Z:\\TEXTUTIL",
        b"PROMPT=$P$G",
        b"BLASTER=A220 I7 D1 H5 P330 T6",
        b"",     // list terminator
        b"\x01", // count of following strings
        b"S:\\READ.EXE",
        //b"UNT.COM\0",  // saw this in dosbox memory, but I think isn't needed
    ]
    .join(b"\0".as_slice());
    let environment_segment = 0x7ca; // from dosbox
    memory[segofs(environment_segment, 0)..][..environment.len()].copy_from_slice(&environment);

    let mut psp = PSP::new();
    psp.memory_top = 0x9fff; // from dosbox
    psp.environment = environment_segment;
    psp.set_args(command_line.unwrap_or(""));
    memory.write(exe.image_base, psp);

    state().psp_segment = (exe.image_base >> 4) as u16;

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
    let mut ctx = load(exe, None);
    start(&mut ctx, exe);
}

static STATE: LazyLock<SingleThreader<RefCell<State>>> =
    LazyLock::new(|| SingleThreader::new(RefCell::new(State::new())));

struct State {
    psp_segment: u16,
    pit: PIT,
    // IVT; TODO: this actually lives in in memory at 0000:0000, not sure if anything depends on that
    interrupt_handlers: [(u16, u16); 0x30],
    vga: Option<VGA>,
    files: Vec<u8>,
}

impl State {
    fn new() -> Self {
        let mut state = State {
            psp_segment: 0,
            pit: PIT::default(),
            interrupt_handlers: [(0, 0); 0x30],
            vga: None,
            // Initial files: stdin, stdout, stderr, stdaux, stdprn; file handles are indexes into this vector
            // TODO: this is the JFT, belongs in the PSP I guess.
            files: vec![0, 0, 0, 0, 0],
        };
        // cpu exception handler
        state.interrupt_handlers[0] = (0xf000, 0xca60); // from dosbox
        // TSR handler
        state.interrupt_handlers[0x2f] = (0xf000, 0xd220); // from dosbox
        state
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
        // write to stdout
        0x09 => {
            let addr = segofs(ctx.cpu.regs.get_ds(), ctx.cpu.regs.get_dx());
            let buf = &ctx.memory.bytes[addr as usize..];
            let end = buf.iter().position(|&c| c == b'$').unwrap();
            let buf = &buf[..end];
            use std::io::Write;
            std::io::stdout().lock().write(buf).unwrap();
            ctx.cpu.regs.set_al(b'$');
        }
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
        // get an access handle
        0x3d => {
            let _access = ctx.cpu.regs.get_al();
            let addr = segofs(ctx.cpu.regs.get_ds(), ctx.cpu.regs.get_dx());
            let name = ctx.memory.read_str(addr);
            let handle = if name == "BLASTER.DRV" {
                let mut state = state();
                let handle = state.files.len() as u8;
                let _ = state.files.push(0);
                Some(handle)
            } else {
                None
            };

            match handle {
                Some(h) => {
                    ctx.cpu.regs.set_ax(h as u16); // TODO: file handle
                    ctx.cpu.flags.remove(runtime::Flags::CF);
                }
                None => {
                    ctx.cpu.regs.set_ax(/* file not found */ 2);
                    ctx.cpu.flags.insert(runtime::Flags::CF);
                }
            }
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
        // set file's access point
        0x42 => {
            let origin = ctx.cpu.regs.get_al();
            let handle = ctx.cpu.regs.get_bx();
            let offset =
                (((ctx.cpu.regs.get_cx() as u32) << 16) | (ctx.cpu.regs.get_dx() as u32)) as i32;
            log::error!("TODO: seek file {handle} {origin} {offset}");

            ctx.cpu.flags.remove(runtime::Flags::CF); // no error
            let offset = 0x26a3u32;
            ctx.cpu.regs.set_dx((offset >> 16) as u16);
            ctx.cpu.regs.set_ax(offset as u16);
        }
        // file i/o
        0x44 => {
            let cmd = ctx.cpu.regs.get_al();
            match cmd {
                // get handle info
                0 => {
                    let handle = ctx.cpu.regs.get_bx();
                    log::warn!("TODO: dos file i/o get handle info handle={handle:x}");
                    ctx.cpu.flags.remove(runtime::Flags::CF); // no error
                    // dx: file attributes, see book for tables
                    // TODO: for now we hardcode responses
                    if handle == 4 {
                        ctx.cpu.regs.set_ax(0x80e0); // from dosbox
                        ctx.cpu.regs.set_dx(0x80e0); // from dosbox
                    } else {
                        ctx.cpu.regs.set_ax(0x80d3); // from dosbox
                        ctx.cpu.regs.set_dx(0x80d3); // from dosbox
                    }
                }
                _ => log::error!("TODO: dos file i/o cmd={cmd:x}"),
            }
        }
        // release memory block
        0x49 => {
            let seg = ctx.cpu.regs.es;
            log::warn!("TODO: release memory seg {seg:x}");
            ctx.cpu.flags.remove(runtime::Flags::CF); // no error
        }
        // resize memory block
        0x4a => {
            let size = ctx.cpu.regs.get_bx() << 4;
            let seg = ctx.cpu.regs.es;
            log::warn!("TODO: resize memory seg {seg:x} to {size:x}");

            ctx.cpu.flags.remove(runtime::Flags::CF); // no error
            // leave bx alone, indicating the requested amount was allocated
            // ctx.cpu.regs.set_bx(available);
            // TODO: dosbox sets this, but it's not clear why -- docs say it should return a success code.
            ctx.cpu.regs.set_ax(ctx.cpu.regs.es);
        }
        // error exit
        0x4c => {
            let code = ctx.cpu.regs.get_al();
            std::process::exit(code as i32);
        }
        // get psp segment
        0x51 => {
            ctx.cpu.regs.set_bx(state().psp_segment);
        }
        _ => log::error!("TODO: dos int 21h ({func:02x})"),
    }
}

fn int2f(ctx: &mut Context) {
    log::error!("TODO: int2f ax={:x}", ctx.cpu.regs.get_ax());
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
        0x2f => int2f(ctx),
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
