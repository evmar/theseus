use std::sync::Arc;

use runtime::Context;

use crate::{
    Ptr,
    gdi32::{self, Bitmap, COLORREF, HDC, HGDIOBJ, HPEN},
};

#[derive(Debug, Clone)]
pub struct Brush(pub COLORREF);

#[derive(Debug, Clone)]
pub struct Pen(pub COLORREF);

#[win32_derive::dllexport]
pub fn CreatePen(
    _ctx: &mut Context,
    iStyle: u32, /* PEN_STYLE */
    cWidth: i32,
    color: COLORREF,
) -> HPEN {
    assert_eq!(iStyle, 0); // PS_SOLID
    assert_eq!(cWidth, 1);
    let pen = Pen(color);
    gdi32::lock().objects.add(Object::Pen(pen))
}

pub enum Object {
    Bitmap(Arc<Bitmap>),
    Brush(Brush),
    Pen(Pen),
}

impl Object {
    pub fn unwrap_brush(&self) -> Brush {
        let Object::Brush(brush) = self else { panic!() };
        brush.clone()
    }
}

#[repr(C)]
#[derive(zerocopy::Immutable, zerocopy::IntoBytes)]
pub struct BITMAP {
    bmType: u32,
    bmWidth: u32,
    bmHeight: u32,
    bmWidthBytes: u32,
    bmPlanes: u16,
    bmBitsPixel: u16,
    bmBits: u32,
}

#[win32_derive::dllexport]
pub fn GetObjectA(ctx: &mut Context, handle: HGDIOBJ, size: u32, lpOut: Ptr<BITMAP>) -> u32 {
    let state = gdi32::lock();
    let object = state.objects.get(handle).unwrap();
    let Object::Bitmap(bitmap) = object else {
        panic!();
    };
    assert!(size == std::mem::size_of::<BITMAP>() as u32);
    let fields = BITMAP {
        bmType: 0,
        bmWidth: bitmap.width,
        bmHeight: bitmap.height,
        bmWidthBytes: 0,
        bmPlanes: 0,
        bmBitsPixel: bitmap.bit_count as u16,
        bmBits: 0,
    };
    lpOut.write(&mut ctx.memory, fields).unwrap();
    size
}

#[derive(Debug, win32_derive::ABIEnum)]
pub enum GetStockObjectArg {
    WHITE_BRUSH = 0,
    LTGRAY_BRUSH = 1,
    GRAY_BRUSH = 2,
    DKGRAY_BRUSH = 3,
    BLACK_BRUSH = 4,
    NULL_BRUSH = 5,
    WHITE_PEN = 6,
    BLACK_PEN = 7,
    NULL_PEN = 8,
    OEM_FIXED_FONT = 10,
    ANSI_FIXED_FONT = 11,
    ANSI_VAR_FONT = 12,
    SYSTEM_FONT = 13,
    DEVICE_DEFAULT_FONT = 14,
    DEFAULT_PALETTE = 15,
    SYSTEM_FIXED_FONT = 16,
    DEFAULT_GUI_FONT = 17,
    DC_BRUSH = 18,
    DC_PEN = 19,
}

#[win32_derive::dllexport]
pub fn GetStockObject(_ctx: &mut Context, i: GetStockObjectArg) -> HGDIOBJ {
    use GetStockObjectArg::*;
    let object = match i {
        LTGRAY_BRUSH => Object::Brush(Brush(COLORREF::from_rgb(0xc0, 0xc0, 0xc0))),
        BLACK_BRUSH => Object::Brush(Brush(COLORREF::from_rgb(0x00, 0x00, 0x00))),
        _ => todo!("{:?}", i),
    };
    gdi32::lock().objects.add(object)
}

#[win32_derive::dllexport]
pub fn SelectObject(_ctx: &mut Context, hdc: HDC, h: HGDIOBJ) -> HGDIOBJ {
    if h.is_null_or_invalid() {
        log::warn!("SelectObject: ignoring null select, likely from a prior stub");
        return HGDIOBJ::null();
    }
    let state = &mut *gdi32::lock();
    let dc = state.dcs.get_mut(hdc).unwrap();
    let object = state.objects.get(h).unwrap();
    match object {
        Object::Bitmap(bitmap) => {
            let prev = dc.bitmap.0;
            dc.bitmap = (h, bitmap.clone());
            prev
        }
        Object::Pen(pen) => {
            let prev = dc.pen.0;
            dc.pen = (h, pen.clone());
            prev
        }
        _ => todo!(),
    }
}
