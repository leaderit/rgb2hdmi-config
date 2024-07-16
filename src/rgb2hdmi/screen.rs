use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
#[repr(u32)]
#[strum(ascii_case_insensitive)]
pub enum Rgb2hdmiScreen {
    #[strum(serialize = "Normal", serialize = "n" )]
    Normal = 0,
    #[strum(serialize = "Wide", serialize = "w" )]
    Wide,
}

impl From<u32> for Rgb2hdmiScreen {
    fn from( c: u32 ) -> Self {
        match c {
            0 => Self::Normal,
            1 => Self::Wide,
            _ => Self::Normal,
        }
    }
}

impl Into<u32> for Rgb2hdmiScreen {
    fn into( self ) -> u32 {
        match self {
            Self::Normal => 0,
            Self::Wide => 1,
        }
    }
}

