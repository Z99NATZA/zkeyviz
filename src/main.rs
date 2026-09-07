use evdev::{Device, EventSummary, InputEvent, KeyCode};
use std::{collections::HashSet, env, error::Error};

const DEFAULT_DEVICE: &str = "/dev/input/by-id/usb-ROYUAN_Gaming_keyboard-event-kbd";

#[derive(Debug, PartialEq)]
enum KeyState {
    Released,
    Pressed,
    Repeated,
}

fn key_state(value: i32) -> Option<KeyState> {
    match value {
        0 => Some(KeyState::Released),
        1 => Some(KeyState::Pressed),
        2 => Some(KeyState::Repeated),
        _ => None,
    }
}

fn handle_event(event: InputEvent, pressed_keys: &mut HashSet<KeyCode>) -> bool {
    let mut change = false;

    if let EventSummary::Key(_, key, value) = event.destructure() {
        if let Some(state) = key_state(value) {
            match state {
                KeyState::Pressed => {
                    change = pressed_keys.insert(key);
                }
                KeyState::Released => {
                    change = pressed_keys.remove(&key);
                }
                KeyState::Repeated => {}
            }
        }
    }

    change
}

fn render(pressed_keys: &HashSet<KeyCode>) {
    println!("{pressed_keys:?}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_DEVICE.to_owned());

    let mut device = Device::open(&path)?;
    let name = device.name().unwrap_or("Unknown device");
    let mut pressed_keys = HashSet::new();

    println!("Using keyboard: {name}");
    println!("Device path: {path}");

    loop {
        for event in device.fetch_events()? {
            if handle_event(event, &mut pressed_keys) {
                render(&pressed_keys);
            }
        }
    }
}
