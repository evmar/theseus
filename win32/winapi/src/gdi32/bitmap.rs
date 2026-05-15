use std::sync::Arc;

use runtime::Context;

use crate::{
    HANDLE,
    gdi32::{self, HDC, Object, State},
    kernel32,
};

pub use crate::bitmap_format::Bitmap;

impl State {
    pub fn new_bitmap_handle(&mut self, bitmap: Bitmap) -> (HBITMAP, Arc<Bitmap>) {
        let bitmap = Arc::new(bitmap);
        let hbitmap = self.objects.add(Object::Bitmap(bitmap.clone()));
        (hbitmap, bitmap)
    }
}

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
    assert!(bmp_dst.is_simple());

    let [pixels_src, pixels_dst] = ctx
        .memory
        .bytes
        .get_disjoint_mut([bmp_src.pixels_range(), bmp_dst.pixels_range()])
        .unwrap();

    // stretching not implemented yet
    assert_eq!(wDest, wSrc);
    assert_eq!(hDest, hSrc);

    let xSrc = xSrc as u32;
    let ySrc = ySrc as u32;
    let xDst = xDest as u32;
    let yDst = yDest as u32;
    let wSrc = wSrc as u32;
    let hSrc = hSrc as u32;
    let wDst = wDest as u32;
    for y in 0..hDest as u32 {
        let dst = &mut pixels_dst[(((yDst + y) * bmp_dst.stride()) + (xDst * 4)) as usize..]
            [..wDst as usize * 4];
        let y_src = ySrc + y;
        bmp_src.read_pixels(
            &pixels_src,
            if bmp_src.is_bottom_up {
                hSrc - y_src - 1
            } else {
                y_src
            },
            xSrc,
            xSrc + wSrc,
            dst,
        );
    }

    true
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
    gdi32::lock().new_bitmap_handle(bitmap).0
}

#[win32_derive::dllexport]
pub fn SetDIBitsToDevice(
    ctx: &mut Context,
    hdc: HDC,
    xDest: u32,
    yDest: u32,
    w: u32,
    h: u32,
    xSrc: u32,
    ySrc: u32,
    StartScan: u32,
    cLines: u32,
    lpvBits: u32,
    lpbmi: u32,    /* BITMAPINFO */
    ColorUse: u32, /* DIB_USAGE */
) -> u32 {
    let (bmp_src, _) = Bitmap::parse(&ctx.memory[lpbmi..]);

    assert_eq!(StartScan, 0);
    assert_eq!(ColorUse, 0); // DIB_RGB_COLORS
    assert_eq!(cLines, h); // why would these ever be different?

    let state = gdi32::lock();
    let dc_dst = state.dcs.get(hdc).unwrap();
    let bmp_dst = &dc_dst.bitmap.1;
    assert!(bmp_dst.is_simple());

    let [pixels_src, pixels_dst] = ctx
        .memory
        .bytes
        .get_disjoint_mut([
            lpvBits as usize..(lpvBits + (h * bmp_src.stride())) as usize,
            bmp_dst.pixels_range(),
        ])
        .unwrap();

    // for i in (0..pixels_src.len()).step_by(bmp_src.stride() as usize) {
    //     log::info!("{:x?}", &pixels_src[i..][..bmp_src.stride() as usize]);
    // }

    for y in 0..h {
        let dst = &mut pixels_dst[((yDest + y) * bmp_dst.stride() + xDest * 4) as usize..];
        let y_src = ySrc + y;
        bmp_src.read_pixels(
            pixels_src,
            if bmp_src.is_bottom_up {
                h - y_src - 1
            } else {
                y_src
            },
            xSrc,
            xSrc + w,
            dst,
        );
    }

    cLines
}
