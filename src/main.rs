use evdev::Device;
use std::{env, error::Error};

const DEFAULT_DEVICE: &str = "/dev/input/by-id/usb-ROYUAN_Gaming_keyboard-event-kbd";

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_DEVICE.to_owned());

    let device = Device::open(&path)?;
    let name = device.name().unwrap_or("Unknown device");

    println!("Using keyboard: {name}");
    println!("Device path: {path}");

    Ok(())
}
