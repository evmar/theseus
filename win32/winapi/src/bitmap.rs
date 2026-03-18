//! The bitmap file/memory format and pixel buffers.

use crate::FromABIParam;
use zerocopy::FromBytes;

#[derive(Debug, Eq, PartialEq, win32_derive::ABIEnum)]
pub enum BI {
    RGB = 0,
    RLE8 = 1,
    RLE4 = 2,
    BITFIELDS = 3,
    JPEG = 4,
    PNG = 5,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BITMAPFILEHEADER {
    pub bfType: u16,
    pub bfSize: u32,
    pub bfReserved1: u16,
    pub bfReserved2: u16,
    pub bfOffBits: u32,
}

#[repr(C)]
#[derive(Debug, Clone, zerocopy::FromBytes)]
pub struct BITMAPCOREHEADER {
    pub bcSize: u32,
    pub bcWidth: u16,
    pub bcHeight: u16,
    pub bcPlanes: u16,
    pub bcBitCount: u16,
}
impl BITMAPCOREHEADER {
    pub fn stride(&self) -> usize {
        // Bitmap row stride is padded out to 4 bytes per row.
        ((((self.bcWidth * self.bcBitCount) as usize) + 31) & !31) >> 3
    }
}

#[repr(C)]
#[derive(Debug, Clone, zerocopy::FromBytes)]
pub struct BITMAPINFOHEADER {
    pub biSize: u32,
    pub biWidth: u32,
    pub biHeight: u32,
    pub biPlanes: u16,
    pub biBitCount: u16,
    pub biCompression: u32,
    pub biSizeImage: u32,
    pub biXPelsPerMeter: u32,
    pub biYPelsPerMeter: u32,
    pub biClrUsed: u32,
    pub biClrImportant: u32,
}

impl BITMAPINFOHEADER {
    pub fn width(&self) -> u32 {
        self.biWidth
    }

    pub fn stride(&self) -> usize {
        // Bitmap row stride is padded out to 4 bytes per row.
        (((self.biWidth as usize * self.biBitCount as usize) + 31) & !31) >> 3
    }

    pub fn height(&self) -> u32 {
        // Height is negative if top-down DIB.
        (self.biHeight as i32).abs() as u32
    }

    pub fn is_bottom_up(&self) -> bool {
        (self.biHeight as i32) > 0
    }

    pub fn compression(&self) -> BI {
        BI::from_abi(self.biCompression)
    }
}

/// Device dependent bitmap: the result of parsing a bitmap header.
pub struct DDB {
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub is_bottom_up: bool,
    pub bit_count: u8,
    pub compression: BI,
    pub palette: Box<[[u8; 4]]>,
    pub pixels: Box<[u8]>,
}

impl std::fmt::Debug for DDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BitmapInfo {{ {w}x{h} stride:{stride} {bpp}bpp bottom_up:{is_bottom_up} compression:{compression:?} palette:{entries} }}",
            w = self.width,
            h = self.height,
            stride = self.stride,
            bpp = self.bit_count,
            is_bottom_up = self.is_bottom_up,
            compression = self.compression,
            entries = self.palette.len(),
        )
    }
}

impl DDB {
    // TODO: when parsing a bitmap from memory it's unclear how much memory we'll need
    // to read until we've read the bitmap header.  This means the caller cannot know how
    // big of a slice to provide.

    pub fn parse(buf: &[u8]) -> Self {
        use zerocopy::FromBytes;
        let (header_size, _) = <u32>::read_from_prefix(buf).unwrap();
        match header_size {
            12 => {
                let (header, rest) = BITMAPCOREHEADER::read_from_prefix(buf).unwrap();
                Self::parseBMPv2(&header, rest)
            }
            40 => {
                let (header, rest) = BITMAPINFOHEADER::read_from_prefix(buf).unwrap();
                Self::parseBMPv3(&header, rest)
            }
            _ => unimplemented!("unimplemented bitmap header size {}", header_size),
        }
    }

    /// buf is the bytes following the header.
    fn parseBMPv2(header: &BITMAPCOREHEADER, buf: &[u8]) -> Self {
        let palette_len = if header.bcBitCount <= 8 {
            2usize.pow(header.bcBitCount as u32)
        } else {
            todo!();
        };
        let (palette, buf) = <[[u8; 3]]>::ref_from_prefix_with_elems(buf, palette_len).unwrap();
        let palette = palette
            .into_iter()
            .map(|&[r, g, b]| [0xff, r, g, b]) // RGBQUAD
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let pixels = buf[..(header.bcHeight as usize * header.stride())]
            .to_owned()
            .into_boxed_slice();

        DDB {
            width: header.bcWidth as u32,
            height: header.bcHeight as u32,
            stride: header.stride(),
            is_bottom_up: true, // MSDN: "BITMAPCOREHEADER bitmaps cannot be top-down bitmaps"
            bit_count: header.bcBitCount as u8,
            compression: BI::RGB,
            palette,
            pixels,
        }
    }

    /// buf is the bytes following the header.
    fn parseBMPv3(header: &BITMAPINFOHEADER, buf: &[u8]) -> Self {
        if header.biCompression != BI::RGB as u32 {
            todo!("compression {:?}", header.biCompression);
        }
        let palette_len = if header.biClrUsed > 0 {
            header.biClrUsed as usize
        } else if header.biBitCount <= 8 {
            2usize.pow(header.biBitCount as u32)
        } else {
            todo!()
        };

        let (palette, buf) = <[[u8; 4]]>::ref_from_prefix_with_elems(buf, palette_len).unwrap();
        let palette = palette
            .into_iter()
            .map(|&[b, g, r, _]| [0xff, r, g, b]) // RGBQUAD
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let pixels = buf[..(header.height() as usize * header.stride())]
            .to_owned()
            .into_boxed_slice();

        DDB {
            width: header.biWidth,
            height: header.height(),
            stride: header.stride(),
            is_bottom_up: header.is_bottom_up(),
            bit_count: header.biBitCount as u8,
            compression: BI::RGB,
            palette,
            pixels,
        }
    }

    pub fn read_pixels(&self, y: u32, x1: u32, x2: u32, dst: &mut [u8]) {
        let y = if self.is_bottom_up {
            self.height - y - 1
        } else {
            y
        };
        match self.bit_count {
            8 => {
                let src = &self.pixels[(y * self.width + x1) as usize..][..(x2 - x1) as usize];
                for i in 0..(x2 - x1) as usize {
                    let [a, r, g, b] = self.palette[src[i] as usize];
                    dst[i * 4] = b;
                    dst[i * 4 + 1] = g;
                    dst[i * 4 + 2] = r;
                    dst[i * 4 + 3] = a;
                }
            }
            _ => todo!(),
        }
    }
}
