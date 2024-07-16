mod args;
mod rgb2hdmi;

use crate::args::Args;
use crate::rgb2hdmi::Rgb2Hdmi;
use crate::rgb2hdmi::Rgb2hdmiProtocol;
use crate::rgb2hdmi::set_debug;
use crate::rgb2hdmi::print_ports_table;

use std::path::PathBuf;

type Result<T> = std::result::Result<T, std::io::Error>;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str    = env!("CARGO_PKG_NAME");
const APP_AUTHORS: &str  = env!("CARGO_PKG_AUTHORS");

fn main() -> Result<()> {
    println!("{} v{}\nRGB2HDMI configuration utility\n(c) {}", 
    APP_NAME, APP_VERSION, APP_AUTHORS);

    let args = Args::new();
    let mut device = Rgb2Hdmi::new();

    device.set_debug( args.debug );
    set_debug( args.debug );

    if args.list_ports {
        Rgb2Hdmi::print_ports()?;
        return Ok(());
    }

    if args.list_ports_table {
        print_ports_table();
        return Ok(());
    }    

    let port = if let Some(port) = args.port {
        Some(PathBuf::from(port))
    } else {
        if let Ok(port) = Rgb2Hdmi::found_device() {
            println!("Device founded on port: {}", port.to_string_lossy());
            Some(port)
        } else {
            println!("Device not connected");
            None
        }
    };

    if let Some(port) = port {
        device.set_port_name( port );
        if let Ok(()) = device.open() {
            if device.is_valid {

                if args.config {
                    device.print_config()?;
                }
    
                if let Some(mode) = args.mode {
                    device.set_mode(mode.into())?;
                }
    
                if let Some(video_mode) = args.video_out {
                    device.set_video_mode(video_mode.into())?;
                }
                
                if let Some(invert) = args.invert {
                    device.set_inv_mask(&invert)?;
                }

                if let Some(clock) = args.clock {
                    device.set_clock_mode( clock )?;
                }

                if let Some(sync) = args.sync {
                    device.set_sync_mode( sync )?;
                }                

                if let Some(value) = args.composite {
                    device.set_composite_mode( value )?;
                }  

                if let Some(value) = args.screen {
                    device.set_screen_mode( value )?;
                }  

                if let Some(value) = args.buffer {
                    device.set_buf_mode( value )?;
                } 

                // Shifts adn delays
                if let Some(value) = args.shift_x {
                    device.write( Rgb2hdmiProtocol::ShiftX, value )?;
                } 

                if let Some(value) = args.shift_y {
                    device.write( Rgb2hdmiProtocol::ShiftY, value )?;
                } 

                if let Some(value) = args.freq_div {
                    device.write( Rgb2hdmiProtocol::ExtFDiv, value )?;
                }

                if let Some(value) = args.freq {
                    device.write( Rgb2hdmiProtocol::IntF, value )?;
                }

                if let Some(value) = args.delay {
                    device.write( Rgb2hdmiProtocol::Delay, value )?;
                }

                if let Some(value) = args.delay_rise {
                    device.write( Rgb2hdmiProtocol::DelayRise, value )?;
                }
                if let Some(value) = args.delay_fall {
                    device.write( Rgb2hdmiProtocol::DelayFall, value )?;
                }
                if let Some(value) = args.vs_len {
                    device.write( Rgb2hdmiProtocol::LenVs, value )?;
                }

                if  args.save {
                    device.save_configuration()?;
                }
            } else {
                println!("Device is not valid");
            }    
        } else {
            println!("Device is not found");
        }
            } else {
        println!("No Device port");
    }
    Ok(())  
}

