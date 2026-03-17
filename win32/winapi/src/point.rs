#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

impl POINT {
    pub fn add(&self, delta: POINT) -> POINT {
        POINT {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }

    pub fn sub(&self, delta: POINT) -> POINT {
        POINT {
            x: self.x - delta.x,
            y: self.y - delta.y,
        }
    }

    pub fn mul(&self, o: POINT) -> POINT {
        POINT {
            x: self.x * o.x,
            y: self.y * o.y,
        }
    }

    pub fn div(&self, o: POINT) -> POINT {
        POINT {
            x: self.x / o.x,
            y: self.y / o.y,
        }
    }
}
