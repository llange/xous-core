pub use usbd_human_interface_device::page::Keyboard as UsbKeyCode;
#[allow(dead_code)]
pub fn char_to_hid_code_us101(key: char) -> Vec<UsbKeyCode> {
    let mut code = vec![];
    match key {
        'a' => code.push(UsbKeyCode::A),
        'b' => code.push(UsbKeyCode::B),
        'c' => code.push(UsbKeyCode::C),
        'd' => code.push(UsbKeyCode::D),
        'e' => code.push(UsbKeyCode::E),
        'f' => code.push(UsbKeyCode::F),
        'g' => code.push(UsbKeyCode::G),
        'h' => code.push(UsbKeyCode::H),
        'i' => code.push(UsbKeyCode::I),
        'j' => code.push(UsbKeyCode::J),
        'k' => code.push(UsbKeyCode::K),
        'l' => code.push(UsbKeyCode::L),
        'm' => code.push(UsbKeyCode::M),
        'n' => code.push(UsbKeyCode::N),
        'o' => code.push(UsbKeyCode::O),
        'p' => code.push(UsbKeyCode::P),
        'q' => code.push(UsbKeyCode::Q),
        'r' => code.push(UsbKeyCode::R),
        's' => code.push(UsbKeyCode::S),
        't' => code.push(UsbKeyCode::T),
        'u' => code.push(UsbKeyCode::U),
        'v' => code.push(UsbKeyCode::V),
        'w' => code.push(UsbKeyCode::W),
        'x' => code.push(UsbKeyCode::X),
        'y' => code.push(UsbKeyCode::Y),
        'z' => code.push(UsbKeyCode::Z),

        'A' => {code.push(UsbKeyCode::A); code.push(UsbKeyCode::LeftShift)},
        'B' => {code.push(UsbKeyCode::B); code.push(UsbKeyCode::LeftShift)},
        'C' => {code.push(UsbKeyCode::C); code.push(UsbKeyCode::LeftShift)},
        'D' => {code.push(UsbKeyCode::D); code.push(UsbKeyCode::LeftShift)},
        'E' => {code.push(UsbKeyCode::E); code.push(UsbKeyCode::LeftShift)},
        'F' => {code.push(UsbKeyCode::F); code.push(UsbKeyCode::LeftShift)},
        'G' => {code.push(UsbKeyCode::G); code.push(UsbKeyCode::LeftShift)},
        'H' => {code.push(UsbKeyCode::H); code.push(UsbKeyCode::LeftShift)},
        'I' => {code.push(UsbKeyCode::I); code.push(UsbKeyCode::LeftShift)},
        'J' => {code.push(UsbKeyCode::J); code.push(UsbKeyCode::LeftShift)},
        'K' => {code.push(UsbKeyCode::K); code.push(UsbKeyCode::LeftShift)},
        'L' => {code.push(UsbKeyCode::L); code.push(UsbKeyCode::LeftShift)},
        'M' => {code.push(UsbKeyCode::M); code.push(UsbKeyCode::LeftShift)},
        'N' => {code.push(UsbKeyCode::N); code.push(UsbKeyCode::LeftShift)},
        'O' => {code.push(UsbKeyCode::O); code.push(UsbKeyCode::LeftShift)},
        'P' => {code.push(UsbKeyCode::P); code.push(UsbKeyCode::LeftShift)},
        'Q' => {code.push(UsbKeyCode::Q); code.push(UsbKeyCode::LeftShift)},
        'R' => {code.push(UsbKeyCode::R); code.push(UsbKeyCode::LeftShift)},
        'S' => {code.push(UsbKeyCode::S); code.push(UsbKeyCode::LeftShift)},
        'T' => {code.push(UsbKeyCode::T); code.push(UsbKeyCode::LeftShift)},
        'U' => {code.push(UsbKeyCode::U); code.push(UsbKeyCode::LeftShift)},
        'V' => {code.push(UsbKeyCode::V); code.push(UsbKeyCode::LeftShift)},
        'W' => {code.push(UsbKeyCode::W); code.push(UsbKeyCode::LeftShift)},
        'X' => {code.push(UsbKeyCode::X); code.push(UsbKeyCode::LeftShift)},
        'Y' => {code.push(UsbKeyCode::Y); code.push(UsbKeyCode::LeftShift)},
        'Z' => {code.push(UsbKeyCode::Z); code.push(UsbKeyCode::LeftShift)},

        '0' => code.push(UsbKeyCode::Keyboard0),
        '1' => code.push(UsbKeyCode::Keyboard1),
        '2' => code.push(UsbKeyCode::Keyboard2),
        '3' => code.push(UsbKeyCode::Keyboard3),
        '4' => code.push(UsbKeyCode::Keyboard4),
        '5' => code.push(UsbKeyCode::Keyboard5),
        '6' => code.push(UsbKeyCode::Keyboard6),
        '7' => code.push(UsbKeyCode::Keyboard7),
        '8' => code.push(UsbKeyCode::Keyboard8),
        '9' => code.push(UsbKeyCode::Keyboard9),
        '!' => {code.push(UsbKeyCode::Keyboard1); code.push(UsbKeyCode::LeftShift)},
        '@' => {code.push(UsbKeyCode::Keyboard2); code.push(UsbKeyCode::LeftShift)},
        '#' => {code.push(UsbKeyCode::Keyboard3); code.push(UsbKeyCode::LeftShift)},
        '$' => {code.push(UsbKeyCode::Keyboard4); code.push(UsbKeyCode::LeftShift)},
        '%' => {code.push(UsbKeyCode::Keyboard5); code.push(UsbKeyCode::LeftShift)},
        '^' => {code.push(UsbKeyCode::Keyboard6); code.push(UsbKeyCode::LeftShift)},
        '&' => {code.push(UsbKeyCode::Keyboard7); code.push(UsbKeyCode::LeftShift)},
        '*' => {code.push(UsbKeyCode::Keyboard8); code.push(UsbKeyCode::LeftShift)},
        '(' => {code.push(UsbKeyCode::Keyboard9); code.push(UsbKeyCode::LeftShift)},
        ')' => {code.push(UsbKeyCode::Keyboard0); code.push(UsbKeyCode::LeftShift)},

        '-' => code.push(UsbKeyCode::Minus),
        '_' => {code.push(UsbKeyCode::Minus); code.push(UsbKeyCode::LeftShift)},
        '[' => code.push(UsbKeyCode::LeftBrace),
        '{' => {code.push(UsbKeyCode::LeftBrace); code.push(UsbKeyCode::LeftShift)},
        ']' => code.push(UsbKeyCode::RightBrace),
        '}' => {code.push(UsbKeyCode::RightBrace); code.push(UsbKeyCode::LeftShift)},
        '/' => code.push(UsbKeyCode::ForwardSlash),
        '?' => {code.push(UsbKeyCode::ForwardSlash); code.push(UsbKeyCode::LeftShift)},
        '\\' => code.push(UsbKeyCode::Backslash),
        '|' => {code.push(UsbKeyCode::Backslash); code.push(UsbKeyCode::LeftShift)},
        '=' => code.push(UsbKeyCode::Equal),
        '+' => {code.push(UsbKeyCode::Equal); code.push(UsbKeyCode::LeftShift)},
        '\'' => code.push(UsbKeyCode::Apostrophe),
        '"' => {code.push(UsbKeyCode::Apostrophe); code.push(UsbKeyCode::LeftShift)},
        ';' => {code.push(UsbKeyCode::Semicolon)},
        ':' => {code.push(UsbKeyCode::Semicolon); code.push(UsbKeyCode::LeftShift)},
        '`' => {code.push(UsbKeyCode::Grave)},
        '~' => {code.push(UsbKeyCode::Grave); code.push(UsbKeyCode::LeftShift)},

        '←' => code.push(UsbKeyCode::LeftArrow),
        '→' => code.push(UsbKeyCode::RightArrow),
        '↑' => code.push(UsbKeyCode::UpArrow),
        '↓' => code.push(UsbKeyCode::DownArrow),

        ',' => code.push(UsbKeyCode::Comma),
        '<' => {code.push(UsbKeyCode::Comma); code.push(UsbKeyCode::LeftShift)},
        '.' => code.push(UsbKeyCode::Dot),
        '>' => {code.push(UsbKeyCode::Dot); code.push(UsbKeyCode::LeftShift)},

        '\u{000d}' => {}, // ignore CR
        '\u{000a}' => code.push(UsbKeyCode::ReturnEnter), // turn LF ('\n') into enter
        ' ' => {code.push(UsbKeyCode::Space); },
        '\u{0008}' => code.push(UsbKeyCode::DeleteBackspace),
        _ => log::warn!("Ignoring unhandled character: {}", key),
    };
    code
}
#[allow(dead_code)]
/// auto-generated using tools/kbd_layout.py + `usb kbdtest` on device for dvorak layout on US101
pub fn char_to_hid_code_dvorak(key: char) -> Vec<UsbKeyCode> {
    let mut code = vec![];
    match key {
        ' ' => {code.push(UsbKeyCode::Space); },
        '!' => {code.push(UsbKeyCode::Keyboard1); code.push(UsbKeyCode::LeftShift); },
        '"' => {code.push(UsbKeyCode::Q); code.push(UsbKeyCode::LeftShift); },
        '#' => {code.push(UsbKeyCode::Keyboard3); code.push(UsbKeyCode::LeftShift); },
        '$' => {code.push(UsbKeyCode::Keyboard4); code.push(UsbKeyCode::LeftShift); },
        '%' => {code.push(UsbKeyCode::Keyboard5); code.push(UsbKeyCode::LeftShift); },
        '&' => {code.push(UsbKeyCode::Keyboard7); code.push(UsbKeyCode::LeftShift); },
        '\'' => {code.push(UsbKeyCode::Q); },
        '(' => {code.push(UsbKeyCode::Keyboard9); code.push(UsbKeyCode::LeftShift); },
        ')' => {code.push(UsbKeyCode::Keyboard0); code.push(UsbKeyCode::LeftShift); },
        '*' => {code.push(UsbKeyCode::Keyboard8); code.push(UsbKeyCode::LeftShift); },
        '+' => {code.push(UsbKeyCode::RightBrace); code.push(UsbKeyCode::LeftShift); },
        ',' => {code.push(UsbKeyCode::W); },
        '-' => {code.push(UsbKeyCode::Apostrophe); },
        '.' => {code.push(UsbKeyCode::E); },
        '/' => {code.push(UsbKeyCode::LeftBrace); },
        '0' => {code.push(UsbKeyCode::Keyboard0); },
        '1' => {code.push(UsbKeyCode::Keyboard1); },
        '2' => {code.push(UsbKeyCode::Keyboard2); },
        '3' => {code.push(UsbKeyCode::Keyboard3); },
        '4' => {code.push(UsbKeyCode::Keyboard4); },
        '5' => {code.push(UsbKeyCode::Keyboard5); },
        '6' => {code.push(UsbKeyCode::Keyboard6); },
        '7' => {code.push(UsbKeyCode::Keyboard7); },
        '8' => {code.push(UsbKeyCode::Keyboard8); },
        '9' => {code.push(UsbKeyCode::Keyboard9); },
        ':' => {code.push(UsbKeyCode::Z); code.push(UsbKeyCode::LeftShift); },
        ';' => {code.push(UsbKeyCode::Z); },
        '<' => {code.push(UsbKeyCode::W); code.push(UsbKeyCode::LeftShift); },
        '=' => {code.push(UsbKeyCode::RightBrace); },
        '>' => {code.push(UsbKeyCode::E); code.push(UsbKeyCode::LeftShift); },
        '?' => {code.push(UsbKeyCode::LeftBrace); code.push(UsbKeyCode::LeftShift); },
        '@' => {code.push(UsbKeyCode::Keyboard2); code.push(UsbKeyCode::LeftShift); },
        'A' => {code.push(UsbKeyCode::A); code.push(UsbKeyCode::LeftShift); },
        'B' => {code.push(UsbKeyCode::N); code.push(UsbKeyCode::LeftShift); },
        'C' => {code.push(UsbKeyCode::I); code.push(UsbKeyCode::LeftShift); },
        'D' => {code.push(UsbKeyCode::H); code.push(UsbKeyCode::LeftShift); },
        'E' => {code.push(UsbKeyCode::D); code.push(UsbKeyCode::LeftShift); },
        'F' => {code.push(UsbKeyCode::Y); code.push(UsbKeyCode::LeftShift); },
        'G' => {code.push(UsbKeyCode::U); code.push(UsbKeyCode::LeftShift); },
        'H' => {code.push(UsbKeyCode::J); code.push(UsbKeyCode::LeftShift); },
        'I' => {code.push(UsbKeyCode::G); code.push(UsbKeyCode::LeftShift); },
        'J' => {code.push(UsbKeyCode::C); code.push(UsbKeyCode::LeftShift); },
        'K' => {code.push(UsbKeyCode::V); code.push(UsbKeyCode::LeftShift); },
        'L' => {code.push(UsbKeyCode::P); code.push(UsbKeyCode::LeftShift); },
        'M' => {code.push(UsbKeyCode::M); code.push(UsbKeyCode::LeftShift); },
        'N' => {code.push(UsbKeyCode::L); code.push(UsbKeyCode::LeftShift); },
        'O' => {code.push(UsbKeyCode::S); code.push(UsbKeyCode::LeftShift); },
        'P' => {code.push(UsbKeyCode::R); code.push(UsbKeyCode::LeftShift); },
        'Q' => {code.push(UsbKeyCode::X); code.push(UsbKeyCode::LeftShift); },
        'R' => {code.push(UsbKeyCode::O); code.push(UsbKeyCode::LeftShift); },
        'S' => {code.push(UsbKeyCode::Semicolon); code.push(UsbKeyCode::LeftShift); },
        'T' => {code.push(UsbKeyCode::K); code.push(UsbKeyCode::LeftShift); },
        'U' => {code.push(UsbKeyCode::F); code.push(UsbKeyCode::LeftShift); },
        'V' => {code.push(UsbKeyCode::Dot); code.push(UsbKeyCode::LeftShift); },
        'W' => {code.push(UsbKeyCode::Comma); code.push(UsbKeyCode::LeftShift); },
        'X' => {code.push(UsbKeyCode::B); code.push(UsbKeyCode::LeftShift); },
        'Y' => {code.push(UsbKeyCode::T); code.push(UsbKeyCode::LeftShift); },
        'Z' => {code.push(UsbKeyCode::ForwardSlash); code.push(UsbKeyCode::LeftShift); },
        '[' => {code.push(UsbKeyCode::Minus); },
        '\\' => {code.push(UsbKeyCode::Backslash); },
        ']' => {code.push(UsbKeyCode::Equal); },
        '^' => {code.push(UsbKeyCode::Keyboard6); code.push(UsbKeyCode::LeftShift); },
        '_' => {code.push(UsbKeyCode::Apostrophe); code.push(UsbKeyCode::LeftShift); },
        '`' => {code.push(UsbKeyCode::Grave); },
        'a' => {code.push(UsbKeyCode::A); },
        'b' => {code.push(UsbKeyCode::N); },
        'c' => {code.push(UsbKeyCode::I); },
        'd' => {code.push(UsbKeyCode::H); },
        'e' => {code.push(UsbKeyCode::D); },
        'f' => {code.push(UsbKeyCode::Y); },
        'g' => {code.push(UsbKeyCode::U); },
        'h' => {code.push(UsbKeyCode::J); },
        'i' => {code.push(UsbKeyCode::G); },
        'j' => {code.push(UsbKeyCode::C); },
        'k' => {code.push(UsbKeyCode::V); },
        'l' => {code.push(UsbKeyCode::P); },
        'm' => {code.push(UsbKeyCode::M); },
        'n' => {code.push(UsbKeyCode::L); },
        'o' => {code.push(UsbKeyCode::S); },
        'p' => {code.push(UsbKeyCode::R); },
        'q' => {code.push(UsbKeyCode::X); },
        'r' => {code.push(UsbKeyCode::O); },
        's' => {code.push(UsbKeyCode::Semicolon); },
        't' => {code.push(UsbKeyCode::K); },
        'u' => {code.push(UsbKeyCode::F); },
        'v' => {code.push(UsbKeyCode::Dot); },
        'w' => {code.push(UsbKeyCode::Comma); },
        'x' => {code.push(UsbKeyCode::B); },
        'y' => {code.push(UsbKeyCode::T); },
        'z' => {code.push(UsbKeyCode::ForwardSlash); },
        '{' => {code.push(UsbKeyCode::Minus); code.push(UsbKeyCode::LeftShift); },
        '|' => {code.push(UsbKeyCode::Backslash); code.push(UsbKeyCode::LeftShift); },
        '}' => {code.push(UsbKeyCode::Equal); code.push(UsbKeyCode::LeftShift); },
        '~' => {code.push(UsbKeyCode::Grave); code.push(UsbKeyCode::LeftShift); },
        '\u{000d}' => {}, // ignore CR
        '\u{000a}' => code.push(UsbKeyCode::ReturnEnter), // turn LF ('\n') into enter
        '\u{0008}' => code.push(UsbKeyCode::DeleteBackspace),
        _ => log::warn!("Ignoring unhandled character: {}", key),
    };
    code
}
