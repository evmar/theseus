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

    /// additional address ranges to scan for code
    #[argh(option, from_str_fn(hex_range))]
    entry_points: Vec<std::ops::Range<u32>>,

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
    state.mem.mappings.alloc("null page".into(), 0x1000);

    let buf = std::fs::read(args.exe).unwrap();
    state.module = tc::load_pe(&mut state.mem, buf);

    state.init_imports();

    state.gather(tc::Gather {
        scan_immediates: args.scan_immediates,
        scan_memory: args.scan_memory,
        externs: args.externs,
        entry_points: [
            args.entry_point
                .into_iter()
                .map(tc::EntryPoint::Single)
                .collect::<Vec<_>>(),
            args.entry_points
                .into_iter()
                .map(tc::EntryPoint::Range)
                .collect(),
        ]
        .concat(),
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
