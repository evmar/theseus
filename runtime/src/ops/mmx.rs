trait Unpack<T> {
    fn unpack(self) -> T;
}

impl Unpack<[u32; 2]> for u64 {
    fn unpack(self) -> [u32; 2] {
        [(self >> 0) as u32, (self >> 32) as u32]
    }
}

impl Unpack<[i32; 2]> for u64 {
    fn unpack(self) -> [i32; 2] {
        [(self >> 0) as i32, (self >> 32) as i32]
    }
}

impl Unpack<[u16; 4]> for u64 {
    fn unpack(self) -> [u16; 4] {
        [
            (self >> 0) as u16,
            (self >> 16) as u16,
            (self >> 32) as u16,
            (self >> 48) as u16,
        ]
    }
}

impl Unpack<[i16; 4]> for u64 {
    fn unpack(self) -> [i16; 4] {
        let x: [u16; 4] = self.unpack();
        [x[0] as i16, x[1] as i16, x[2] as i16, x[3] as i16]
    }
}

impl Unpack<[i8; 8]> for u64 {
    fn unpack(self) -> [i8; 8] {
        self.to_le_bytes().map(|b| b as i8)
    }
}

impl Unpack<[u16; 2]> for u32 {
    fn unpack(self) -> [u16; 2] {
        [(self >> 0) as u16, (self >> 16) as u16]
    }
}

impl Unpack<[u8; 4]> for u32 {
    fn unpack(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}

trait Pack {
    type Target;
    fn pack(self) -> Self::Target;
}

impl Pack for [u32; 2] {
    type Target = u64;
    fn pack(self) -> u64 {
        (self[0] as u64) | ((self[1] as u64) << 32)
    }
}

impl Pack for [i16; 4] {
    type Target = u64;
    fn pack(self) -> u64 {
        self.map(|b| b as u16).pack()
    }
}

impl Pack for [u16; 4] {
    type Target = u64;
    fn pack(self) -> u64 {
        (self[0] as u64)
            | ((self[1] as u64) << 16)
            | ((self[2] as u64) << 32)
            | ((self[3] as u64) << 48)
    }
}

impl Pack for [u8; 8] {
    type Target = u64;
    fn pack(self) -> u64 {
        u64::from_le_bytes(self)
    }
}

impl Pack for [i8; 8] {
    type Target = u64;
    fn pack(self) -> u64 {
        self.map(|b| b as u8).pack()
    }
}

impl Pack for [u16; 2] {
    type Target = u32;
    fn pack(self) -> u32 {
        (self[0] as u32) | ((self[1] as u32) << 16)
    }
}

impl Pack for [u8; 4] {
    type Target = u32;
    fn pack(self) -> u32 {
        u32::from_le_bytes(self)
    }
}

pub fn paddsb(x: u64, y: u64) -> u64 {
    let x: [i8; 8] = x.unpack();
    let y: [i8; 8] = y.unpack();
    [
        x[0].saturating_add(y[0]),
        x[1].saturating_add(y[1]),
        x[2].saturating_add(y[2]),
        x[3].saturating_add(y[3]),
        x[4].saturating_add(y[4]),
        x[5].saturating_add(y[5]),
        x[6].saturating_add(y[6]),
        x[7].saturating_add(y[7]),
    ]
    .pack()
}

pub fn paddsw(x: u64, y: u64) -> u64 {
    let x: [i16; 4] = x.unpack();
    let y: [i16; 4] = y.unpack();
    [
        x[0].saturating_add(y[0]),
        x[1].saturating_add(y[1]),
        x[2].saturating_add(y[2]),
        x[3].saturating_add(y[3]),
    ]
    .pack()
}

pub fn punpcklbw(x: u32, y: u32) -> u64 {
    let x: [u8; 4] = x.unpack();
    let y: [u8; 4] = y.unpack();
    [x[0], y[0], x[1], y[1], x[2], y[2], x[3], y[3]].pack()
}

pub fn pmullw(x: u64, y: u64) -> u64 {
    let x: [u16; 4] = x.unpack();
    let y: [u16; 4] = y.unpack();
    [
        x[0].wrapping_mul(y[0]),
        x[1].wrapping_mul(y[1]),
        x[2].wrapping_mul(y[2]),
        x[3].wrapping_mul(y[3]),
    ]
    .pack()
}

pub fn psrlw(x: u64, y: u64) -> u64 {
    if y > 15 {
        return 0;
    }
    let x: [u16; 4] = x.unpack();
    [x[0] >> y, x[1] >> y, x[2] >> y, x[3] >> y].pack()
}

pub fn packuswb(x: u64, y: u64) -> u64 {
    fn saturate(x: i16) -> u8 {
        if x < 0 {
            0
        } else if x > 0xFF {
            0xFF
        } else {
            x as u8
        }
    }
    let x: [u16; 4] = x.unpack();
    let y: [u16; 4] = y.unpack();
    [
        saturate(x[0] as i16),
        saturate(x[1] as i16),
        saturate(x[2] as i16),
        saturate(x[3] as i16),
        saturate(y[0] as i16),
        saturate(y[1] as i16),
        saturate(y[2] as i16),
        saturate(y[3] as i16),
    ]
    .pack()
}
