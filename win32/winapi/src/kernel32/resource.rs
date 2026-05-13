use runtime::Context;

use crate::kernel32::{self, HMODULE, State};

pub type HRSRC = u32;
pub type HGLOBAL = u32;

fn is_intresource(x: u32) -> bool {
    x >> 16 == 0
}

impl State {
    pub fn find_resource<'ctx>(
        &self,
        ctx: &'ctx Context,
        typ: pe::ResourceName,
        name: pe::ResourceName,
    ) -> Option<&'ctx [u8]> {
        let section = &ctx.memory[self.resources.clone()];
        let span = pe::find_resource(section, typ, name)?;
        let image_base = self.image_base;
        Some(&ctx.memory[image_base + span.start..image_base + span.end])
    }
}

#[win32_derive::dllexport]
pub fn FindResourceW(
    ctx: &mut Context,
    _hModule: HMODULE,
    lpName: u32, /* WSTR */
    lpType: u32, /* WSTR */
) -> HRSRC {
    let name = if is_intresource(lpName) {
        pe::ResourceName::Id(lpName)
    } else {
        pe::ResourceName::Name(&ctx.memory.read_wstr(lpName))
    };
    let typ = if is_intresource(lpType) {
        pe::ResourceName::Id(lpType)
    } else {
        pe::ResourceName::Name(&ctx.memory.read_wstr(lpType))
    };
    let buf = kernel32::lock().find_resource(ctx, typ, name).unwrap();
    unsafe { buf.as_ptr().byte_offset_from(ctx.memory.as_ptr()) as u32 }
}

#[win32_derive::dllexport]
pub fn LoadResource(_ctx: &mut Context, _hModule: HMODULE, hResInfo: HRSRC) -> HGLOBAL {
    hResInfo
}

#[win32_derive::dllexport]
pub fn LockResource(_ctx: &mut Context, hResData: HGLOBAL) -> u32 {
    hResData
}
