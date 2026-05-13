use std::rc::Rc;

use runtime::Context;
use zerocopy::FromBytes;

use crate::{
    HANDLE,
    bitmap::BITMAPINFOHEADER,
    gdi32::{HDC, State, state},
};

pub use crate::bitmap::DDB;

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

    let dcs = state().dcs.borrow();
    let dc_src = dcs.get(hdcSrc).unwrap();
    let bmp_src = dc_src.bitmap.as_ref().unwrap();
    let BitmapType::DDB(ddb_src) = &bmp_src.typ else {
        todo!()
    };

    log::info!("src {:?}", bmp_src);

    let dc_dst = dcs.get(hdcDest).unwrap();
    let bmp_dst = dc_dst.bitmap.as_ref().unwrap();
    let BitmapType::DIB(dib_dst) = &bmp_dst.typ else {
        todo!()
    };
    let pixels_dst =
        &mut ctx.memory[dib_dst.pixels..][..(dib_dst.width * dib_dst.height * 4) as usize];

    assert_eq!(wDest, wSrc);
    assert_eq!(hDest, hSrc);

    let w = wDest as u32;
    for y in 0..hDest as u32 {
        ddb_src.read_pixels(
            ySrc as u32 + y,
            xSrc as u32,
            (xSrc + wSrc) as u32,
            &mut pixels_dst[(((yDest as u32 + y) * w + xDest as u32) * 4) as usize..]
                [..w as usize * 4],
        );
    }

    true
}

#[derive(Debug)]
pub struct DIB {
    pub width: u32,
    pub height: u32,
    /// pointer to pixel data
    pub pixels: u32,
}

impl DIB {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: 0,
        }
    }
}

#[derive(Debug)]
pub enum BitmapType {
    DDB(DDB),
    DIB(DIB),
}

#[derive(Debug)]
pub struct Bitmap {
    pub handle: HANDLE,
    pub typ: BitmapType,
}

impl State {
    pub fn new_bitmap(&self, bitmap: BitmapType) -> HBITMAP {
        let mut objects = self.objects.borrow_mut();
        let handle = objects.reserve();
        let bitmap = Rc::new(Bitmap {
            handle,
            typ: bitmap,
        });
        objects.set(handle, bitmap.clone());
        handle
    }
}

pub type HBITMAP = HANDLE;

#[win32_derive::dllexport]
pub fn CreateCompatibleBitmap(_ctx: &mut Context, hdc: HDC, cx: i32, cy: i32) -> HBITMAP {
    let dcs = state().dcs.borrow();
    let dc = dcs.get(hdc).unwrap();
    let bitmap = dc.bitmap.as_ref().unwrap();
    let bitmap = match &bitmap.typ {
        BitmapType::DDB(_) => todo!("ddb"),
        BitmapType::DIB(_) => BitmapType::DIB(DIB::new(cx as u32, cy as u32)),
    };
    state().new_bitmap(bitmap)
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
    log::info!("{:#x?}", header);
    assert_eq!(header.biCompression, 0); // BI_RGB
    assert_eq!(ColorUse, 0); // DIB_RGB_COLORS

    assert_eq!(header.biClrUsed, 0);
    let palette_size = 2usize.pow(header.biBitCount as u32);
    let _palette = &rest[..palette_size];
    let _pixels = &ctx.memory[lpvBits..];

    assert_eq!(StartScan, 0);

    cLines
}
