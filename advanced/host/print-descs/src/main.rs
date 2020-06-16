use anyhow::anyhow;

fn main() -> Result<(), anyhow::Error> {
    for dev in rusb::devices()?.iter() {
        let dev_desc = dev.device_descriptor()?;
        if dev_desc.vendor_id() == consts::VID && dev_desc.product_id() == consts::PID {
            println!("{:#?}", dev_desc);
            for i in 0..dev_desc.num_configurations() {
                println!("{}: {:#?}", i, dev.config_descriptor(i)?);
            }
            return Ok(());
        }
    }

    Err(anyhow!("nRF52840 USB device not found"))
}
