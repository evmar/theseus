#[repr(C)]
#[derive(
    zerocopy::FromBytes, zerocopy::IntoBytes, Debug, Clone, Copy, PartialEq, Eq, Default, Hash,
)]
#[cfg_attr(feature = "serde", derive(ts_rs::TS))]
#[cfg_attr(feature = "serde", ts(as = "String"))]
pub struct SegOfs {
    // note: field order matters; we use repr(C) and want little-endian
    pub ofs: u16,
    pub seg: u16,
}

impl SegOfs {
    pub const fn new(seg: u16, ofs: u16) -> SegOfs {
        SegOfs { seg, ofs }
    }

    pub const fn abs(&self) -> u32 {
        segofs(self.seg, self.ofs)
    }

    pub const fn with_ofs(&self, ofs: u16) -> SegOfs {
        SegOfs { seg: self.seg, ofs }
    }

    pub fn parse(val: &str) -> Result<SegOfs, String> {
        let Some((seg, ofs)) = val.split_once(':') else {
            return Err("invalid segofs".into());
        };
        let seg = u16::from_str_radix(seg, 16).map_err(|err| err.to_string())?;
        let ofs = u16::from_str_radix(ofs, 16).map_err(|err| err.to_string())?;
        Ok((seg, ofs).into())
    }

    pub fn is_null(&self) -> bool {
        self.seg == 0 && self.ofs == 0
    }
}

impl From<(u16, u16)> for SegOfs {
    fn from((seg, ofs): (u16, u16)) -> Self {
        SegOfs::new(seg, ofs)
    }
}

impl std::fmt::Display for SegOfs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{seg:04x}:{ofs:04x}", seg = self.seg, ofs = self.ofs)
    }
}

impl PartialOrd for SegOfs {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SegOfs {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO: should this just compare .abs()?
        // Do we want segofs with different segs to be possibly equal?
        match self.seg.cmp(&other.seg) {
            core::cmp::Ordering::Equal => self.ofs.cmp(&other.ofs),
            ord => ord,
        }
    }
}

/// Combine a seg:ofs address into a single flat u32 address.
pub const fn segofs(seg: u16, off: u16) -> u32 {
    ((seg as u32) << 4) + (off as u32)
}

#[cfg(feature = "serde")]
impl serde::Serialize for SegOfs {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.serialize_str(format!("{}", self).as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SegOfs {
    fn deserialize<D>(deserializer: D) -> Result<SegOfs, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SegOfsVisitor;

        impl<'de> serde::de::Visitor<'de> for SegOfsVisitor {
            type Value = SegOfs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("seg:ofs pair")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                SegOfs::parse(value).map_err(|err| E::custom(err))
            }
        }

        deserializer.deserialize_str(SegOfsVisitor)
    }
}
