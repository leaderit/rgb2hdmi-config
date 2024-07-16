// use std::io::Write;

use strum_macros::Display;
use strum_macros::EnumString;
use serial2::SerialPort;

type Result<T> = std::result::Result<T, std::io::Error>;

static mut DEBUG: bool = false;

pub fn set_debug( value: bool ) {
    unsafe { DEBUG = value };
}


pub fn debug() -> bool {
    unsafe { DEBUG }
}

#[derive(Display, Debug, PartialEq, EnumString)]
pub enum Rgb2hdmiProtocol {
    // Control commands
    #[strum(serialize = "ping")]            
    Ping,
    #[strum(serialize = "mode")]            
    Mode,
    #[strum(serialize = "reset")]
    Reset,
    #[strum(serialize = "exit")]
    Exit,
    #[strum(serialize = "save")]
    Save,

    // Read/Write parameters with prefix r/w
    // in user mode only works:
    // cap_sh_x
    // cap_sh_y

    #[strum(serialize = "cap_sh_y")]
    ShiftY,
    #[strum(serialize = "cap_sh_x")]
    ShiftX,

    // in configurator mode only works
    #[strum(serialize = "cap_ext_f_div")]
    ExtFDiv,
    #[strum(serialize = "cap_int_f")]
    IntF,
    #[strum(serialize = "cap_delay")]
    Delay,
    #[strum(serialize = "cap_delay_rise")]
    DelayRise,
    #[strum(serialize = "cap_delay_fall")]
    DelayFall,
    #[strum(serialize = "cap_len_VS")]
    LenVs,
    #[strum(serialize = "wide_mode")]
    WideMode,  
    #[strum(serialize = "video_out")]       //
    VideoOut, 
    #[strum(serialize = "c_mode")]
    CMode,
    #[strum(serialize = "is_3X_bufmode")]
    Bufmode,
    #[strum(serialize = "cap_sync_mode")]   //
    SyncMode,
    #[strum(serialize = "cap_p_clk_mode")]
    PClkMode,
    #[strum(serialize = "cap_in_inv_mask")] //
    InInvMask,
}

impl Rgb2hdmiProtocol {
    pub fn is_command(&self) -> bool {
        match self {
            Self::Ping |
            Self::Mode |
            Self::Reset |
            Self::Save |
            Self::Exit => true,
            _ => false
        }
    }

    fn port_send_command( port: &SerialPort, cmd: &str ) -> Result<String> {
        let mut buffer = [0; 256];

        let _ = port.write( format!("{cmd}\n").as_bytes() );
        if cfg!(target_os = "windows") {
            port.flush()?;
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let _ = port.read(&mut buffer);
        let answer = std::str::from_utf8(&buffer).unwrap_or("");
        let (answer,_) = answer.split_once("\n").unwrap_or(("",""));
        if debug() { 
            println!("command:{cmd}, answer: {:?}", answer); 
            println!("Raw answer: {:?}", buffer); 
        }
        Ok(answer.to_string())
    }

    pub fn command( &self, port: &SerialPort ) -> Result<String> {
        let mut answer: String = "".into();
        if self.is_command() {
            let cmd = self.to_string();
            answer = Self::port_send_command(port, &cmd)?;
        }
        Ok( answer )
    }

    pub fn read( &self, port: &SerialPort ) -> Result<u32> {
        let mut answer: u32 = 0;
        if !self.is_command() {
            let cmd = format!("r{}", self.to_string());
            let raw_answer = Self::port_send_command(port, &cmd)?;
            (_, answer ) = Self::answer_to_u32(&raw_answer);
        }
        Ok( answer )
    }

    pub fn write( &self, port: &SerialPort, value: u32 ) -> Result<String> {
        let mut answer: String = "".into();
        if !self.is_command() {
            let cmd = format!("w{} {}", self.to_string(), value);
            answer = Self::port_send_command(port, &cmd)?;
        }
        Ok( answer )
    }

    pub fn _answer_to_string( answer: &str ) -> (String, String) {
        if let Some((name, value)) = answer.split_once(' ') {
            (name.into(), value.into() )
        } else {
            ("".into(), "".into())
        }
    }

    pub fn answer_to_u32( answer: &str ) -> (String, u32) {
        if let Some((name, value)) = answer.split_once(' ') {
            (name.into(), value.parse().unwrap_or(0) )
        } else {
            ("".into(), 0)
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let c = Rgb2hdmiProtocol::Mode;
        let p = Rgb2hdmiProtocol::Bufmode; 
        assert_eq!( "mode", c.to_string());
        assert_eq!( Rgb2hdmiProtocol::Bufmode, p );
        assert_eq!( true, c.is_command() );
        assert_eq!( false, p.is_command() );        
    }
}