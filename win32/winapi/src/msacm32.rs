use runtime::Context;

#[win32_derive::dllexport]
pub fn acmMetrics(_ctx: &mut Context, _hao: u32, _uMetric: u32, _pMetric: u32) -> u32 {
    crate::stub!(1) // MMSYSERR_ERROR: no codecs available
}
