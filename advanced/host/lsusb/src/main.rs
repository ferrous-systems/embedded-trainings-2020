use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    for dev in rusb::devices()?.iter() {
        let desc = dev.device_descriptor()?;
        let mut show_config = false;
        let suffix = match (desc.vendor_id(), desc.product_id()) {
            (0x1366, 0x1015) => " <- J-Link on the nRF52840 Development Kit",
            (0x1915, 0x521f) => " <- nRF52840 Dongle (in bootloader mode)",
            (0x2020, 0x0309) => " <- nRF52840 Dongle (loopback.hex)",
            (consts::VID, consts::PID) => {
                show_config = true;
                " <- nRF52840 on the nRF52840 Development Kit"
            }
            _ => "",
        };

        println!("{:?}{}", dev, suffix);
        if show_config {
            let desc = dev.config_descriptor(0)?;
            println!("> {:?}", desc);
        }
    }

    Ok(())
}
