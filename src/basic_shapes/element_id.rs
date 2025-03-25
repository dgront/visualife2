use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementID { pub id: [u8; 16], }

impl ElementID {
    /// Creates a new ElementID by adding a prefix to this ID.
    pub fn new_with_prefix(&self, prefix: &str) -> ElementID {
        let original = self.to_string();
        let combined = format!("{prefix}{original}");

        ElementID::from(combined)
    }
}

impl Hash for ElementID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// impl TryFrom<&str> for ElementID {
//     type Error = &'static str;
//
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let bytes = value.as_bytes();
//         if bytes.len() > 8 {
//             return Err("String too long");
//         }
//
//         let mut id = [0u8; 8];
//         id[..bytes.len()].copy_from_slice(bytes);
//         Ok(ElementID { id })
//     }
// }
//
// impl TryFrom<i32> for ElementID {
//     type Error = &'static str;
//
//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         ElementID::try_from(value.to_string().as_str())
//     }
// }

impl From<&str> for ElementID {
    fn from(value: &str) -> Self {
        let bytes = value.as_bytes();
        let mut id = [0u8; 16];
        let len = bytes.len().min(16);
        id[..len].copy_from_slice(&bytes[..len]);
        ElementID { id }
    }
}

impl From<i32> for ElementID {
    fn from(value: i32) -> Self {
        ElementID::from(value.to_string().as_str())
    }
}

impl From<String> for ElementID {
    fn from(value: String) -> Self {
        ElementID::from(value.as_str())
    }
}

impl From<&String> for ElementID {
    fn from(value: &String) -> Self {
        ElementID::from(value.as_str())
    }
}

impl fmt::Display for ElementID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Trim trailing null bytes and convert to &str
        let end = self.id.iter().position(|&b| b == 0).unwrap_or(8);
        let valid_bytes = &self.id[..end];
        match std::str::from_utf8(valid_bytes) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "<invalid utf-8>"),
        }
    }
}