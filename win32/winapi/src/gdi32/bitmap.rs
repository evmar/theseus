use std::sync::Arc;

use runtime::Context;
use zerocopy::FromBytes;

use crate::{
    HANDLE,
    bitmap_format::BITMAPINFOHEADER,
    gdi32::{self, HDC, State},
    kernel32,
};

pub use crate::bitmap_format::Bitmap;

#[win32_derive::dllexport]
pub fn BitBlt(
    ctx: &mut Context,
    hdc: HDC,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    hdcSrc: HDC,
    x1: i32,
    y1: i32,
    rop: u32, /* ROP_CODE */
) -> bool {
    StretchBlt(ctx, hdc, x, y, cx, cy, hdcSrc, x1, y1, cx, cy, rop)
}

#[win32_derive::dllexport]
pub fn StretchBlt(
    ctx: &mut Context,
    hdcDest: HDC,
    xDest: i32,
    yDest: i32,
    wDest: i32,
    hDest: i32,
    hdcSrc: HDC,
    xSrc: i32,
    ySrc: i32,
    wSrc: i32,
    hSrc: i32,
    rop: u32, /* ROP_CODE */
) -> bool {
    assert_eq!(rop, 0xcc0020);

    let state = gdi32::lock();
    let dc_src = state.dcs.get(hdcSrc).unwrap();
    let bmp_src = &dc_src.bitmap.1;

    let dc_dst = state.dcs.get(hdcDest).unwrap();
    let bmp_dst = &dc_dst.bitmap.1;

    let [pixels_src, pixels_dst] = ctx
        .memory
        .bytes
        .get_disjoint_mut([bmp_src.pixels_range(), bmp_dst.pixels_range()])
        .unwrap();

    assert_eq!(wDest, wSrc);
    assert_eq!(hDest, hSrc);

    let w = wDest as u32;
    for y in 0..hDest as u32 {
        bmp_src.read_pixels(
            &pixels_src,
            ySrc as u32 + y,
            xSrc as u32,
            (xSrc + wSrc) as u32,
            &mut pixels_dst[(((yDest as u32 + y) * w + xDest as u32) * 4) as usize..]
                [..w as usize * 4],
        );
    }

    true
}

impl State {
    pub fn new_hbitmap(&mut self, bitmap: Arc<Bitmap>) -> HBITMAP {
        let handle = self.objects.reserve();
        self.objects.set(handle, bitmap);
        handle
    }
}

pub type HBITMAP = HANDLE;

#[win32_derive::dllexport]
pub fn CreateCompatibleBitmap(ctx: &mut Context, _hdc: HDC, cx: i32, cy: i32) -> HBITMAP {
    let w = cx as u32;
    let h = cy as u32;
    let pixels = kernel32::lock()
        .process_heap
        .alloc(&mut ctx.memory, w * h * 4);
    let bitmap = Bitmap::new_simple(w, h, pixels);
    gdi32::lock().new_hbitmap(Arc::new(bitmap))
}

#[win32_derive::dllexport]
pub fn SetDIBitsToDevice(
    ctx: &mut Context,
    _hdc: HDC,
    _xDest: i32,
    _yDest: i32,
    _w: u32,
    _h: u32,
    _xSrc: i32,
    _ySrc: i32,
    StartScan: u32,
    cLines: u32,
    lpvBits: u32,
    lpbmi: u32,    /* BITMAPINFO */
    ColorUse: u32, /* DIB_USAGE */
) -> u32 {
    let (header, rest) = <BITMAPINFOHEADER>::read_from_prefix(&ctx.memory[lpbmi..]).unwrap();
    assert_eq!(header.biCompression, 0); // BI_RGB
    assert_eq!(ColorUse, 0); // DIB_RGB_COLORS

    assert_eq!(header.biClrUsed, 0);
    let palette_size = 2usize.pow(header.biBitCount as u32);
    let _palette = &rest[..palette_size];
    let _pixels = &ctx.memory[lpvBits..];

    assert_eq!(StartScan, 0);

    cLines
}
