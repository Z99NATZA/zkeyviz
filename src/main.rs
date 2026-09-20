use evdev::{Device, EventSummary, InputEvent, KeyCode};
use std::io::{self, Write};
use std::option::Option;
use std::{collections::HashSet, env, error::Error};

const DEFAULT_DEVICE: &str = "/dev/input/by-id/usb-ROYUAN_Gaming_keyboard-event-kbd";
const DISPLAY_MAX: usize = 100;

fn is_ctrl_down(pressed_keys: &HashSet<KeyCode>) -> bool {
    pressed_keys.contains(&KeyCode::KEY_LEFTCTRL) || pressed_keys.contains(&KeyCode::KEY_RIGHTCTRL)
}

fn is_shift_down(pressed_keys: &HashSet<KeyCode>) -> bool {
    pressed_keys.contains(&KeyCode::KEY_LEFTSHIFT)
        || pressed_keys.contains(&KeyCode::KEY_RIGHTSHIFT)
}

fn is_alt_down(pressed_keys: &HashSet<KeyCode>) -> bool {
    pressed_keys.contains(&KeyCode::KEY_LEFTALT) || pressed_keys.contains(&KeyCode::KEY_RIGHTALT)
}

fn is_super_down(pressed_keys: &HashSet<KeyCode>) -> bool {
    pressed_keys.contains(&KeyCode::KEY_LEFTMETA) || pressed_keys.contains(&KeyCode::KEY_RIGHTMETA)
}

fn shortcut_text(key: KeyCode, pressed_keys: &HashSet<KeyCode>) -> String {
    let mut parts = Vec::new();

    if is_ctrl_down(pressed_keys) {
        parts.push("Ctrl".to_string());
    }

    if is_shift_down(pressed_keys) {
        parts.push("Shift".to_string());
    }

    if is_alt_down(pressed_keys) {
        parts.push("Alt".to_string());
    }

    if is_super_down(pressed_keys) {
        parts.push("Super".to_string());
    }

    parts.push(clean_key_name(key, pressed_keys));

    parts.join("+")
}

fn is_modifier(key: KeyCode) -> bool {
    matches!(
        key,
        KeyCode::KEY_LEFTCTRL
            | KeyCode::KEY_RIGHTCTRL
            | KeyCode::KEY_LEFTSHIFT
            | KeyCode::KEY_RIGHTSHIFT
            | KeyCode::KEY_LEFTALT
            | KeyCode::KEY_RIGHTALT
    )
}

fn clean_key_name(key: KeyCode, pressed_keys: &HashSet<KeyCode>) -> String {
    let name = format!("{key:?}")
        .replace("KEY_LEFT", "")
        .replace("KEY_RIGHT", "")
        .replace("KEY_", "");

    if name.len() == 1 && name.chars().all(|c| c.is_ascii_alphabetic()) {
        if is_shift_down(pressed_keys) {
            name.to_uppercase()
        } else {
            name.to_lowercase()
        }
    } else {
        let name = name.as_str();

        let text = match name {
            "CTRL" => "Ctrl",
            "SHIFT" => "Shift",
            "ALT" => "Alt",
            "ESC" => "Esc",
            "ENTER" => "⤶",
            "SPACE" => "␣",
            "BACKSPACE" => "⇤",
            "EQUAL" => "=",
            "MINUS" => "-",
            "COMMA" => ",",
            "BACKSLASH" => "\\",
            "SLASH" => "/",
            "DOT" => ".",
            "SEMICOLON" => ";",
            "COLON" => ":",
            "META" => "Super",
            _ => name,
        };

        text.to_string()
    }
}

fn handle_event(event: InputEvent, pressed_keys: &mut HashSet<KeyCode>) -> Option<String> {
    if let EventSummary::Key(_, key, value) = event.destructure() {
        match value {
            0 => {
                pressed_keys.remove(&key);
                None
            }
            1 => {
                if !pressed_keys.insert(key) {
                    return None;
                }

                if !is_modifier(key) {
                    return Some(shortcut_text(key, pressed_keys));
                }

                Some(clean_key_name(key, pressed_keys))
            }
            2 => {
                if is_modifier(key) {
                    None
                } else {
                    Some(clean_key_name(key, pressed_keys))
                }
            }
            _ => None,
        }
    } else {
        None
    }
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

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_DEVICE.to_owned());

    let mut device = Device::open(&path)?;
    let _name = device.name().unwrap_or("Unknown device");
    let mut pressed_keys = HashSet::new();
    let mut texts = String::new();

    // println!("Using keyboard: {name}");
    // println!("Device path: {path}");
    println!("\n\n\n\n\n\n\n");

    loop {
        for event in device.fetch_events()? {
            if let Some(key) = handle_event(event, &mut pressed_keys) {
                texts.push_str(&key);
                texts.push(' ');
                print_scroll(&texts);
            }
        }
    }
}
