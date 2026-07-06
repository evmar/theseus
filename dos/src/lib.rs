mod timer;
mod vga;

use std::{
    cell::{RefCell, RefMut},
    sync::LazyLock,
};

use host::SingleThreader;
use runtime::{CPU, Context, EXEData, Mappings, Memory, segofs};
use zerocopy::FromBytes;

use crate::{timer::PIT, vga::VGA};

/// DOSBox-X loads com files into this segment.
pub const DOSBOX_SEG: u16 = 0x813;

#[repr(C)]
#[derive(Clone, Copy, zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
struct IVTEntry(u16, u16);

impl From<IVTEntry> for (u16, u16) {
    fn from(IVTEntry(seg, ofs): IVTEntry) -> Self {
        (seg, ofs)
    }
}

fn ivt(mem: &mut Memory) -> &mut [IVTEntry] {
    <[IVTEntry]>::mut_from_prefix_with_elems(&mut mem.bytes, 0x400)
        .unwrap()
        .0
}

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
#[derive(
    Debug, zerocopy::FromBytes, zerocopy::KnownLayout, zerocopy::IntoBytes, zerocopy::Immutable,
)]
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

    let ivt = ivt(&mut memory);
    // cpu exception handler
    ivt[0] = IVTEntry(0xf000, 0xca60); // from dosbox
    // TSR handler
    ivt[0x2f] = IVTEntry(0xf000, 0xd220); // from dosbox

    let mut state = state();
    state.psp_segment = (exe.image_base >> 4) as u16;

    // TODO: values copied from dosbox
    *state.program_mcb(&mut memory) = MCB {
        typ: b'M',
        owner: U16::new(0x813),
        size: U16::new(0x2cb1),
        reserved: [0; 3],
        owner_name: [0; 8],
    };

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

/// Open file.
#[derive(Default)]
struct File {
    buf: Vec<u8>,
    /// Current read/write offset.
    ofs: u32,
}

static STATE: LazyLock<SingleThreader<RefCell<State>>> =
    LazyLock::new(|| SingleThreader::new(RefCell::new(State::new())));

pub struct State {
    psp_segment: u16,
    pit: PIT,
    vga: Option<VGA>,
    pub read_file: Option<Box<dyn FnMut(&str) -> Option<Vec<u8>>>>,
    files: Vec<File>,
}

impl State {
    fn new() -> Self {
        let mut files: Vec<File> = vec![];
        // Initial files: stdin, stdout, stderr, stdaux, stdprn; file handles are indexes into this vector
        // TODO: the JFT belongs in the PSP I guess.
        files.resize_with(5, Default::default);
        State {
            psp_segment: 0,
            pit: PIT::default(),
            vga: None,
            read_file: None,
            files,
        }
    }

    fn read_file(&mut self, path: &str) -> Option<Vec<u8>> {
        let read_file = self.read_file.as_mut()?;
        read_file(path)
    }

    fn program_mcb<'a>(&self, mem: &'a mut Memory) -> &'a mut MCB {
        MCB::mut_from_bytes(&mut mem[segofs(self.psp_segment - 1, 0)..][..0x10]).unwrap()
    }
}

pub fn state() -> RefMut<'static, State> {
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
            ivt(&mut ctx.memory)[int as usize] = IVTEntry(seg, ofs);
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
            let IVTEntry(seg, ofs) = ivt(&mut ctx.memory)[int as usize];
            ctx.cpu.regs.set_es(seg);
            ctx.cpu.regs.set_bx(ofs);
        }
        // get an access handle
        0x3d => {
            let access = ctx.cpu.regs.get_al();
            if access != 0 {
                log::warn!("TODO: file access {access:x}");
            }
            let addr = segofs(ctx.cpu.regs.get_ds(), ctx.cpu.regs.get_dx());
            let name = ctx.memory.read_str(addr);
            let mut state = state();
            let Some(buf) = state.read_file(name) else {
                log::warn!("open {name:?}: not found");
                ctx.cpu.regs.set_ax(/* file not found */ 2);
                ctx.cpu.flags.insert(runtime::Flags::CF);
                return;
            };
            let handle = state.files.len() as u8;
            let _ = state.files.push(File { buf, ofs: 0 });
            ctx.cpu.regs.set_ax(handle as u16);
            ctx.cpu.flags.remove(runtime::Flags::CF);
        }
        // delete an access handle
        0x3e => {
            let handle = ctx.cpu.regs.get_bx();
            let mut state = state();
            let _ = &mut state.files[handle as usize];
            log::warn!("TODO: close file");
            ctx.cpu.regs.set_al(1); // docs say AX is clobbered, match dosbox for now
            ctx.cpu.flags.remove(runtime::Flags::CF); // no error
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

            let mut state = state();
            let file = &mut state.files[handle as usize];
            let offset = match origin {
                0 => offset,
                1 => file.ofs as i32 + offset,
                2 => file.buf.len() as i32 + offset,
                _ => panic!(),
            } as u32;

            ctx.cpu.flags.remove(runtime::Flags::CF); // no error
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
            let size = ctx.cpu.regs.get_bx(); // in paragraphs
            let seg = ctx.cpu.regs.es;

            let state = state();
            assert_eq!(seg, state.psp_segment);
            let mcb = state.program_mcb(&mut ctx.memory);
            mcb.size.set(size);

            ctx.cpu.flags.remove(runtime::Flags::CF); // no error
            // leave bx alone, indicating the requested amount was allocated
            // ctx.cpu.regs.set_bx(available);
            // TODO: dosbox sets this, but it's not clear why -- docs say it should return a success code.
            ctx.cpu.regs.set_ax(ctx.cpu.regs.es);
        }
        // load a program for execution
        0x4b => {
            let func = ctx.cpu.regs.get_al();
            let cmd = ctx
                .memory
                .read_str(segofs(ctx.cpu.regs.get_ds(), ctx.cpu.regs.get_dx()));
            let params_addr = segofs(ctx.cpu.regs.get_es(), ctx.cpu.regs.get_bx());

            match func {
                0 => todo!("load+run exe {cmd}"),
                1 => todo!("load exe {cmd}"),
                3 => {
                    // overlay load
                    let seg = ctx.memory.read::<u16>(params_addr);
                    let relo = ctx.memory.read::<u16>(params_addr + 2);

                    let Some(buf) = state().read_file(cmd) else {
                        panic!()
                    };
                    let header = exe::DOS::parse(&buf).unwrap();
                    let load_addr = segofs(seg, 0);
                    let data = &buf[header.image_offset()..];
                    log::info!("loading {cmd:?} at {seg:x}:0 size {:x}", buf.len());
                    ctx.memory[load_addr..][..data.len()].copy_from_slice(data);
                    log::info!("TODO: relocations {relo:x}");

                    ctx.cpu.flags.remove(runtime::Flags::CF); // no error
                    // on success, no register values are known; match dosbox here
                    ctx.cpu.regs.set_ax(0);
                    ctx.cpu.regs.set_dx(0);
                }
                _ => panic!("int21 4b invalid func"),
            }
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
        let handler = ivt(&mut ctx.memory)[8];
        self.pit.check_timer(ctx, handler.into());
        if let Some(vga) = &mut self.vga {
            vga.update_screen(ctx);
        }
    }
}

/// Handler for address 0; what happens if the entry point returns.
pub fn exit(_ctx: &mut Context) -> runtime::Cont {
    std::process::exit(0);
}
