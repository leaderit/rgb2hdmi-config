# Console Configuration utility for Raspberry Pi rp2040 RGB-TO-VGA-HDMI module
# (c) Valerii Grazhdankin, 2024

## How to build under MacOs

- Install and run Docker daemon

- install targets
    rustup target add \
        x86_64-apple-darwin \
        aarch64-apple-darwin \
        x86_64-pc-windows-gnu \
        x86_64-unknown-linux-gnu 

- install cross for smoorth build of the linux target
    cargo install cross --git https://github.com/cross-rs/cross

- ./build.sh

You'll have all binaries in the root directory of the project

## Help screen

    RGB2HDMI configuration utility
    (c) Valerii Grazhdankin <ias-projects@mail.ru>
    Usage: rgb2hdmi-config [OPTIONS]

    Options:
    -l, --list-ports                                       List availabale ports
        --list-ports-table                                 List availabale ports in table view
        --debug                                            Debug output
        --ping                                             Ping a device
        --mode <CONFIG, WORK>                              Set mode
    -p, --port <NAME>                                      Set device port
    -s, --save                                             Save configuration
    -c, --config                                           Print device configuration
        --video-out <VGA, HDMI, RGB, COMPOSITE>            Set video mode
        --clock <AUTO, EXTERNAL, Z80PIN6>                  Set clock mode
        --sync <VSHS, VHS>                                 Set sync mode
        --composite <PAL, NTSC>                            Composite mode
        --buffer <1X, 3X>                                  Buffer mode
        --screen <NORMAL, WIDE>                            Screen mode
        --invert <RED, GREEN, BLUE, INTENS, HS, VS, FREQ>  Invert pin
        --shift-x <SHIFT_X>                                Shif x
        --shift-y <SHIFT_Y>                                Shif y
        --freq-div <FREQ_DIV>                              Frequency divider
        --freq <FREQ>                                      Internal Frequency
        --delay <DELAY>                                    Delay
        --delay-rise <DELAY_RISE>                          Delay rise
        --delay-fall <DELAY_FALL>                          Delay fall
        --vs-len <VS_LEN>                                  Lenght of the Vertical Sync
    -h, --help                                             Print help
    -V, --version                                          Print version

## List of an avalable serial ports

    # rgb2hdmi-config --list-ports-table
    rgb2hdmi-config v0.9.9
    RGB2HDMI configuration utility
    (c) Valerii Grazhdankin <ias-projects@mail.ru>

    Available ports:
    +----------------------------------+--------------+---------+--------+
    | Name                             | Vendor       | Product | USB    |
    +----------------------------------+--------------+---------+--------+
    | /dev/cu.BLTH                     |      --      |   --    |   --   |
    +----------------------------------+--------------+---------+--------+
    | /dev/tty.BLTH                    |      --      |   --    |   --   |
    +----------------------------------+--------------+---------+--------+
    | /dev/cu.usbmodem2231201          | Raspberry Pi |  Pico   | 2e8a:a |
    +----------------------------------+--------------+---------+--------+
    | /dev/tty.usbmodem2231201         | Raspberry Pi |  Pico   | 2e8a:a |
    +----------------------------------+--------------+---------+--------+
    | /dev/cu.Bluetooth-Incoming-Port  |      --      |   --    |   --   |
    +----------------------------------+--------------+---------+--------+
    | /dev/tty.Bluetooth-Incoming-Port |      --      |   --    |   --   |
    +----------------------------------+--------------+---------+--------+

## Display a current configuration of the device

    #rgb2hdmi-config --config
    rgb2hdmi-config v0.9.9
    RGB2HDMI configuration utility
    (c) Valerii Grazhdankin <ias-projects@mail.ru>
    Device founded on port: /dev/cu.usbmodem2231201

    Device configuration:

    Mode                  : CONFIG
    Video Mode            : HDMI
    Composite Mode        : PAL
    Inversion Mask        : [Red, Green, Blue, Intens, Hs, Vs, Freq]
    Sync mode             : VHS
    Clock mode            : z80pin6
    Buffer mode           : 1X
    Screen mode           : Normal

    Shift X               : 0
    Shift Y               : 0
    Frequency divider     : 5
    Internal Frequency    : 11000000
    Delay                 : 31
    Delay Rise            : 31
    Delay Fall            : 31
    Lenght Vertycal Sync  : 500

## Command line examples

