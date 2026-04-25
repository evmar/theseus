fn hex(val: &str) -> Result<u32, String> {
    if !val.starts_with("0x") {
        return Err("hex value must start with 0x".into());
    }
    u32::from_str_radix(&val[2..], 16).map_err(|err| err.to_string())
}

#[derive(argh::FromArgs)]
/// theseus compiler
struct Args {
    /// scan data sections for code-looking pointers
    #[argh(switch)]
    scan: bool,

    /// scan immediates for code-looking pointers
    #[argh(switch)]
    scan_immediates: bool,

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
    state
        .mem
        .mappings
        .alloc("null page".into(), Some(0), 0x1000);

    let buf = std::fs::read(args.exe).unwrap();
    let module = tc::load_pe(&mut state.mem, buf);

    state.set_module(module);

    let mut traverse = tc::Traverse::new(&mut state, args.scan_immediates);
    for addr in args.externs {
        traverse.add_extern(addr);
    }
    if args.scan {
        traverse.scan_for_pointers();
    }
    traverse.run();

    state.generate(&args.out)
}

fn main() {
    if let Err(err) = run() {
        log::error!("error: {err}");
        std::process::exit(1);
    }
}
