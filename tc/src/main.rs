fn hex(val: &str) -> Result<u32, String> {
    if !val.starts_with("0x") {
        return Err("hex value must start with 0x".into());
    }
    u32::from_str_radix(&val[2..], 16).map_err(|err| err.to_string())
}

fn hex_range(val: &str) -> Result<std::ops::Range<u32>, String> {
    let (start, end) = val
        .split_once("..")
        .ok_or_else(|| "range must include '..'".to_string())?;
    let start = hex(start)?;
    let end = hex(end)?;
    Ok(start..end)
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

    /// additional addresses to create a block
    #[argh(option, from_str_fn(hex))]
    entry_point: Vec<u32>,

    /// additional addresses containing pointers to code
    #[argh(option, from_str_fn(hex_range))]
    jump_table: Vec<std::ops::Range<u32>>,

    /// additional address ranges to scan for code
    #[argh(option, from_str_fn(hex_range))]
    entry_points: Vec<std::ops::Range<u32>>,

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
    #[argh(option, long = "extern", from_str_fn(hex))]
    externs: Vec<u32>,
}

fn run() -> anyhow::Result<()> {
    logger::init();
    let args: Args = argh::from_env();

    let mut state = tc::State::default();

    if let Some(path) = &args.symbols_csv {
        state.load_symbols(std::fs::File::open(path)?)?;
    }

    let buf = std::fs::read(&args.exe).unwrap();
    if args.exe.to_ascii_lowercase().ends_with(".com") {
        state.module = tc::com::load_com(&mut state.mem, buf);
    } else if args.exe.to_ascii_lowercase().ends_with(".exe") {
        state.module = tc::load_exe(&mut state.mem, buf);
        state.init_imports();
    } else {
        anyhow::bail!("unexpected file extension");
    }

    let mut entry_points = vec![];
    for addr in args.entry_point {
        entry_points.push(tc::EntryPoint::Single(addr));
    }
    for range in args.jump_table {
        for addr in range.step_by(4) {
            let code = state.mem.read::<u32>(addr);
            log::info!("{addr:x} -> {code:x}");
            entry_points.push(tc::EntryPoint::Single(code));
        }
    }
    for range in args.entry_points {
        entry_points.push(tc::EntryPoint::Range(range));
    }

    state.gather(tc::Gather {
        scan_immediates: args.scan_immediates,
        scan_memory: args.scan_memory,
        externs: args.externs,
        entry_points,
        ..Default::default() // todo: entry_points, jump_tables?
    });

    state.generate(&args.out)
}

fn main() {
    if let Err(err) = run() {
        log::error!("error: {err}");
        std::process::exit(1);
    }
}
