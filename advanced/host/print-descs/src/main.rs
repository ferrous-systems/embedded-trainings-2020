use anyhow::anyhow;

fn main() -> Result<(), anyhow::Error> {
    for dev in rusb::devices()?.iter() {
        let dev_desc = dev.device_descriptor()?;
        if dev_desc.vendor_id() == consts::VID && dev_desc.product_id() == consts::PID {
            println!("{:#?}", dev_desc);
            println!("address: {}", dev.address());
            for i in 0..dev_desc.num_configurations() {
                let conf_desc = dev.config_descriptor(i)?;
                println!("config{}: {:#?}", i, conf_desc);

                for iface in conf_desc.interfaces() {
                    println!(
                        "iface{}: {:#?}",
                        iface.number(),
                        iface.descriptors().collect::<Vec<_>>()
                    );
                }
            }

            // TODO open the device; this will force the OS to configure it
            // let mut handle = dev.open()?;

            return Ok(());
        }
    }

    Err(anyhow!("nRF52840 USB device not found"))
}
