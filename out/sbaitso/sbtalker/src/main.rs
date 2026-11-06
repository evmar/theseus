use sbaitso_sbtalker::SBTalker;

fn main() {
    let mut ctx = sbaitso_sbtalker::main();
    let ctx = &mut ctx;

    let sbtalker = SBTalker::new(ctx);

    let parsed = sbtalker.parse(ctx, "hello mastodon, i am dr. sbaitso.");
    println!("parsed: {}", parsed);

    let audio = sbtalker.speak(ctx, "hello mastodon, i am dr. sbaitso.");
    log::info!("audio buffer: {:x} bytes", audio.len());
    std::fs::write("sb.raw", audio).unwrap();
}
