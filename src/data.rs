use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Data {
    /// String of characters
    Text(String),
    /// Single character
    Character(char),
    /// Number without decimal values
    Integer(i64),
    /// True or false value
    Boolean(bool),
    /// Vector of bytes
    Blob(Vec<u8>),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            Data::Text(value) => value.clone(),
            Data::Character(value) => value.to_string(),
            Data::Integer(value) => value.to_string(),
            Data::Boolean(value) => value.to_string(),
            Data::Blob(_) => String::from("-- blob --"),
        };

        write!(f, "{}", data)
    }
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Data::Text(String::from(value))
    }
}

impl From<String> for Data {
    fn from(value: String) -> Self {
        Data::Text(value)
    }
}

impl From<char> for Data {
    fn from(value: char) -> Self {
        Data::Character(value)
    }
}

impl From<i64> for Data {
    fn from(value: i64) -> Self {
        Data::Integer(value)
    }
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::Boolean(value)
    }
}

impl From<Vec<u8>> for Data {
    fn from(value: Vec<u8>) -> Self {
        Data::Blob(value)
    }
}
