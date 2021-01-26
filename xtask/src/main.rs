#![deny(warnings)]

mod config;
mod tasks;

use std::{env, path::PathBuf};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // first arg is the name of the executable; skip it
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|arg| &arg[..]).collect::<Vec<_>>();

    match &args[..] {
        ["change-channel", channel] => tasks::change_channel(channel),
        ["dongle-flash", hex] => {
            let hexpath = env::current_dir()?.join(hex);
            tasks::dongle_flash(hexpath.to_str().unwrap())
        }
        ["serial-term"] => tasks::serial_term(),
        ["usb-list"] => tasks::usb_list(),
        _ => {
            eprintln!(
                "cargo xtask
Workshop-specific tools

USAGE:
    cargo xtask [COMMAND]

COMMANDS:
    change-channel [NUMBER]  instructs the nRF Dongle to listen to a different radio channel
    dongle-flash [PATH]      flashes the hex file on the dongle (NOTE PATH is relative to the boards/dongle directory)
    serial-term              displays the log output of the Dongle
    usb-list                 list all connected USB devices; highlights workshop devices
"
            );

            Ok(())
        }
    }
}

// changes directory into the given path, relative to the root of the repository
fn cd(segments: &[&str]) -> color_eyre::Result<()> {
    let mut path = repository_root()?;
    for segment in segments {
        path.push(segment);
    }
    env::set_current_dir(path)?;
    Ok(())
}

fn repository_root() -> color_eyre::Result<PathBuf> {
    // path to this crate (the directory that contains this crate's Cargo.toml)
    Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        // from there go one level up
        .parent()
        .unwrap()
        .to_owned())
}
