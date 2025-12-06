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
                use convert_case::{Case, Casing};
                let key = self.to_case(Case::Pascal);

                match key.as_str() {
                    $(
                        stringify!($keycode) => Some(KeyCode::$keycode),
                    )*
                    _ => None,
                }
            }
        }
    };
}

#[cfg(target_os = "windows")]
macro_rules! key_for {
    ($ch:ident) => {
        Key::$ch
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! key_for {
    ($ch:ident) => {
        Key::Unicode(stringify!($ch).chars().next().unwrap())
    };
}

map_keys! {
    A => key_for!(A),
    B => key_for!(B),
    C => key_for!(C),
    D => key_for!(D),
    E => key_for!(E),
    F => key_for!(F),
    G => key_for!(G),
    H => key_for!(H),
    I => key_for!(I),
    J => key_for!(J),
    K => key_for!(K),
    L => key_for!(L),
    M => key_for!(M),
    N => key_for!(N),
    O => key_for!(O),
    P => key_for!(P),
    Q => key_for!(Q),
    R => key_for!(R),
    S => key_for!(S),
    T => key_for!(T),
    U => key_for!(U),
    V => key_for!(V),
    W => key_for!(W),
    X => key_for!(X),
    Y => key_for!(Y),
    Z => key_for!(Z),

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ToKeyCode;
    use enigo::Key;

    #[test]
    fn parse_key_case_insensitive() {
        struct Case {
            raw: &'static str,
            expected: Option<KeyCode>,
        }

        let cases = [
            Case {
                raw: "command",
                expected: Some(KeyCode::Command),
            },
            Case {
                raw: "COMMAND",
                expected: Some(KeyCode::Command),
            },
            Case {
                raw: "Num1",
                expected: Some(KeyCode::Num1),
            },
            Case {
                raw: "f1",
                expected: Some(KeyCode::F1),
            },
            Case {
                raw: "not-a-key",
                expected: None,
            },
        ];

        for case in cases {
            let parsed = case.raw.to_string().to_key_code();
            let matches = match (&parsed, &case.expected) {
                (Some(got), Some(expected)) => {
                    std::mem::discriminant(got) == std::mem::discriminant(expected)
                }
                (None, None) => true,
                _ => false,
            };

            assert!(
                matches,
                "input `{}` parsed to {:?}, expected {:?}",
                case.raw, parsed, case.expected
            );
        }
    }

    #[test]
    fn to_enigo_key_mapping() {
        #[cfg(target_os = "windows")]
        assert_eq!(KeyCode::A.to_enigo_key(), Key::A);
        #[cfg(not(target_os = "windows"))]
        assert_eq!(KeyCode::A.to_enigo_key(), Key::Unicode('A'));

        assert_eq!(KeyCode::Num3.to_enigo_key(), Key::Unicode('3'));
        assert_eq!(KeyCode::Enter.to_enigo_key(), Key::Return);
        assert_eq!(KeyCode::Command.to_enigo_key(), Key::Meta);
        assert_eq!(KeyCode::Option.to_enigo_key(), Key::Alt);
        assert_eq!(KeyCode::Control.to_enigo_key(), Key::Control);
        assert_eq!(KeyCode::Shift.to_enigo_key(), Key::Shift);
    }
}
