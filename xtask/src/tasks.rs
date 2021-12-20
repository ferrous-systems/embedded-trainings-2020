use std::{
    convert::TryFrom,
    fs,
    io::{self, Write as _},
    path::Path,
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use color_eyre::eyre::{anyhow, bail, Report};
use hidapi::HidApi;
use ihex::record::Record;
use serialport::SerialPortType;
use tempfile::TempDir;
use xmas_elf::{
    program::{SegmentData, Type},
    ElfFile,
};

use crate::config;

pub fn change_channel(channel: &str) -> color_eyre::Result<()> {
    fn check_pid(pid: u16) -> bool {
        pid == pids::LOOPBACK || pid == pids::PUZZLE
    }

    let api = HidApi::new()?;
    let dev = api
        .device_list()
        .filter(|dev| dev.vendor_id() == consts::VID && check_pid(dev.product_id()))
        .next()
        .ok_or_else(|| anyhow!("device not found"))?
        .open_device(&api)?;

    let chan = channel.parse::<u8>()?;
    if chan < 11 || chan > 26 {
        bail!("channel is out of range (`11..=26`)")
    }
    const REPORT_ID: u8 = 0;
    dev.write(&[REPORT_ID, chan])?;
    println!("requested channel change to channel {}", chan);

    Ok(())
}

pub fn dongle_flash(hex: &str) -> color_eyre::Result<()> {
    const VID: u16 = 0x1915;
    const PID: u16 = 0x521f;

    crate::cd(config::DONGLE_PATH)?;

    let dongle = serialport::available_ports()?
        .into_iter()
        .filter(|info| match &info.port_type {
            SerialPortType::UsbPort(info) => info.vid == VID && info.pid == PID,
            _ => false,
        })
        .next()
        .ok_or_else(|| {
            anyhow!("nRF52840 Dongle not found or it's not in bootloader mode (red LED should be blinking)\n
connect the Dongle to your laptop or PC and press the reset button to put it in bootloader mode\n
if the red LED was blinking and you got this message then the device wasn't correctly enumerated; remove it and try again")
        })?;

    let path = Path::new(hex);
    let filename = Path::new(
        path.file_name()
            .ok_or_else(|| anyhow!("{} has no file name", path.display()))?,
    );

    // NOTE assume that files with `.hex` extension are already in the IHEX format
    let is_hex = path.extension().map(|ext| ext == "hex").unwrap_or(false);

    let dir = TempDir::new()?;
    let ihex = if is_hex {
        path.to_owned()
    } else {
        // TODO move to own function
        // ELF -> IHEX
        // here we map the ELF loadable segments -- these correspond to sections like `.text`, `.rodata`
        // and `.data` (initial values) -- to ihex records
        let bytes = fs::read(path)?;
        let elf_file = ElfFile::new(&bytes).map_err(Report::msg)?;
        let mut records = vec![];
        for ph in elf_file.program_iter() {
            if ph.get_type() == Ok(Type::Load) {
                let start = ph.physical_addr();

                match ph.get_data(&elf_file).map_err(Report::msg)? {
                    SegmentData::Undefined(bytes) => {
                        // this is what `objcopy -O ihex` uses and it works with `nrfutil`
                        const RECORD_SIZE: usize = 16;

                        for (i, chunk) in bytes.chunks(RECORD_SIZE).enumerate() {
                            let offset =
                                u16::try_from(start as usize + i * RECORD_SIZE).map_err(|_| {
                                    anyhow!(
                                        "ELF with loadable addresses outside \
                                     the 16-bit address space are not supported"
                                    )
                                })?;

                            records.push(Record::Data {
                                offset,
                                value: chunk.to_owned(),
                            })
                        }
                    }

                    _ => bail!("unexpected segment data at {:#010x}", start),
                }
            }
        }

        // NOTE `objcopy` adds a `StartSegmentAddress` record before EOF but it doesn't seem to be
        // needed -- AFAICT that record matches the ELF's entry point address
        records.push(Record::EndOfFile);

        let ihex = dir.path().join(filename).with_extension("hex");
        fs::write(
            &ihex,
            ihex::writer::create_object_file_representation(&records)?,
        )?;

        println!("ELF -> iHex conversion DONE");

        ihex
    };

    // create in temp folder for automatic cleanup
    let dfu = dir.path().join(filename).with_extension("zip");

    println!("packaging iHex using nrfutil ...");

    // IHEX -> DFU package
    // silence this command; it's quite noisy
    let output = Command::new("nrfutil")
        .args(&["pkg", "generate", "--application"])
        .arg(ihex)
        .args(&[
            "--hw-version",
            "52",
            "--sd-req",
            "0x00",
            "--application-version",
            "0",
        ])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .arg(&dfu)
        .output()?;
    if !output.status.success() {
        // do show its output if it failed
        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        bail!("`nrfutil package` failed");
    }

    println!("DONE");

    // flash DFU
    let status = Command::new("nrfutil")
        .args(&["dfu", "usb-serial", "-pkg"])
        .arg(dfu)
        .args(&["-p", &dongle.port_name])
        .status()?;
    if !status.success() {
        bail!("`nrfutil dfu` failed");
    }

    Ok(())
}

pub fn serial_term() -> color_eyre::Result<()> {
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

    let mut port = serialport::new(&dongle.port_name, 115200).open()?;

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

pub fn usb_list() -> color_eyre::Result<()> {
    for dev in rusb::devices()?.iter() {
        let desc = dev.device_descriptor()?;
        let suffix = match (desc.vendor_id(), desc.product_id()) {
            (0x1366, pid) => {
                let higher_byte = pid >> 8;
                // 0x1015 and 0x0105 are valid PIDs for a J-Link probe
                if higher_byte == 0x10 || higher_byte == 0x01 {
                    " <- J-Link on the nRF52840 Development Kit"
                } else {
                    ""
                }
            }
            (0x1915, 0x521f) => " <- nRF52840 Dongle (in bootloader mode)",
            (0x2020, pids::LOOPBACK) => " <- nRF52840 Dongle (loopback.hex)",
            (0x2020, pids::PUZZLE) => " <- nRF52840 Dongle (puzzle.hex)",
            (consts::VID, consts::PID) => " <- nRF52840 on the nRF52840 Development Kit",
            _ => "",
        };

        println!("{:?}{}", dev, suffix);
    }

    Ok(())
}
