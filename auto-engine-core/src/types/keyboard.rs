use enigo::Key;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum KeyBoardKeyMode {
    #[default]
    Click,
    Type,
    Down,
    Up,
}

pub trait ToKeyMode {
    fn to_key_mode(&self) -> KeyBoardKeyMode;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct KeyBoardParams {
    #[serde(default)]
    pub mode: KeyBoardKeyMode,
    pub key: KeyCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    Win,
    Alt,

    Command, // ⌘  U+2318
    Option,  // ⌥  U+2325
    Control, // ⌃  U+2303
    Shift,   // ⇧  U+21E7
}

pub trait ToKeyCode {
    fn to_key_code(&self) -> Option<KeyCode>;
}

macro_rules! map_keys {
    (
        $( $keycode:ident => $enigo_key:expr ),* $(,)?
    ) => {
        impl KeyCode {
            pub fn to_enigo_key(&self) -> enigo::Key {
                match self {
                    $(
                        KeyCode::$keycode => $enigo_key,
                    )*
                }
            }
        }
        impl ToKeyCode for String {
            fn to_key_code(&self) -> Option<KeyCode> {
                match self.to_uppercase().as_str() {
                    $(
                        stringify!($keycode) => Some(KeyCode::$keycode),
                    )*
                    _ => None,
                }
            }
        }
    };
}

map_keys! {
    A => Key::Unicode('A'),
    B => Key::Unicode('B'),
    C => Key::Unicode('C'),
    D => Key::Unicode('D'),
    E => Key::Unicode('E'),
    F => Key::Unicode('F'),
    G => Key::Unicode('G'),
    H => Key::Unicode('H'),
    I => Key::Unicode('I'),
    J => Key::Unicode('J'),
    K => Key::Unicode('K'),
    L => Key::Unicode('L'),
    M => Key::Unicode('M'),
    N => Key::Unicode('N'),
    O => Key::Unicode('O'),
    P => Key::Unicode('P'),
    Q => Key::Unicode('Q'),
    R => Key::Unicode('R'),
    S => Key::Unicode('S'),
    T => Key::Unicode('T'),
    U => Key::Unicode('U'),
    V => Key::Unicode('V'),
    W => Key::Unicode('W'),
    X => Key::Unicode('X'),
    Y => Key::Unicode('Y'),
    Z => Key::Unicode('Z'),

    Num0 => Key::Unicode('0'),
    Num1 => Key::Unicode('1'),
    Num2 => Key::Unicode('2'),
    Num3 => Key::Unicode('3'),
    Num4 => Key::Unicode('4'),
    Num5 => Key::Unicode('5'),
    Num6 => Key::Unicode('6'),
    Num7 => Key::Unicode('7'),
    Num8 => Key::Unicode('8'),
    Num9 => Key::Unicode('9'),

    F1 => Key::F1,
    F2 => Key::F2,
    F3 => Key::F3,
    F4 => Key::F4,
    F5 => Key::F5,
    F6 => Key::F6,
    F7 => Key::F7,
    F8 => Key::F8,
    F9 => Key::F9,
    F10 => Key::F10,
    F11 => Key::F11,
    F12 => Key::F12,

    Enter => Key::Return,
    Escape => Key::Escape,
    Backspace => Key::Backspace,
    Tab => Key::Tab,
    Space => Key::Space,

    Command => Key::Meta,
    Win => Key::Meta,
    Option => Key::Alt,
    Alt => Key::Alt,
    Control => Key::Control,
    Shift => Key::Shift,
}
