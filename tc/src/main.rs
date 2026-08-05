use tc::{AddrInfo, IP, Module};

fn parse_hex(val: &str) -> Result<u32, String> {
    u32::from_str_radix(val, 16).map_err(|err| err.to_string())
}

fn parse_ip(val: &str) -> Result<IP, String> {
    if let Some((seg, ofs)) = val.split_once(':') {
        let seg = u16::from_str_radix(seg, 16).map_err(|err| err.to_string())?;
        let ofs = u16::from_str_radix(ofs, 16).map_err(|err| err.to_string())?;
        Ok(IP::Seg((seg, ofs).into()))
    } else {
        let addr = u32::from_str_radix(val, 16).map_err(|err| err.to_string())?;
        Ok(IP::Flat(addr))
    }
}

fn parse_ip_range(val: &str) -> Result<std::ops::Range<IP>, String> {
    let (start, end) = val
        .split_once("..")
        .ok_or_else(|| "range must include '..'".to_string())?;
    let start = parse_ip(start)?;
    let end = parse_ip(end)?;
    Ok(start..end)
}

fn parse_extern(val: &str) -> Result<(u32, Option<String>), String> {
    let (val, name) = match val.split_once('=') {
        None => (val, None),
        Some((val, name)) => (val, Some(name.into())),
    };
    Ok((parse_hex(val)?, name))
}

#[derive(argh::FromArgs)]
/// theseus compiler
struct Args {
    /// scan data sections for code-looking pointers
    #[argh(switch)]
    scan_memory: bool,

    /// scan immediates for code-looking pointers
    #[argh(switch)]
    scan_immediates: bool,

    /// scan unexplored code ranges for function prologues
    #[argh(switch)]
    scan_prologues: bool,

    /// additional addresses to create a block
    #[argh(option, from_str_fn(parse_ip))]
    entry_point: Vec<IP>,

    /// file with additional entry point addresses, one hex address per line
    #[argh(option)]
    entry_points_file: Option<String>,

    /// additional addresses containing pointers to code
    #[argh(option, from_str_fn(parse_ip_range))]
    jump_table: Vec<std::ops::Range<IP>>,

    /// additional address ranges to scan for code
    #[argh(option, from_str_fn(parse_ip_range))]
    entry_points: Vec<std::ops::Range<IP>>,

    /// ghidra symbols csv
    #[argh(option)]
    symbols_csv: Option<String>,

    /// path to input executable
    #[argh(option)]
    exe: String,

    /// path to output directory
    #[argh(option)]
    out: String,

    /// blocks written by hand
    #[argh(option, long = "extern", from_str_fn(parse_extern))]
    externs: Vec<(u32, Option<String>)>,

    /// emit output that traces each basic block as it's executed
    #[argh(switch)]
    trace: bool,
}

fn run() -> anyhow::Result<()> {
    logger::init();
    let args: Args = argh::from_env();

    let mut state = tc::State::default();

    if let Some(path) = &args.symbols_csv {
        state.load_symbols(std::fs::File::open(path)?)?;
    }
    for (addr, name) in args.externs {
        let name = name.unwrap_or_else(|| format!("x{:x}", addr));
        state.addr_info.insert(
            addr,
            AddrInfo {
                name: format!("crate::externs::{}", name),
                is_extern: true,
            },
        );
    }

    let buf = std::fs::read(&args.exe).unwrap();
    if args.exe.to_ascii_lowercase().ends_with(".com") {
        state.module = Module::DOS(tc::com::load_com(&mut state.mem, buf));
    } else if args.exe.to_ascii_lowercase().ends_with(".exe") {
        state.module = tc::exe::load_exe(&mut state.mem, buf);
        state.init_imports();
    } else {
        anyhow::bail!("unexpected file extension");
    }
    state.init_system_hooks();

    let mut entry_points = vec![];
    for ip in args.entry_point {
        if matches!(ip, IP::Seg(_)) != state.module.segment_addressed() {
            anyhow::bail!("--entry-point {ip} must be ip");
        }
        entry_points.push(tc::EntryPoint::Single(ip));
    }
    if let Some(path) = &args.entry_points_file {
        for line in std::fs::read_to_string(path)?.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let ip = parse_ip(line)
                .map_err(|err| anyhow::anyhow!("{path}: bad address {line:?}: {err}"))?;
            entry_points.push(tc::EntryPoint::Single(ip));
        }
    }
    for range in args.jump_table {
        let mut src = range.start; // TODO
        while src <= range.end {
            let next: IP;
            let dst = if state.module.segment_addressed() {
                let IP::Seg(addr) = src else {
                    anyhow::bail!("--jump-table {src} must be seg:ofs");
                };
                next = IP::Seg((addr.seg, addr.ofs + 2).into());
                let val = state.mem.read::<u16>(src.to_addr());
                IP::Seg((addr.seg, val).into())
            } else {
                let IP::Flat(addr) = src else {
                    anyhow::bail!("--jump-table {src} must be flat address");
                };
                if addr == 0 {
                    continue;
                }
                next = IP::Flat(addr + 4);
                let val = state.mem.read::<u32>(src.to_addr());
                IP::Flat(val)
            };
            log::info!("jump table {src} -> {dst}");
            entry_points.push(tc::EntryPoint::Single(dst));
            src = next;
        }
    }
    for range in args.entry_points {
        if matches!(range.start, IP::Seg(_)) != state.module.segment_addressed() {
            anyhow::bail!("--entry-points {ip} must be ip", ip = range.start);
        }
        if matches!(range.end, IP::Seg(_)) != state.module.segment_addressed() {
            anyhow::bail!("--entry-points {ip} must be ip", ip = range.end);
        }
        entry_points.push(tc::EntryPoint::Range(range));
    }

    state.gather(tc::Gather {
        scan_immediates: args.scan_immediates,
        scan_memory: args.scan_memory,
        scan_prologues: args.scan_prologues,
        entry_points,
    });

    state.generate(args.trace, &args.out)
}

fn main() {
    if let Err(err) = run() {
        log::error!("error: {err}");
        std::process::exit(1);
    }
}
