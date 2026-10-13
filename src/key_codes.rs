#![allow(dead_code)]
use evdev::KeyCode;

pub fn evdev_to_key_name(code: u16) -> &'static str {
    match code {
        // Letras
        16 => "Q",
        17 => "W",
        18 => "E",
        19 => "R",
        20 => "T",
        21 => "Y",
        22 => "U",
        23 => "I",
        24 => "O",
        25 => "P",
        30 => "A",
        31 => "S",
        32 => "D",
        33 => "F",
        34 => "G",
        35 => "H",
        36 => "J",
        37 => "K",
        38 => "L",
        44 => "Z",
        45 => "X",
        46 => "C",
        47 => "V",
        48 => "B",
        49 => "N",
        50 => "M",

        // Números linha superior
        2 => "1",
        3 => "2",
        4 => "3",
        5 => "4",
        6 => "5",
        7 => "6",
        8 => "7",
        9 => "8",
        10 => "9",
        11 => "0",

        // Modificadores e Teclas Especiais
        1 => "Escape",
        14 => "Backspace",
        15 => "Tab",
        28 => "Enter",
        29 => "Left Ctrl",
        42 => "Left Shift",
        54 => "Right Shift",
        56 => "Left Alt",
        57 => "Space",
        58 => "Caps Lock",
        97 => "Right Ctrl",
        100 => "Right Alt",
        125 => "Left Super (Win)",
        126 => "Right Super (Win)",

        // Função F1 - F12
        59 => "F1",
        60 => "F2",
        61 => "F3",
        62 => "F4",
        63 => "F5",
        64 => "F6",
        65 => "F7",
        66 => "F8",
        67 => "F9",
        68 => "F10",
        87 => "F11",
        88 => "F12",

        // Navegação
        102 => "Home",
        103 => "Up Arrow",
        104 => "Page Up",
        105 => "Left Arrow",
        106 => "Right Arrow",
        107 => "End",
        108 => "Down Arrow",
        109 => "Page Down",
        110 => "Insert",
        111 => "Delete",

        // Sinais
        12 => "Minus (-)",
        13 => "Equal (=)",
        26 => "[",
        27 => "]",
        39 => ";",
        40 => "'",
        41 => "`",
        43 => "\\",
        51 => ",",
        52 => ".",
        53 => "/",

        // Teclado Numérico (Numpad)
        69 => "Num Lock",
        98 => "Numpad /",
        55 => "Numpad *",
        74 => "Numpad -",
        78 => "Numpad +",
        96 => "Numpad Enter",
        79 => "Numpad 7",
        80 => "Numpad 8",
        81 => "Numpad 9",
        75 => "Numpad 4",
        76 => "Numpad 5",
        77 => "Numpad 6",
        71 => "Numpad 1",
        72 => "Numpad 2",
        73 => "Numpad 3",
        82 => "Numpad 0",
        83 => "Numpad .",

        _ => "Unknown Key",
    }
}

pub fn evdev_code_to_keycode(code: u16) -> KeyCode {
    KeyCode::new(code)
}

pub fn egui_key_to_evdev(key: egui::Key) -> Option<u16> {
    use egui::Key;
    let code = match key {
        Key::A => 30,
        Key::B => 48,
        Key::C => 46,
        Key::D => 32,
        Key::E => 18,
        Key::F => 33,
        Key::G => 34,
        Key::H => 35,
        Key::I => 23,
        Key::J => 36,
        Key::K => 37,
        Key::L => 38,
        Key::M => 50,
        Key::N => 49,
        Key::O => 24,
        Key::P => 25,
        Key::Q => 16,
        Key::R => 19,
        Key::S => 31,
        Key::T => 20,
        Key::U => 22,
        Key::V => 47,
        Key::W => 17,
        Key::X => 45,
        Key::Y => 21,
        Key::Z => 44,

        Key::Num0 => 11,
        Key::Num1 => 2,
        Key::Num2 => 3,
        Key::Num3 => 4,
        Key::Num4 => 5,
        Key::Num5 => 6,
        Key::Num6 => 7,
        Key::Num7 => 8,
        Key::Num8 => 9,
        Key::Num9 => 10,

        Key::Space => 57,
        Key::Enter => 28,
        Key::Tab => 15,
        Key::Backspace => 14,
        Key::Insert => 110,
        Key::Delete => 111,
        Key::Home => 102,
        Key::End => 107,
        Key::PageUp => 104,
        Key::PageDown => 109,

        Key::ArrowUp => 103,
        Key::ArrowDown => 108,
        Key::ArrowLeft => 105,
        Key::ArrowRight => 106,

        Key::F1 => 59,
        Key::F2 => 60,
        Key::F3 => 61,
        Key::F4 => 62,
        Key::F5 => 63,
        Key::F6 => 64,
        Key::F7 => 65,
        Key::F8 => 66,
        Key::F9 => 67,
        Key::F10 => 68,
        Key::F11 => 87,
        Key::F12 => 88,

        _ => return None,
    };
    Some(code)
}

/// Converte o código antigo do Qt (do projeto original em C++) para evdev
pub fn qt_key_to_linux_evdev(qt_key: i32) -> u16 {
    match qt_key {
        // Letras (ASCII/Qt maiúsculo)
        0x41 => 30, // A
        0x42 => 48, // B
        0x43 => 46, // C
        0x44 => 32, // D
        0x45 => 18, // E
        0x46 => 33, // F
        0x47 => 34, // G
        0x48 => 35, // H
        0x49 => 23, // I
        0x4a => 36, // J
        0x4b => 37, // K
        0x4c => 38, // L
        0x4d => 50, // M
        0x4e => 49, // N
        0x4f => 24, // O
        0x50 => 25, // P
        0x51 => 16, // Q
        0x52 => 19, // R
        0x53 => 31, // S
        0x54 => 20, // T
        0x55 => 22, // U
        0x56 => 47, // V
        0x57 => 17, // W
        0x58 => 45, // X
        0x59 => 21, // Y
        0x5a => 44, // Z

        // Números
        0x30 => 11, // 0
        0x31 => 2,  // 1
        0x32 => 3,  // 2
        0x33 => 4,  // 3
        0x34 => 5,  // 4
        0x35 => 6,  // 5
        0x36 => 7,  // 6
        0x37 => 8,  // 7
        0x38 => 9,  // 8
        0x39 => 10, // 9

        // Especiais
        0x20 => 57,        // Space
        0x01000004 => 28,  // Return / Enter
        0x01000005 => 96,  // Enter Numpad
        0x01000001 => 15,  // Tab
        0x01000003 => 14,  // Backspace
        0x01000020 => 42,  // Shift
        0x01000021 => 29,  // Control
        0x01000023 => 56,  // Alt
        0x01000022 => 125, // Meta/Super
        0x01000024 => 58,  // CapsLock
        0x01000000 => 1,   // Escape

        // Setas
        0x01000013 => 103, // Up
        0x01000015 => 108, // Down
        0x01000012 => 105, // Left
        0x01000014 => 106, // Right

        _ => 0,
    }
}
