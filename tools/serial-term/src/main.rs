use core::sync::atomic::{AtomicBool, Ordering};
use std::{
    fs::OpenOptions,
    io::{self, Read as _, Write as _},
};

use serialport::SerialPortType;

#[cfg(target_os = "windows")]
compiler_error!("FIXME(Windows)");

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

    // FIXME works with ttyUSB devices but not with ttyACM devices (?)
    // let mut port = serialport::open(&dongle.port_name)?;
    let mut port = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&dongle.port_name)?;

    static CONTINUE: AtomicBool = AtomicBool::new(true);

    // properly close the serial device on Ctrl-C
    ctrlc::set_handler(|| CONTINUE.store(false, Ordering::Relaxed))?;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut read_buf = [0; 64];
    while CONTINUE.load(Ordering::Relaxed) {
        // if port.bytes_to_read()? != 0 {
        let n = port.read(&mut read_buf)?;
        stdout.write_all(&read_buf[..n])?;
        stdout.flush()?;
        // } else {
        // time span between consecutive FS USB packets
        // thread::sleep(Duration::from_millis(1));
        // }
    }

    eprintln!("(closing the serial port)");
    Ok(())
}
