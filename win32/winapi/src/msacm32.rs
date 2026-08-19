//! Audio Compression Manager. Games probe it to decide whether sound is
//! available; we report a working ACM with no codecs installed, which is all
//! PCM playback needs.

use runtime::Context;

const MMSYSERR_NOERROR: u32 = 0;
const MMSYSERR_NOTSUPPORTED: u32 = 8;

#[derive(Debug, PartialEq, Eq, win32_derive::ABIEnum)]
pub enum ACM_METRIC {
    COUNT_DRIVERS = 1,
    COUNT_CODECS = 2,
    COUNT_CONVERTERS = 3,
    COUNT_FILTERS = 4,
    COUNT_DISABLED = 5,
    MAX_SIZE_FORMAT = 50,
    MAX_SIZE_FILTER = 51,
}

#[win32_derive::dllexport]
pub fn acmMetrics(
    ctx: &mut Context,
    _hao: u32,
    uMetric: Result<ACM_METRIC, u32>,
    pMetric: u32,
) -> u32 {
    let value = match uMetric {
        // The largest WAVEFORMATEX any driver might describe. Callers size a
        // format buffer with this, so it has to cover WAVEFORMATEX plus the
        // extra bytes a compressed format would carry.
        Ok(ACM_METRIC::MAX_SIZE_FORMAT) | Ok(ACM_METRIC::MAX_SIZE_FILTER) => 64,
        // No codecs: PCM needs no conversion.
        Ok(ACM_METRIC::COUNT_DRIVERS)
        | Ok(ACM_METRIC::COUNT_CODECS)
        | Ok(ACM_METRIC::COUNT_CONVERTERS)
        | Ok(ACM_METRIC::COUNT_FILTERS)
        | Ok(ACM_METRIC::COUNT_DISABLED) => 0,
        Err(metric) => {
            log::warn!("acmMetrics: unhandled metric {metric}");
            return MMSYSERR_NOTSUPPORTED;
        }
    };
    if pMetric != 0 {
        ctx.memory.write::<u32>(pMetric, value);
    }
    MMSYSERR_NOERROR
}
