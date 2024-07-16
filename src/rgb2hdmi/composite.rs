use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
#[repr(u32)]
#[strum(ascii_case_insensitive)]
pub enum Rgb2hdmiComposite {
    #[strum(serialize = "PAL", serialize = "p" )]
    PAL = 0,
    #[strum(serialize = "NTSC", serialize = "n" )]
    NTSC,
}

impl From<u32> for Rgb2hdmiComposite {
    fn from( c: u32 ) -> Self {
        match c {
            0 => Self::PAL,
            1 => Self::NTSC,
            _ => Self::PAL,
        }
    }
}

impl Into<u32> for Rgb2hdmiComposite {
    fn into( self ) -> u32 {
        match self {
            Self::PAL => 0,
            Self::NTSC => 1,
        }
    }
}
