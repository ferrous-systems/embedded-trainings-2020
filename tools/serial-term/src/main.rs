use core::sync::atomic::{AtomicBool, Ordering};
use std::{
    fs::OpenOptions,
    io::{self, Read as _, Write as _},
    thread,
    time::Duration,
};

use serialport::SerialPortType;

fn main() -> Result<(), anyhow::Error> {
    let mut once = true;
    let dongle = loop {
        if let Some(dongle) = serialport::available_ports()?
            .into_iter()
            .filter(|info| match &info.port_type {
                SerialPortType::UsbPort(usb) => usb.vid == consts::VID,
                _ => false,
            })
            .next()
        {
            break dongle;
        } else if once {
            once = false;

            eprintln!("(waiting for the Dongle to be connected)");
        }
    };

    let mut port = serialport::open(&dongle.port_name)?;

    static CONTINUE: AtomicBool = AtomicBool::new(true);

    // properly close the serial device on Ctrl-C
    ctrlc::set_handler(|| CONTINUE.store(false, Ordering::Relaxed))?;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut read_buf = [0; 64];
    while CONTINUE.load(Ordering::Relaxed) {
        if port.bytes_to_read()? != 0 {
            let n = port.read(&mut read_buf)?;
            stdout.write_all(&read_buf[..n])?;
            stdout.flush()?;
        } else {
            // time span between two consecutive FS USB packets
            thread::sleep(Duration::from_millis(1));
        }
    }

    eprintln!("(closing the serial port)");
    Ok(())
}
