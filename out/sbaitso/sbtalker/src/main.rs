mod generated;

fn main() {
    let drv = include_bytes!("../../../../scratch/dos/sbaitso/BLASTER.DRV").as_slice();
    let mut ctx = dos::load(&generated::EXEDATA, Some(" /dBLASTER"));
    dos::state().read_file = Some(Box::new(|path| {
        if path == "BLASTER.DRV" {
            Some(drv.to_owned())
        } else {
            None
        }
    }));
    dos::start(&mut ctx, &generated::EXEDATA);
}
