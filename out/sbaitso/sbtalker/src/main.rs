mod generated;

fn main() {
    let mut ctx = dos::load(&generated::EXEDATA, Some(" /dBLASTER"));
    dos::start(&mut ctx, &generated::EXEDATA);
}
