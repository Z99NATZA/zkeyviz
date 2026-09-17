use evdev::{Device, EventSummary, InputEvent, KeyCode};
use std::io::{self, Write};
use std::option::Option;
use std::{collections::HashSet, env, error::Error};

const DEFAULT_DEVICE: &str = "/dev/input/by-id/usb-ROYUAN_Gaming_keyboard-event-kbd";
const DISPLAY_MAX: usize = 50;

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

fn handle_event(event: InputEvent, pressed_keys: &mut HashSet<KeyCode>) -> Option<KeyCode> {
    if let EventSummary::Key(_, key, value) = event.destructure()
        && let Some(state) = key_state(value)
    {
        match state {
            KeyState::Pressed => {
                pressed_keys.insert(key);
                return Some(key);
            }
            KeyState::Released => {
                pressed_keys.remove(&key);
            }
            KeyState::Repeated => {
                return Some(key);
            }
        }
    }

    None
}

fn print_scroll(text: &str) {
    let display: String = text
        .chars()
        .rev()
        .take(DISPLAY_MAX)
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect();

    print!("\x1B[?25l\r\x1B[2K{:<DISPLAY_MAX$}", display);
    let _ = io::stdout().flush();
}

fn formatting(key: KeyCode, started_shift: &mut bool) -> String {
    let key = format!("{key:?}");
    let key = key
        .to_lowercase()
        .replace("key_", "")
        .replace("left", "")
        .replace("right", "");
    let mut result = String::new();

    if key.contains("shift") {
        *started_shift = true;
        return "⌅ ".to_string();
    } else if key.contains("ctrl") {
        result = "Ctrl".to_string();
    } else if key.contains("alt") {
        result = "Alt".to_string();
    } else {
        result = clean_display_text(key.clone());

        if *started_shift {
            *started_shift = false;

            return result.to_uppercase() + " ";
        }
    }

    format!("{result} ")
}

fn clean_display_text(text: String) -> String {
    let mut cleaned = text;

    cleaned = match cleaned.as_str() {
        "backspace" => "⇤".to_string(),
        "backslash" => "\\".to_string(),
        "space" => "␣".to_string(),
        "enter" => "⮠".to_string(),
        "meta" => "super".to_string(),
        "minus" => "-".to_string(),
        "equal" => "=".to_string(),
        _ => cleaned,
    };

    cleaned.to_lowercase()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_DEVICE.to_owned());

    let mut device = Device::open(&path)?;
    let _name = device.name().unwrap_or("Unknown device");
    let mut pressed_keys = HashSet::new();
    let mut texts = String::new();
    let mut started_shift = false;

    // println!("Using keyboard: {name}");
    // println!("Device path: {path}");
    println!("\n\n\n\n\n\n\n");

    loop {
        for event in device.fetch_events()? {
            if let Some(key) = handle_event(event, &mut pressed_keys) {
                let key = formatting(key, &mut started_shift);
                texts.push_str(key.as_str());
                print_scroll(texts.as_str());
            }
        }
    }
}
