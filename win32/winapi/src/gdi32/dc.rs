use std::sync::Arc;

use runtime::Context;
use zerocopy::IntoBytes;

use crate::{
    HANDLE, POINT,
    gdi32::{self, Bitmap, COLORREF, HBITMAP, Object, State},
    stub,
};

pub type HDC = HANDLE;

impl State {
    pub fn new_memory_dc(&mut self, bitmap: Bitmap) -> HDC {
        let bitmap = Arc::new(bitmap);
        let hbitmap = self.objects.add(Object::Bitmap(bitmap.clone()));
        let dc = DC {
            bitmap: (hbitmap, bitmap),
            pos: POINT::default(),
        };
        self.dcs.add(dc)
    }

    pub fn release_dc(&mut self, hdc: HDC) {
        self.dcs.remove(hdc);
    }
}

pub struct DC {
    /// Store the HBITMAP as well as the Bitmap itself so that when it is switched via SelectObject we can return it.
    pub bitmap: (HBITMAP, Arc<Bitmap>),
    pos: POINT,
}

impl DC {
    pub fn bitmap(&self) -> &Arc<Bitmap> {
        &self.bitmap.1
    }
}

#[win32_derive::dllexport]
pub fn CreateCompatibleDC(_ctx: &mut Context, hdc: HDC) -> HDC {
    // 1x1 monochrome bitmap
    let bitmap = Bitmap {
        width: 1,
        height: 1,
        is_bottom_up: false,
        bit_count: 1,
        palette: Box::new([[0; 4]]),
        pixels: 0,
    };
    let new_hdc = gdi32::lock().new_memory_dc(bitmap);
    if hdc.is_null() {
        // memory DC compatible with screen
        new_hdc
    } else {
        // memory DC compatible with hdc
        new_hdc
    }
}

#[win32_derive::dllexport]
pub fn DeleteDC(_ctx: &mut Context, _hdc: HDC) -> bool {
    stub!(true)
}

#[win32_derive::dllexport]
pub fn GetLayout(_ctx: &mut Context, _hdc: HDC) -> u32 {
    0 // LTR
}

#[win32_derive::dllexport]
pub fn SetROP2(_ctx: &mut Context, _hdc: HDC, _rop2: u32 /* R2_MODE */) -> i32 {
    stub!(0)
}

fn ascending(a: i32, b: i32) -> std::ops::RangeInclusive<i32> {
    if a < b { a..=b } else { b..=a }
}

#[win32_derive::dllexport]
pub fn LineTo(ctx: &mut Context, hdc: HDC, x: i32, y: i32) -> bool {
    let mut state = gdi32::lock();
    let dc = state.dcs.get_mut(hdc).unwrap();
    let bitmap = dc.bitmap();
    assert!(bitmap.is_simple());

    let pixels = bitmap.pixels_mut(&mut ctx.memory);
    if x == dc.pos.x {
        for y in ascending(dc.pos.y, y) {
            let i = ((y as u32 * bitmap.stride()) + (x as u32 * 4)) as usize;
            pixels[i] = 0;
            pixels[i + 1] = 0;
            pixels[i + 2] = 0;
            pixels[i + 3] = 0;
        }
    } else if y == dc.pos.y {
        for x in ascending(dc.pos.x, x) {
            let i = ((y as u32 * bitmap.stride()) + (x as u32 * 4)) as usize;
            pixels[i] = 0;
            pixels[i + 1] = 0;
            pixels[i + 2] = 0;
            pixels[i + 3] = 0;
        }
    } else {
        todo!(); // only axis-aligned supported for now
    }

    dc.pos = POINT { x, y };

    true
}

#[win32_derive::dllexport]
pub fn MoveToEx(ctx: &mut Context, hdc: HDC, x: i32, y: i32, lppt: u32 /* POINT */) -> bool {
    let mut state = gdi32::lock();
    let dc = state.dcs.get_mut(hdc).unwrap();
    dc.pos = POINT { x, y };
    if lppt != 0 {
        dc.pos.write_to_prefix(&mut ctx.memory[lppt..]).unwrap();
    }
    true
}

#[win32_derive::dllexport]
pub fn SetLayout(_ctx: &mut Context, _hdc: HDC, _l: u32 /* DC_LAYOUT */) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetPixel(_ctx: &mut Context, _hdc: HDC, _x: i32, _y: i32, _color: COLORREF) -> COLORREF {
    todo!()
}
