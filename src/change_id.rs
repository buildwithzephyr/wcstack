use crate::errors::WcStackError;
use reverse_hex::{to_forward_hex, to_reverse_hex};
use std::fmt::Display;

/// Map between standard hex digits and the "reverse" hex digits used by Jujutsu's ChangeIDs
/// [Jujutsu docs](https://martinvonz.github.io/jj/v0.14.0/glossary/#change-id)
mod reverse_hex {
    fn to_reverse_hex_digit(b: u8) -> Option<u8> {
        let value = match b {
            b'0'..=b'9' => b - b'0',
            b'A'..=b'F' => b - b'A' + 10,
            b'a'..=b'f' => b - b'a' + 10,
            _ => return None,
        };
        Some(b'z' - value)
    }

    fn to_forward_hex_digit(b: u8) -> Option<u8> {
        let value = match b {
            b'k'..=b'z' => b'z' - b,
            b'K'..=b'Z' => b'Z' - b,
            _ => return None,
        };
        if value < 10 {
            Some(b'0' + value)
        } else {
            Some(b'a' + value - 10)
        }
    }

    pub fn to_forward_hex(reverse_hex: &str) -> Option<String> {
        reverse_hex
            .bytes()
            .map(|b| to_forward_hex_digit(b).map(char::from))
            .collect()
    }

    pub fn to_reverse_hex(forward_hex: &str) -> Option<String> {
        forward_hex
            .bytes()
            .map(|b| to_reverse_hex_digit(b).map(char::from))
            .collect()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_valid_forward_hex() {
            assert_eq!(
                to_forward_hex("zyxwvutsrqponmlkPONMLK"),
                Some("0123456789abcdefabcdef".to_string())
            );
        }

        #[test]
        fn test_invalid_forward_hex() {
            assert_eq!(to_forward_hex("zyxwvuJsrqponmlkPONMLK"), None);
        }

        #[test]
        fn test_inverse_operations_when_lowercase() {
            assert_eq!(
                to_reverse_hex(&to_forward_hex("zyxwvutsrqponmlkponmlk").unwrap()),
                Some("zyxwvutsrqponmlkponmlk".to_owned())
            );
            assert_eq!(
                to_forward_hex(&to_reverse_hex("0123456789abcdefabcdef").unwrap()),
                Some("0123456789abcdefabcdef".to_owned())
            );
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChangeId(pub Vec<u8>);

impl TryFrom<&str> for ChangeId {
    type Error = WcStackError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes: Vec<u8> =
            hex::decode(to_forward_hex(value).ok_or(WcStackError::BadChangeId(value.to_owned()))?)
                // Bad values will be caught in to_forward_hex; if it outputs Some(), then hex will decode
                .unwrap();
        Ok(Self(bytes))
    }
}

impl Display for ChangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_reverse_hex(&hex::encode(&self.0)).unwrap())
    }
}
