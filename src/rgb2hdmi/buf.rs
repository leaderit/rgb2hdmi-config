use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
#[repr(u32)]
#[strum(ascii_case_insensitive)]
pub enum Rgb2hdmiBuf {
    #[strum(serialize = "1X" )]
    Buf1x = 0,
    #[strum(serialize = "3X" )]
    Buf3x,
}

impl From<u32> for Rgb2hdmiBuf {
    fn from( c: u32 ) -> Self {
        match c {
            0 => Self::Buf1x,
            1 => Self::Buf3x,
            _ => Self::Buf1x,
        }
    }
}

impl Into<u32> for Rgb2hdmiBuf {
    fn into( self ) -> u32 {
        match self {
            Self::Buf1x => 0,
            Self::Buf3x => 1,
        }
    }
}

