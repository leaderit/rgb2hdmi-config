use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
#[repr(u32)]
#[strum(ascii_case_insensitive)]
pub enum Rgb2hdmiClock {
    #[strum(serialize = "auto", serialize = "a" )]
    AUTO = 0,
    #[strum(serialize = "external", serialize = "e" )]
    EXTERNAL,
    #[strum(serialize = "z80pin6", serialize = "z" )]
    Z80PIN6
}

impl From<u32> for Rgb2hdmiClock {
    fn from( c: u32 ) -> Self {
        match c {
            0 => Self::AUTO,
            1 => Self::EXTERNAL,
            2 => Self::Z80PIN6,
            _ => Self::AUTO, 
        }
    }
}

impl Into<u32> for Rgb2hdmiClock {
    fn into( self ) -> u32 {
        match self {
            Self::AUTO => 0,
            Self::EXTERNAL => 1,
            Self::Z80PIN6 => 2,            
        }
    }
}

