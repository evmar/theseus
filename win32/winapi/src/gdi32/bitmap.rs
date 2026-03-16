use crate::{
    HANDLE,
    bitmap::BitmapInfo,
    gdi32::{HDC, state},
};

#[win32_derive::dllexport]
pub fn StretchBlt(
    _hdcDest: HDC,
    _xDest: i32,
    _yDest: i32,
    _wDest: i32,
    _hDest: i32,
    _hdcSrc: HDC,
    _xSrc: i32,
    _ySrc: i32,
    _wSrc: i32,
    _hSrc: i32,
    _rop: u32, /* ROP_CODE */
) -> bool {
    todo!()
}

pub struct Bitmap {
    pub handle: HANDLE,
    pub header: BitmapInfo,
}

pub fn parse_bitmap(buf: &[u8]) -> HANDLE {
    let header = BitmapInfo::parse(buf);
    println!("loaded bitmap {:#x?}", header);

    let mut objects = state().objects.borrow_mut();
    let handle = objects.reserve();
    let bitmap = Bitmap { handle, header };
    objects.set(handle, bitmap);
    handle
}
