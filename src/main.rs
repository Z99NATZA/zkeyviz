fn main() {
    println!("Available input devices:");

    for (path, device) in evdev::enumerate() {
        let name = device.name().unwrap_or("Unknown device");

        println!("{}: {}", path.display(), name);
    }
}
