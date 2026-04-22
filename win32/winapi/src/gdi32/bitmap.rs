use std::rc::Rc;

use runtime::Context;

use crate::{
    HANDLE,
    bitmap::DDB,
    gdi32::{HDC, state},
};

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
    pub pixels: u32,
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

pub fn parse_bitmap(buf: &[u8]) -> Rc<Bitmap> {
    let ddb = DDB::parse(buf);
    println!("loaded bitmap {:#x?}", ddb);

    let mut objects = state().objects.borrow_mut();
    let handle = objects.reserve();
    let bitmap = Rc::new(Bitmap {
        handle,
        typ: BitmapType::DDB(ddb),
    });
    objects.set(handle, bitmap.clone());
    bitmap
}
