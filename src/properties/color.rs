/// Representing a RGB color value
/// 
/// 4 bytes only
#[derive(Clone, Copy)]
pub struct ColorRgb {
    /// Color value
    color: u32
}

impl Default for ColorRgb {
    fn default() -> Self {
        Self {
            color: 0
        }
    }
}

impl From<(u8, u8, u8, u8)> for ColorRgb {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        let mut value: u32 = 0;

        value = (value << 8) | (r as u32);
        value = (value << 8) | (g as u32);
        value = (value << 8) | (b as u32);
        value = (value << 8) | (a as u32);

        Self {
            color: value
        }
    }
}

impl From<(u8, u8, u8)> for ColorRgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::from((r, g, b, 255))
    }
}

impl Into<(u8, u8, u8, u8)> for ColorRgb {
    fn into(self) -> (u8, u8, u8, u8) {
        let r = (self.color >> 24 & 0xff) as u8;
        let g = (self.color >> 16 & 0xff) as u8;
        let b = (self.color >> 8 & 0xff) as u8;
        let a = (self.color & 0xff) as u8;

        (r, g, b, a)
    }
}

impl From<&[u8]> for ColorRgb {
    fn from(bytes: &[u8]) -> Self {
        if bytes.len() == 4 {
            Self::from((
                bytes[0],
                bytes[1],
                bytes[2],
                bytes[3]
            ))
        } else if bytes.len() == 3 {
            Self::from((
                bytes[0],
                bytes[1],
                bytes[2]
            ))
        } else {
            Self::default()
        }
    }
}

impl AsRef<u32> for ColorRgb {
    fn as_ref(&self) -> &u32 {
        &self.color
    }
}
