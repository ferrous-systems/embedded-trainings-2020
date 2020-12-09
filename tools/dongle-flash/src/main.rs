use core::convert::TryFrom;
use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail, ensure};
use ihex::record::Record;
use serialport::SerialPortType;
use tempfile::TempDir;
use xmas_elf::{
    program::{SegmentData, Type},
    ElfFile,
};

const VID: u16 = 0x1915;
const PID: u16 = 0x521f;

fn main() -> Result<(), anyhow::Error> {
    let args = env::args().skip(1 /* program name */).collect::<Vec<_>>();

    ensure!(args.len() == 1, "expected exactly one argument");

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

    let path = Path::new(&args[0]);
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
        // ELF -> IHEX
        // here we map the ELF loadable segments -- these correspond to sections like `.text`, `.rodata`
        // and `.data` (initial values) -- to ihex records
        let bytes = fs::read(path)?;
        let elf_file = ElfFile::new(&bytes).map_err(anyhow::Error::msg)?;
        let mut records = vec![];
        for ph in elf_file.program_iter() {
            if ph.get_type() == Ok(Type::Load) {
                let start = ph.physical_addr();

                match ph.get_data(&elf_file).map_err(anyhow::Error::msg)? {
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
