use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Display, Debug, PartialEq, EnumString, Clone)]
#[repr(u32)]
#[strum(ascii_case_insensitive)]
pub enum Rgb2hdmiPin {   
    #[strum(serialize = "red", serialize = "r" )]
    Red = 0,
    #[strum(serialize = "green", serialize = "g")]
    Green,
    #[strum(serialize = "blue", serialize = "b")]
    Blue,
    #[strum(serialize = "intens", serialize = "i")]
    Intens,
    #[strum(serialize = "hs", serialize = "h")]
    Hs,
    #[strum(serialize = "vs", serialize = "v")]
    Vs, 
    #[strum(serialize = "freq", serialize = "f")]
    Freq
}

impl Into<u32> for Rgb2hdmiPin {
    fn into( self ) -> u32 {
        self as u32
    }
}

impl From<u32> for Rgb2hdmiPin {
    fn from( c: u32 ) -> Self {
        match c {
            0 => Self::Red,
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::Intens,
            4 => Self::Hs,
            5 => Self::Vs,
            6 => Self::Freq,
            _ => Self::Freq, 
        }        

    }
}

// Red     = 0b0000001,
// Green   = 0b0000010,
// Blue    = 0b0000100,
// Intens  = 0b0001000,
// Hs      = 0b0010000,
// Vs      = 0b0100000, 
// Freq    = 0b1000000

impl Rgb2hdmiPin {
    pub fn mask( &self ) -> u32 {
        1 << self.clone() as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!( Rgb2hdmiPin::Blue.mask(), 0b0000100);
        assert_eq!( Rgb2hdmiPin::Freq.mask(), 0b1000000);
        assert_eq!( Rgb2hdmiPin::Red.mask(),  0b0000001);
    }

    #[test]
    fn test2() {
        let v: Rgb2hdmiPin = 2.into();
        assert_eq!( Rgb2hdmiPin::Blue, v);
        assert_eq!( Rgb2hdmiPin::Red as u32,  0 );
    }    
}