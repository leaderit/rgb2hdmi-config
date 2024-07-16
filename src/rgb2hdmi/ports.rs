use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use serial_enumerator::{get_serial_list, SerialInfo};

#[derive(Table)]
struct SerialItem {
    #[table(title = "Name")]
    name: String,
    #[table(title = "Vendor", justify = "Justify::Center")]
    vendor: String,
    #[table(title = "Product", justify = "Justify::Center")]
    product: String,
    #[table(title = "USB", justify = "Justify::Center")]
    usb: String,
}

impl SerialItem {
    pub fn from_serial_info(serial_info: SerialInfo) -> SerialItem {
        let field_or_else = || Some(String::from("--"));
        return SerialItem {
            name: serial_info.name,
            vendor: serial_info.vendor.or_else(field_or_else).unwrap(),
            product: serial_info.product.or_else(field_or_else).unwrap(),
            usb: serial_info
                .usb_info
                .and_then(|usbinfo| Some(format!("{}:{}", usbinfo.vid, usbinfo.pid)))
                .or_else(field_or_else)
                .unwrap(),
        };
    }
}

pub fn print_ports_table() {
    let serials_info = get_serial_list();
    let mut serials_table = Vec::new();
    for serial_info in serials_info {
        serials_table.push(SerialItem::from_serial_info(serial_info));
    }
    println!("\nAvailable ports:");
    print_stdout(serials_table.with_title()).unwrap();
}