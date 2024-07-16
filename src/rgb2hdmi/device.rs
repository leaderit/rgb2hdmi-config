use std::path::PathBuf;
use serial2::SerialPort;

use super::Rgb2hdmiComposite;
use super::Rgb2hdmiMode;
use super::Rgb2hdmiVideoOut;
use super::Rgb2hdmiProtocol;
use super::Rgb2hdmiClock;
use super::Rgb2hdmiPin;
use super::Rgb2hdmiSync;
use super::Rgb2hdmiBuf;
use super::Rgb2hdmiScreen;
use super::debug;

type Result<T> = std::result::Result<T, std::io::Error>;

pub struct Rgb2Hdmi {
    debug: bool,
    pub is_valid: bool,
    mode: Rgb2hdmiMode,
    port: Option<SerialPort>,
    port_name: Option<PathBuf>
}


impl Rgb2Hdmi {
    ///
    pub fn new() -> Self {

        Self {
            debug: false,
            is_valid: false,
            mode: Rgb2hdmiMode::UNKNOWN,
            port: None,
            port_name: None
        }

    }

    ///
    pub fn set_port_name(&mut self, new_name: PathBuf ) {
        self.port_name = Some(new_name);
    }

    ///
    pub fn set_debug(&mut self, debug_on: bool ) {
        self.debug = debug_on;
    }

    ///
    pub fn open(&mut self) -> Result<()> {
        if let Some(port_name) = &self.port_name {
            let port = SerialPort::open( &port_name, 115200)?;
            if cfg!(target_os = "windows") {
                port.set_dtr(true)?;
                port.set_rts(true)?;
            }
            self.port = Some(port);

            self.is_valid = self.ping();
            self.get_mode()?; 
            if self.mode != Rgb2hdmiMode::CONFIG {
                println!("Device mode is: {}", self.mode );
                print!("Set configuration mode ... ");
                self.set_mode( Rgb2hdmiMode::CONFIG )?;
                println!("Done");
                // if cfg!(target_os = "windows") {
                println!("Try again after device will be rebooted.");
                std::process::exit(0);
                // }
            }
        }
        Ok(())
    }

    ///
    pub fn ports_list() -> Result<Vec<PathBuf>> {

        let mut ports = SerialPort::available_ports()?;
        if cfg!(target_os = "windows") {

        }
        if cfg!(target_os = "macos") {
            ports = ports.iter().filter(|&p| p.to_string_lossy().starts_with("/dev/cu.usbmodem")).map(|p| p.clone()).collect();
        }
        if cfg!(target_os = "unix") {

        }

    
        Ok(ports)
    }

    pub fn port_ping( port: &SerialPort ) -> bool {
        let answer = Rgb2hdmiProtocol::Ping.command(port).unwrap_or("".into());
        answer.eq_ignore_ascii_case("ping ok")
    } 

    pub fn command(&self, cmd: Rgb2hdmiProtocol ) -> Result<String> {
        let mut answer: String = "".into();

        if let Some(port) = &self.port {
            answer = cmd.command(port)?;
        }
        Ok(answer)
    }    
    
    pub fn read(&self, cmd: Rgb2hdmiProtocol ) -> Result<u32> {
        let mut answer: u32 = 0;

        if let Some(port) = &self.port {
            answer = cmd.read(port)?;
        }
        Ok(answer)
    }  

    pub fn write(&self, cmd: Rgb2hdmiProtocol, value: u32 ) -> Result<String> {
        let mut answer: String = "".into();

        if let Some(port) = &self.port {
            answer = cmd.write(port, value)?;
        }
        Ok(answer)
    }      
    
    pub fn ping( &self ) -> bool {


        if let Some( port ) = &self.port {
            Self::port_ping( port )
        } else {
            false
        }
    }
    
    pub fn save_configuration( &mut self ) -> Result<()> {
        print!("Saving configuration ... ");
        self.command( Rgb2hdmiProtocol::Save )?;      
        println!("Done");      
        Ok(())
    }
    
    pub fn get_mode( &mut self ) -> Result<()> {
    
        let mode = self.command( Rgb2hdmiProtocol::Mode )?;
        let (_, mode) = Rgb2hdmiProtocol::answer_to_u32( &mode );
        self.mode = Rgb2hdmiMode::from( mode ); 
        Ok( () )
    }
    
    pub fn set_mode( &mut self, new_mode: Rgb2hdmiMode ) -> Result<()> {
        // println!("Mode is {}", self.mode);

        if self.mode != new_mode {
            if new_mode == Rgb2hdmiMode::USER {
                self.command( Rgb2hdmiProtocol::Exit )?;
            } else {
                self.command( Rgb2hdmiProtocol::Reset )?;
            }
            // std::thread::sleep(std::time::Duration::from_millis(1000));
            // self.open()?;

            // for _ in 0..15 {
            //     std::thread::sleep(std::time::Duration::from_millis(1000));
            //     if let Ok(_) = self.open() {
            //         break;
            //     }
            // }

            // if self.mode == new_mode {
            //     // println!("Now Mode is {}", self.mode);
            // }    
        }
        Ok( () )
    }
    
    pub fn found_device() -> Result<PathBuf> {
        let ports = Self::ports_list()?;
        // let mut buffer = [0; 256];    
        for port_name in ports {
            if debug() {
                print!("Try port: {:?} ... ", &port_name );
            }
            match SerialPort::open(&port_name, 115200) {
                Ok(port) => {
                    if debug() {
                        println!("opened");
                    }
                    if cfg!(target_os = "windows") {
                        port.set_dtr(true)?;
                        port.set_rts(true)?;
                    }
                    if Self::port_ping(&port) {
                        return Ok(port_name);
                    }
                },
                Err( e ) => {
                    if debug() {
                        println!("open error {:?}", e);
                    }
                    continue
                },
            };
        };
        Err( std::io::Error::last_os_error() )
    }
    
    pub fn print_ports() -> Result<()> {
        let ports = Self::ports_list()?;
        println!("Available serial ports:\n");
        for port in ports {
            println!("    {}", port.to_string_lossy());
        }
        Ok(())
    }
    
    pub fn print_config(&mut self) -> Result<()> {
        if self.is_valid {
            // Get configuration
            if self.mode ==  Rgb2hdmiMode::CONFIG {
                // Print parameters
                println!("\nDevice configuration:\n");
                println!("Mode                  : {}", self.mode);
                println!("Video Mode            : {}", self.get_video_mode()?);
                println!("Composite Mode        : {}", self.get_composite_mode()?);
                println!("Inversion Mask        : {:?}", self.get_inv_mask()?);
                println!("Sync mode             : {}", self.get_sync_mode()?);
                println!("Clock mode            : {}", self.get_clock_mode()?);
                println!("Buffer mode           : {}", self.get_buf_mode()?);
                println!("Screen mode           : {}", self.get_screen_mode()?);
                println!("");
                println!("Shift X               : {}", self.read( Rgb2hdmiProtocol::ShiftX )? );
                println!("Shift Y               : {}", self.read( Rgb2hdmiProtocol::ShiftY )? );
                println!("Frequency divider     : {}", self.read( Rgb2hdmiProtocol::ExtFDiv )? );
                println!("Internal Frequency    : {}", self.read( Rgb2hdmiProtocol::IntF )? );
                println!("Delay                 : {}", self.read( Rgb2hdmiProtocol::Delay )? );
                println!("Delay Rise            : {}", self.read( Rgb2hdmiProtocol::DelayRise )? );
                println!("Delay Fall            : {}", self.read( Rgb2hdmiProtocol::DelayFall )? );
                println!("Lenght Vertycal Sync  : {}", self.read( Rgb2hdmiProtocol::LenVs )? );
            } else {
                println!("Incorrect device mode.");
            }
        }
        Ok(())
    }        

    pub fn get_video_mode( &mut self ) -> Result<Rgb2hdmiVideoOut> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            let video_mode = self.read( Rgb2hdmiProtocol::VideoOut )?;
            return Ok(video_mode.into());  
        } else {
            println!("Incorrect device mode.");
        }
        Ok( Rgb2hdmiVideoOut::VGA )
    }    

    pub fn set_video_mode( &mut self, video_mode: Rgb2hdmiVideoOut ) -> Result<()> {
        if self.mode ==  Rgb2hdmiMode::CONFIG {   
            self.write( Rgb2hdmiProtocol::VideoOut, video_mode.into() )?;
        } else {
            println!("Incorrect device mode.");
        }
        Ok(())
    }    

    pub fn get_composite_mode( &mut self ) -> Result<Rgb2hdmiComposite> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            let video_mode = self.read( Rgb2hdmiProtocol::CMode )?;
            return Ok(video_mode.into());  
        } else {
            println!("Incorrect device mode.");
        }
        Ok( Rgb2hdmiComposite::PAL )
    }    

    pub fn set_composite_mode( &mut self, video_mode: Rgb2hdmiComposite ) -> Result<()> {
        if self.mode ==  Rgb2hdmiMode::CONFIG {   
            self.write( Rgb2hdmiProtocol::CMode, video_mode.into() )?;
        } else {
            println!("Incorrect device mode.");
        }
        Ok(())
    }    

    pub fn get_inv_mask( &mut self ) -> Result<Vec<Rgb2hdmiPin>> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            let mode = self.read( Rgb2hdmiProtocol::InInvMask )?;
            let mut mask = 1;
            let mut invert: Vec<Rgb2hdmiPin> = vec![]; 
            for bit in 0..7 as u32 {
                if mode & mask !=0 {
                    let value: Rgb2hdmiPin = bit.into();
                    invert.push(value);
                }
                mask = mask << 1;
            }
            return Ok(invert);  
        } else {
            println!("Incorrect device mode.");
        }
        Ok( vec![] )
    }

    pub fn set_inv_mask( &mut self, invert: &Vec<Rgb2hdmiPin> ) -> Result<()> {
        if self.mode ==  Rgb2hdmiMode::CONFIG {   
            let mut value: u32 = 0;
            for pin in invert {
                value = value | pin.mask();
            }    
            self.write( Rgb2hdmiProtocol::InInvMask, value )?;
        } else {
            println!("Incorrect device mode.");
        }
        Ok(())
    }


    pub fn get_sync_mode( &mut self ) -> Result<Rgb2hdmiSync> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            let mode = self.read( Rgb2hdmiProtocol::SyncMode )?;
            return Ok( mode.into() );  
        } else {
            println!("Incorrect device mode.");
        }
        Ok( Rgb2hdmiSync::VsHs )
    }

    pub fn set_sync_mode( &mut self, value: Rgb2hdmiSync ) -> Result<()> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            self.write( Rgb2hdmiProtocol::SyncMode, value.into() )?;
        } else {
            println!("Incorrect device mode.");
        }
        Ok(())
    }       

    pub fn get_clock_mode( &mut self ) -> Result<Rgb2hdmiClock> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            let mode = self.read( Rgb2hdmiProtocol::PClkMode )?;
            return Ok(mode.into() );  
        } else {
            println!("Incorrect device mode.");
        }
        Ok( Rgb2hdmiClock::AUTO )
    }

    pub fn set_clock_mode( &mut self, value: Rgb2hdmiClock ) -> Result<()> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            self.write( Rgb2hdmiProtocol::PClkMode, value.into() )?;
        } else {
            println!("Incorrect device mode.");
        }
        Ok(())
    }    

    pub fn get_buf_mode( &mut self ) -> Result<Rgb2hdmiBuf> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            let mode = self.read( Rgb2hdmiProtocol::Bufmode )?;
            return Ok(mode.into() );  
        } else {
            println!("Incorrect device mode.");
        }
        Ok( Rgb2hdmiBuf::Buf1x )
    }

    pub fn set_buf_mode( &mut self, value: Rgb2hdmiBuf ) -> Result<()> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            self.write( Rgb2hdmiProtocol::Bufmode, value.into() )?;
        } else {
            println!("Incorrect device mode.");
        }
        Ok(())
    }       

    pub fn get_screen_mode( &mut self ) -> Result<Rgb2hdmiScreen> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            let mode = self.read( Rgb2hdmiProtocol::WideMode )?;
            return Ok(mode.into() );  
        } else {
            println!("Incorrect device mode.");
        }
        Ok( Rgb2hdmiScreen::Normal )
    }

    pub fn set_screen_mode( &mut self, value: Rgb2hdmiScreen ) -> Result<()> {
        if self.mode ==  Rgb2hdmiMode::CONFIG { 
            self.write( Rgb2hdmiProtocol::WideMode, value.into() )?;
        } else {
            println!("Incorrect device mode.");
        }
        Ok(())
    }
    
}
