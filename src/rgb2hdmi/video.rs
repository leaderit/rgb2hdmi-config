use strum_macros::Display;
use strum_macros::EnumString;
#[derive(Display, Debug, PartialEq, EnumString, Clone)]
#[repr(u32)]
#[strum(ascii_case_insensitive)]
pub enum Rgb2hdmiVideoOut {
    #[strum(serialize = "VGA", serialize = "v")]
    VGA = 0,
    #[strum(serialize = "HDMI", serialize = "h")]
    HDMI,
    #[strum(serialize = "RGB", serialize = "r")]
    RGB,
    #[strum(serialize = "Composite", serialize = "c")]
    COMPOSITE
}

impl From<u32> for Rgb2hdmiVideoOut {
    fn from( c: u32 ) -> Self {
        match c {
            0 => Rgb2hdmiVideoOut::VGA,
            1 => Rgb2hdmiVideoOut::HDMI,
            2 => Rgb2hdmiVideoOut::RGB,
            3 => Rgb2hdmiVideoOut::COMPOSITE,
            _ => Rgb2hdmiVideoOut::VGA,
        }
    }
}

impl Into<u32> for Rgb2hdmiVideoOut {
    fn into( self ) -> u32 {
        match self {
            Rgb2hdmiVideoOut::VGA => 0,
            Rgb2hdmiVideoOut::HDMI => 1,
            Rgb2hdmiVideoOut::RGB => 2,
            Rgb2hdmiVideoOut::COMPOSITE => 3,
        }
    }
}

