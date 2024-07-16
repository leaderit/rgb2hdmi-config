use clap::Parser;

use crate::rgb2hdmi::Rgb2hdmiClock;
use crate::rgb2hdmi::Rgb2hdmiVideoOut;
use crate::rgb2hdmi::Rgb2hdmiPin;
use crate::rgb2hdmi::Rgb2hdmiSync;
use crate::rgb2hdmi::Rgb2hdmiComposite;
use crate::rgb2hdmi::Rgb2hdmiScreen;
use crate::rgb2hdmi::Rgb2hdmiBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// List availabale ports
    #[arg(short, long)]
    pub list_ports: bool,

    /// List availabale ports in table view
    #[arg(long)]
    pub list_ports_table: bool,

    /// Debug output
    #[arg(long)]
    pub debug: bool,

    /// Ping a device
    #[arg(long)]
    pub ping: bool,
    /// Set mode
    #[arg(long, value_name = "CONFIG, WORK")]
    pub mode: Option<u32>,
    /// Set device port
    #[arg(short, long, value_name = "NAME")]
    pub port: Option<String>,
    /// Save configuration
    #[arg(short, long)]
    pub save: bool,    

    /// Print device configuration
    #[arg(short, long)]
    pub config: bool,

    /// Set video mode
    #[arg(long, value_name = "VGA, HDMI, RGB, COMPOSITE")]
    pub video_out: Option<Rgb2hdmiVideoOut>,
    /// Set clock mode
    #[arg(long, value_name = "AUTO, EXTERNAL, Z80PIN6")]
    pub clock: Option<Rgb2hdmiClock>,

    /// Set sync mode
    #[arg(long, value_name = "VSHS, VHS")]
    pub sync: Option<Rgb2hdmiSync>,

    /// Composite mode
    #[arg(long, value_name = "PAL, NTSC")]
    pub composite: Option<Rgb2hdmiComposite>,

    /// Buffer mode
    #[arg(long, value_name = "1X, 3X")]
    pub buffer: Option<Rgb2hdmiBuf>,

    /// Screen mode
    #[arg(long, value_name = "NORMAL, WIDE")]
    pub screen: Option<Rgb2hdmiScreen>,    

    /// Invert pin
    #[arg(long, value_delimiter = ',', value_name = "RED, GREEN, BLUE, INTENS, HS, VS, FREQ")]
    pub invert: Option<Vec<Rgb2hdmiPin>>,

    /// Shif x
    #[arg(long)]
    pub shift_x: Option<u32>,
    
    /// Shif y
    #[arg(long)]
    pub shift_y: Option<u32>,    

    /// Frequency divider
    #[arg(long)]
    pub freq_div: Option<u32>, 

    /// Internal Frequency
    #[arg(long)]
    pub freq: Option<u32>,

    /// Delay
    #[arg(long)]
    pub delay: Option<u32>, 

    /// Delay rise
    #[arg(long)]
    pub delay_rise: Option<u32>, 

    /// Delay fall
    #[arg(long)]
    pub delay_fall: Option<u32>, 

    /// Lenght of the Vertical Sync
    #[arg(long)]
    pub vs_len: Option<u32>,     
}

impl Args {
    pub fn new() -> Args {
        Args::parse()
    }
}

