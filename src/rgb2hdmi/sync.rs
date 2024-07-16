use strum_macros::Display;
use strum_macros::EnumString;

// #[derive(Debug, PartialEq)]
#[derive(Display, Debug, PartialEq, EnumString, Clone)]
#[repr(u32)]
#[strum(ascii_case_insensitive)]
pub enum Rgb2hdmiSync {
    #[strum(serialize = "VSHS" )]
    VsHs = 0,
    #[strum(serialize = "VHS" )]
    Vhs,
}

impl From<u32> for Rgb2hdmiSync {
    fn from( c: u32 ) -> Self {
        match c {
            0 => Self::VsHs,
            1 => Self::Vhs,
            _ => Self::VsHs,
        }
    }
}

impl Into<u32> for Rgb2hdmiSync {
    fn into( self ) -> u32 {
        match self {
            Self::VsHs => 0,
            Self::Vhs => 1,
        }
    }
}

// impl fmt::Display for Rgb2hdmiSync {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::CONFIG => write!(f, "CONFIG"),
//             Self::USER => write!(f, "USER"),
//             Self::UNKNOWN => write!(f, "UNKNOWN"),
//         }
//     }
// }
