//! Audio Compression Manager. Games probe it to decide whether sound is
//! available; we report a working ACM with no codecs installed, which is all
//! PCM playback needs.

use runtime::Context;

const MMSYSERR_NOERROR: u32 = 0;
const MMSYSERR_NOTSUPPORTED: u32 = 8;

const ACM_METRIC_COUNT_DRIVERS: u32 = 1;
const ACM_METRIC_COUNT_CODECS: u32 = 2;
const ACM_METRIC_COUNT_CONVERTERS: u32 = 3;
const ACM_METRIC_COUNT_FILTERS: u32 = 4;
const ACM_METRIC_COUNT_DISABLED: u32 = 5;
const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50;
const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51;

#[win32_derive::dllexport]
pub fn acmMetrics(ctx: &mut Context, _hao: u32, uMetric: u32, pMetric: u32) -> u32 {
    let value = match uMetric {
        // The largest WAVEFORMATEX any driver might describe. Callers size a
        // format buffer with this, so it has to cover WAVEFORMATEX plus the
        // extra bytes a compressed format would carry.
        ACM_METRIC_MAX_SIZE_FORMAT => 64,
        ACM_METRIC_MAX_SIZE_FILTER => 64,
        // No codecs: PCM needs no conversion.
        ACM_METRIC_COUNT_DRIVERS
        | ACM_METRIC_COUNT_CODECS
        | ACM_METRIC_COUNT_CONVERTERS
        | ACM_METRIC_COUNT_FILTERS
        | ACM_METRIC_COUNT_DISABLED => 0,
        _ => {
            log::warn!("acmMetrics: unhandled metric {uMetric}");
            return MMSYSERR_NOTSUPPORTED;
        }
    };
    if pMetric != 0 {
        ctx.memory.write::<u32>(pMetric, value);
    }
    MMSYSERR_NOERROR
}
