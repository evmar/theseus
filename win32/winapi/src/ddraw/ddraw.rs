use std::{cell::RefCell, rc::Rc};

use runtime::*;

use super::types::*;
use crate::{
    RECT,
    ddraw::{GUID, ddraw1, ddraw7, state},
    kernel32,
    user32::{self, HWND},
};

pub struct DirectDraw {
    pub addr: u32,
    pub bytes_per_pixel: u32,
    pub window: Option<Rc<RefCell<user32::Window>>>,
}

impl DirectDraw {
    pub fn set_cooperative_level(&mut self, _hwnd: HWND, _flags: u32) {
        let window = user32::state().window.borrow().as_ref().unwrap().clone();
        self.window = Some(window);
    }
}

struct SurfaceParams {
    is_primary: bool,
    width: u32,
    height: u32,
    bytes_per_pixel: u32,
}

impl DirectDraw {
    pub fn create_surface(
        &mut self,
        desc: &DDSURFACEDESC2,
        new_pointer: &mut dyn FnMut() -> u32,
    ) -> Rc<RefCell<Surface>> {
        let is_primary = desc.dwFlags.contains(DDSD::CAPS)
            && desc.ddsCaps.dwCaps.contains(DDSCAPS::PRIMARYSURFACE);

        let window = self.window.as_ref().unwrap().borrow();
        let width = if desc.dwFlags.contains(DDSD::WIDTH) {
            desc.dwWidth
        } else {
            window.width
        };
        let height = if desc.dwFlags.contains(DDSD::HEIGHT) {
            desc.dwHeight
        } else {
            window.height
        };
        drop(window);

        // An offscreen surface takes the display mode's format unless the app
        // asks for a specific one, which is what lets a palettized game blit
        // between its buffers without conversion.
        let bytes_per_pixel = if desc.dwFlags.contains(DDSD::PIXELFORMAT) {
            let bits = desc.ddpfPixelFormat.dwRGBBitCount;
            if bits == 0 {
                self.bytes_per_pixel
            } else {
                bits.div_ceil(8)
            }
        } else {
            self.bytes_per_pixel
        };

        let surface = self.create_one_surface(
            new_pointer(),
            &SurfaceParams {
                is_primary,
                width,
                height,
                bytes_per_pixel,
            },
        );

        if desc.dwFlags.contains(DDSD::CKSRCBLT) {
            surface.borrow_mut().src_color_key = Some(ColorKey {
                low: desc.ddckCKSrcBlt.dwColorSpaceLowValue,
                high: desc.ddckCKSrcBlt.dwColorSpaceHighValue,
            });
        }
        if desc.dwFlags.contains(DDSD::CKDESTBLT) {
            surface.borrow_mut().dst_color_key = Some(ColorKey {
                low: desc.ddckCKDestBlt.dwColorSpaceLowValue,
                high: desc.ddckCKDestBlt.dwColorSpaceHighValue,
            });
        }

        if let Some(count) = desc.back_buffer_count() {
            assert_eq!(count, 1);
            let back = self.create_one_surface(
                new_pointer(),
                &SurfaceParams {
                    is_primary: false,
                    width,
                    height,
                    bytes_per_pixel,
                },
            );
            back.borrow_mut().primary.replace(surface.clone());
            surface.borrow_mut().attached.replace(back);
        }

        surface
    }

    fn create_one_surface(&mut self, addr: u32, params: &SurfaceParams) -> Rc<RefCell<Surface>> {
        let window = self.window.as_ref().unwrap();
        let target = if params.is_primary {
            Target::Window(window.clone())
        } else {
            let texture = window
                .borrow_mut()
                .host
                .create_surface(params.width, params.height);
            Target::Texture(texture)
        };

        let surf = Rc::new(RefCell::new(Surface {
            addr,
            refs: 1,
            width: params.width,
            height: params.height,
            bytes_per_pixel: params.bytes_per_pixel,
            target,
            primary: Default::default(),
            attached: Default::default(),
            pixels: None,
            palette: None,
            src_color_key: None,
            dst_color_key: None,
        }));
        // TODO: move surf to ddraw
        state().surf.borrow_mut().insert(addr, surf.clone());
        surf
    }
}

pub enum Target {
    Window(Rc<RefCell<user32::Window>>),
    Texture(host::Surface),
}

/// A DDCOLORKEY: the inclusive range of pixel values a blit treats as
/// transparent.
#[derive(Copy, Clone, Debug)]
pub struct ColorKey {
    pub low: u32,
    pub high: u32,
}

impl ColorKey {
    pub fn matches(&self, pixel: u32) -> bool {
        (self.low..=self.high).contains(&pixel)
    }
}

pub struct Surface {
    pub addr: u32,
    /// COM reference count. An app that balances AddRef/Release expects the
    /// surface to outlive the matching Release, so this has to be real.
    pub refs: u32,
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,
    pub target: Target,

    // How does surface attachment actually work?
    // Docs are unclear, and wine's comments are also full of speculation and frustration, ha.
    /// Present on surfaces attached to Target::Window
    pub primary: Option<Rc<RefCell<Surface>>>,
    /// Present on Target::Window, TODO should be vec
    pub attached: Option<Rc<RefCell<Surface>>>,

    /// Address of pixel data.
    pub pixels: Option<u32>,

    pub palette: Option<Rc<RefCell<Palette>>>,

    /// Pixel values that read as transparent when this surface is the source
    /// of a blit — how sprites get their transparent background.
    pub src_color_key: Option<ColorKey>,
    /// Pixel values that may be overwritten when this surface is the
    /// destination of a blit.
    pub dst_color_key: Option<ColorKey>,
}

impl Surface {
    pub fn lock(&mut self, mem: &mut Memory) -> u32 {
        match self.pixels {
            Some(addr) => addr,
            None => {
                let size = self.width * self.height * self.bytes_per_pixel;
                let addr = kernel32::lock().process_heap.alloc(mem, size);
                // scribble on pixels so we can see it
                mem[addr..][..size as usize].fill(0x8F);
                self.pixels = Some(addr);
                addr
            }
        }
    }

    pub fn unlock(&mut self, mem: &mut Memory) {
        match self.target {
            // Writes to the primary surface go straight to the screen.
            Target::Window(_) => self.present(mem),
            Target::Texture(_) => self.update_texture(mem, &None),
        }
    }

    /// Convert this surface's pixels to the RGBA the host wants, using
    /// `palette` for palettized formats. Borrows the pixels directly when they
    /// are already RGBA. Returns None when there is nothing to show, e.g. an
    /// 8-bit surface with no palette attached yet.
    fn to_rgba<'a>(
        &self,
        mem: &'a Memory,
        palette: &Option<Rc<RefCell<Palette>>>,
    ) -> Option<std::borrow::Cow<'a, [u8]>> {
        let addr = self.pixels?;
        let size = self.width * self.height * self.bytes_per_pixel;
        let pixels = &mem[addr..][..size as usize];
        Some(match self.bytes_per_pixel {
            1 => {
                let palette = palette.as_ref()?;
                let entries = &palette.borrow().entries;
                let mut buf = Vec::with_capacity(pixels.len() * 4);
                for &p in pixels {
                    let entry = &entries[p as usize];
                    // ABGR8888 layout: R,G,B,A in byte order.
                    buf.push(entry.peRed);
                    buf.push(entry.peGreen);
                    buf.push(entry.peBlue);
                    buf.push(0);
                }
                buf.into()
            }
            2 => {
                // RGB565, the standard 16-bit display format.
                let mut buf = Vec::with_capacity(pixels.len() * 2);
                for pixel in pixels.chunks_exact(2) {
                    let pixel = u16::from_le_bytes([pixel[0], pixel[1]]);
                    let (r, g, b) = (pixel >> 11, (pixel >> 5) & 0x3f, pixel & 0x1f);
                    // Replicate the high bits into the low ones so full-scale
                    // values stay full-scale.
                    buf.push((r << 3 | r >> 2) as u8);
                    buf.push((g << 2 | g >> 4) as u8);
                    buf.push((b << 3 | b >> 2) as u8);
                    buf.push(0);
                }
                buf.into()
            }
            4 => pixels.into(),
            bpp => {
                log::warn!("unsupported surface format: {bpp} bytes per pixel");
                return None;
            }
        })
    }

    // App can write pixels to back buffer but attach palette to front buffer,
    // so take palette as an argument.
    fn update_texture(&mut self, mem: &mut Memory, palette: &Option<Rc<RefCell<Palette>>>) {
        let Some(pixels) = self.to_rgba(mem, palette) else {
            return;
        };
        let width = self.width;
        match &mut self.target {
            Target::Window(_) => unreachable!(),
            Target::Texture(texture) => {
                texture.set_pixels(&pixels, width * 4);
            }
        }
    }

    /// Present this surface's own pixel buffer to the window it targets, used
    /// when an app draws directly to the primary surface (via Lock or Blt)
    /// instead of flipping. No-op for non-primary surfaces.
    pub fn present(&mut self, mem: &mut Memory) {
        let Target::Window(window) = &self.target else {
            return;
        };
        // We have no texture of our own; borrow the back buffer's.
        let Some(back) = self.attached.clone() else {
            return;
        };
        let Some(pixels) = self.to_rgba(mem, &self.palette) else {
            return;
        };
        let mut back = back.borrow_mut();
        let Target::Texture(texture) = &mut back.target else {
            return;
        };
        texture.set_pixels(&pixels, self.width * 4);
        window.borrow_mut().host.render(texture);
    }

    pub fn flip(&mut self, mem: &mut Memory) {
        // "Flip can be called only for a surface that has the DDSCAPS_FLIP and DDSCAPS_FRONTBUFFER capabilities."
        let Target::Window(window) = &self.target else {
            unreachable!()
        };

        let mut back = self.attached.as_ref().unwrap().borrow_mut();
        if self.palette.is_some() {
            back.update_texture(mem, &self.palette);
        }
        let Target::Texture(texture) = &mut back.target else {
            unreachable!()
        };

        let mut window = window.borrow_mut();
        window.host.render(texture);
    }
}

pub struct Palette {
    pub entries: Vec<PALETTEENTRY>,
}

pub fn get_pixel_format() -> DDPIXELFORMAT {
    DDPIXELFORMAT {
        dwSize: std::mem::size_of::<DDPIXELFORMAT>() as u32,
        dwFlags: 0x00000040,
        dwFourCC: 0,
        dwRGBBitCount: 32,
        dwRBitMask: 0x0000_00FF,
        dwGBitMask: 0x0000_FF00,
        dwBBitMask: 0x00FF_0000,
        dwRGBAlphaBitMask: 0xFF00_0000,
    }
}

#[win32_derive::dllexport]
pub fn DirectDrawCreate(ctx: &mut Context, lpGUID: u32, lplpDD: u32, pUnkOuter: u32) -> DD {
    DirectDrawCreateEx(ctx, lpGUID, lplpDD, 0, pUnkOuter)
}

#[win32_derive::dllexport]
pub fn DirectDrawCreateEx(
    ctx: &mut Context,
    lpGuid: u32,
    lplpDD: u32,
    iid: u32,
    _pUnkOuter: u32,
) -> DD {
    assert!(lpGuid == 0);
    let iid = if iid == 0 {
        None
    } else {
        Some(ctx.memory.read::<GUID>(iid))
    };

    let mut kernel32 = kernel32::lock();
    let addr: u32 = match iid {
        None => ddraw1::IDirectDraw::new(ctx, &mut kernel32.process_heap),
        Some(ddraw7::IID_IDirectDraw7) => {
            ddraw7::IDirectDraw7::new(ctx, &mut kernel32.process_heap)
        }
        _ => panic!(),
    };

    let mut ddraw = state().ddraw.borrow_mut();
    assert!(ddraw.is_none());
    *ddraw = Some(DirectDraw {
        addr,
        bytes_per_pixel: 4,
        window: None,
    });

    ctx.memory.write(lplpDD, addr);
    DD::OK
}

pub fn read_rect(ctx: &Context, addr: u32) -> Option<RECT> {
    if addr == 0 {
        None
    } else {
        crate::Ptr::<RECT>::new(addr).read(&ctx.memory)
    }
}

/// Copy a rect between two surfaces (which may be the same one; the copy
/// stages through a temporary buffer).
///
/// With a `color_key`, source pixels inside its range are left alone in the
/// destination, which is how sprites get transparent backgrounds.
pub fn blit_copy(
    ctx: &mut Context,
    dst_ptr: u32,
    dst_rect: Option<RECT>,
    src_ptr: u32,
    src_rect: Option<RECT>,
    color_key: Option<ColorKey>,
) {
    let src_rc = state().surf.borrow_mut().get(&src_ptr).unwrap().clone();
    let dst_rc = state().surf.borrow_mut().get(&dst_ptr).unwrap().clone();

    let (rows, row_bytes, row_count, bpp) = {
        let mut src = src_rc.borrow_mut();
        let addr = src.lock(&mut ctx.memory);
        let bpp = src.bytes_per_pixel;
        let stride = src.width * bpp;
        let rect = src_rect
            .unwrap_or_else(|| RECT::from_size(src.width, src.height))
            .clip_to_size(src.width, src.height);
        let row_bytes = ((rect.right - rect.left).max(0) as u32 * bpp) as usize;
        let row_count = (rect.bottom - rect.top).max(0) as usize;
        let mut rows = Vec::with_capacity(row_bytes * row_count);
        for y in rect.top..rect.bottom {
            let start = addr + y as u32 * stride + rect.left as u32 * bpp;
            rows.extend_from_slice(&ctx.memory[start..][..row_bytes]);
        }
        (rows, row_bytes, row_count, bpp)
    };

    let mut dst = dst_rc.borrow_mut();
    if dst.bytes_per_pixel != bpp {
        log::warn!("blit between different pixel formats");
        return;
    }
    let addr = dst.lock(&mut ctx.memory);
    let stride = dst.width * bpp;
    let want = dst_rect.unwrap_or_else(|| RECT::from_size(dst.width, dst.height));
    let rect = want.clip_to_size(dst.width, dst.height);
    // Whatever the clip took off the top and left has to come off the
    // source as well, otherwise the image slides instead of being cropped.
    let skip_x = (rect.left - want.left).max(0) as usize * bpp as usize;
    let skip_y = (rect.top - want.top).max(0) as usize;
    // No stretching: copy 1:1, clipped to both rects.
    let copy_bytes = row_bytes
        .saturating_sub(skip_x)
        .min(((rect.right - rect.left).max(0) as u32 * bpp) as usize);
    let copy_rows = row_count
        .saturating_sub(skip_y)
        .min((rect.bottom - rect.top).max(0) as usize);
    for i in 0..copy_rows {
        let dst_start = addr + (rect.top + i as i32) as u32 * stride + rect.left as u32 * bpp;
        let row = &rows[(i + skip_y) * row_bytes + skip_x..][..copy_bytes];
        match color_key {
            None => ctx.memory[dst_start..][..copy_bytes].copy_from_slice(row),
            Some(key) => {
                for (x, pixel) in row.chunks_exact(bpp as usize).enumerate() {
                    let value = match bpp {
                        1 => pixel[0] as u32,
                        2 => u16::from_le_bytes(pixel.try_into().unwrap()) as u32,
                        4 => u32::from_le_bytes(pixel.try_into().unwrap()),
                        _ => {
                            log::warn!("colorkey blit at {bpp} bytes per pixel");
                            return;
                        }
                    };
                    if key.matches(value) {
                        continue;
                    }
                    let at = dst_start + x as u32 * bpp;
                    ctx.memory[at..][..bpp as usize].copy_from_slice(pixel);
                }
            }
        }
    }
    dst.present(&mut ctx.memory);
}

pub fn surface_src_color_key(surface: u32) -> Option<ColorKey> {
    let surfaces = state().surf.borrow();
    let key = surfaces.get(&surface)?.borrow().src_color_key;
    if key.is_none() {
        log::warn!("blit asked for a source color key, but none is set");
    }
    key
}

pub fn blt(
    ctx: &mut Context,
    this: u32,
    lpDstRect: u32,
    lpDDSrcSurface: u32,
    lpSrcRect: u32,
    dwFlags: u32,
    lpDDBLTFX: u32,
) -> DD {
    const DDBLT_COLORFILL: u32 = 0x0400;
    const DDBLT_KEYSRC: u32 = 0x8000;
    const DDBLT_KEYSRCOVERRIDE: u32 = 0x0001_0000;
    const DDBLT_WAIT: u32 = 0x0100_0000;
    const KNOWN: u32 = DDBLT_COLORFILL | DDBLT_KEYSRC | DDBLT_KEYSRCOVERRIDE | DDBLT_WAIT;
    if dwFlags & !KNOWN != 0 {
        log::warn!("Blt: ignoring flags {:#x}", dwFlags & !KNOWN);
    }

    let dst_rect = read_rect(ctx, lpDstRect);
    if dwFlags & DDBLT_COLORFILL != 0 {
        // DDBLTFX.dwFillColor is at offset 80.
        let color = ctx.memory.read::<u32>(lpDDBLTFX + 80);
        let dst_rc = state().surf.borrow_mut().get(&this).unwrap().clone();
        let mut dst = dst_rc.borrow_mut();
        let bpp = dst.bytes_per_pixel;
        let rect = dst_rect
            .unwrap_or_else(|| RECT::from_size(dst.width, dst.height))
            .clip_to_size(dst.width, dst.height);
        let addr = dst.lock(&mut ctx.memory);
        let stride = dst.width * bpp;
        for y in rect.top..rect.bottom {
            let start = addr + y as u32 * stride + rect.left as u32 * bpp;
            let width_bytes = ((rect.right - rect.left).max(0) as u32 * bpp) as usize;
            match bpp {
                1 => ctx.memory[start..][..width_bytes].fill(color as u8),
                4 => {
                    for x in 0..(rect.right - rect.left).max(0) as u32 {
                        ctx.memory.write::<u32>(start + x * 4, color);
                    }
                }
                _ => todo!("Blt colorfill bpp {bpp}"),
            }
        }
        dst.present(&mut ctx.memory);
        return DD::OK;
    }

    let color_key = if dwFlags & DDBLT_KEYSRCOVERRIDE != 0 {
        // DDBLTFX.ddckSrcColorkey, past the z-buffer and alpha fields.
        Some(ColorKey {
            low: ctx.memory.read::<u32>(lpDDBLTFX + 92),
            high: ctx.memory.read::<u32>(lpDDBLTFX + 96),
        })
    } else if dwFlags & DDBLT_KEYSRC != 0 {
        surface_src_color_key(lpDDSrcSurface)
    } else {
        None
    };

    let src_rect = read_rect(ctx, lpSrcRect);
    blit_copy(ctx, this, dst_rect, lpDDSrcSurface, src_rect, color_key);
    DD::OK
}

pub fn blt_fast(
    ctx: &mut Context,
    this: u32,
    dwX: u32,
    dwY: u32,
    lpDDSrcSurface: u32,
    lpSrcRect: u32,
    dwTrans: u32,
) -> DD {
    const DDBLTFAST_SRCCOLORKEY: u32 = 0x0001;
    const DDBLTFAST_DESTCOLORKEY: u32 = 0x0002;
    const DDBLTFAST_WAIT: u32 = 0x0010;
    const KNOWN: u32 = DDBLTFAST_SRCCOLORKEY | DDBLTFAST_DESTCOLORKEY | DDBLTFAST_WAIT;
    if dwTrans & !KNOWN != 0 {
        log::warn!("BltFast: ignoring flags {:#x}", dwTrans & !KNOWN);
    }
    if dwTrans & DDBLTFAST_DESTCOLORKEY != 0 {
        // Would need to test the destination pixel rather than the source;
        // no caller has needed it.
        log::warn!("BltFast: destination color key not supported");
    }
    let color_key = if dwTrans & DDBLTFAST_SRCCOLORKEY != 0 {
        surface_src_color_key(lpDDSrcSurface)
    } else {
        None
    };

    let src_rect = read_rect(ctx, lpSrcRect);
    let (w, h) = match &src_rect {
        Some(r) => ((r.right - r.left).max(0), (r.bottom - r.top).max(0)),
        None => {
            let src = state()
                .surf
                .borrow_mut()
                .get(&lpDDSrcSurface)
                .unwrap()
                .clone();
            let src = src.borrow();
            (src.width as i32, src.height as i32)
        }
    };
    let dst_rect = RECT {
        left: dwX as i32,
        top: dwY as i32,
        right: dwX as i32 + w,
        bottom: dwY as i32 + h,
    };
    blit_copy(
        ctx,
        this,
        Some(dst_rect),
        lpDDSrcSurface,
        src_rect,
        color_key,
    );
    DD::OK
}

pub fn set_color_key(ctx: &mut Context, this: u32, dwFlags: u32, lpDDColorKey: u32) -> DD {
    let key = if lpDDColorKey == 0 {
        None
    } else {
        // DDCOLORKEY: dwColorSpaceLowValue, dwColorSpaceHighValue.
        Some(ColorKey {
            low: ctx.memory.read::<u32>(lpDDColorKey),
            high: ctx.memory.read::<u32>(lpDDColorKey + 4),
        })
    };
    let surfaces = state().surf.borrow();
    let Some(surface) = surfaces.get(&this) else {
        return DD::ERR_GENERIC;
    };
    let mut surface = surface.borrow_mut();
    if dwFlags & (DDCKEY_SRCOVERLAY | DDCKEY_DESTOVERLAY) != 0 {
        log::warn!("SetColorKey: overlays are not supported");
    }
    if dwFlags & DDCKEY_DESTBLT != 0 {
        surface.dst_color_key = key;
    } else {
        surface.src_color_key = key;
    }
    DD::OK
}

pub fn get_color_key(ctx: &mut Context, this: u32, dwFlags: u32, lpDDColorKey: u32) -> DD {
    let key = {
        let surfaces = state().surf.borrow();
        let Some(surface) = surfaces.get(&this) else {
            return DD::ERR_GENERIC;
        };
        let surface = surface.borrow();
        if dwFlags & DDCKEY_DESTBLT != 0 {
            surface.dst_color_key
        } else {
            surface.src_color_key
        }
    };
    let Some(key) = key else {
        return DD::ERR_NOCOLORKEY;
    };
    ctx.memory.write::<u32>(lpDDColorKey, key.low);
    ctx.memory.write::<u32>(lpDDColorKey + 4, key.high);
    DD::OK
}
