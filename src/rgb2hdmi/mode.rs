use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rgb2hdmiMode {
    UNKNOWN,
    USER,
    CONFIG
}

impl From<u32> for Rgb2hdmiMode {
    fn from( c: u32 ) -> Self {
        match c {
            0 => Rgb2hdmiMode::CONFIG,
            1 => Rgb2hdmiMode::USER,
            _ => Rgb2hdmiMode::UNKNOWN,
        }
    }
}

impl Into<u32> for Rgb2hdmiMode {
    fn into( self ) -> u32 {
        match self {
            Rgb2hdmiMode::CONFIG => 0,
            Rgb2hdmiMode::USER => 1,
            Rgb2hdmiMode::UNKNOWN => 0,
        }
    }
}

impl fmt::Display for Rgb2hdmiMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rgb2hdmiMode::CONFIG => write!(f, "CONFIG"),
            Rgb2hdmiMode::USER => write!(f, "USER"),
            Rgb2hdmiMode::UNKNOWN => write!(f, "UNKNOWN"),
        }
    }
}
