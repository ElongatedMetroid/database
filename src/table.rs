use std::{error::Error, fmt};

#[derive(Debug)]
pub struct TableError {
    source: TableErrorSource,
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database")
    }
}

impl Error for TableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
enum TableErrorSource {
    IncorrectRowSize,
}

impl fmt::Display for TableErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TableErrorSource::IncorrectRowSize => "row is not the correct size to be pushed to the table",
            }
        )
    }
}

impl Error for TableErrorSource {}

#[derive(Debug)]
pub struct Table {
    /// Name of the table
    pub(crate) name: Data,
    /// Attributes for each column
    attributes: Vec<Data>,
    /// 2D vector of Data's
    data: Vec<Vec<Option<Data>>>,
}

#[derive(Debug, PartialEq)]
pub enum Data {
    /// String of characters
    Text(String),
    /// Single character
    Character(char),
    /// Number without decimal values
    Integer(i64),
    /// Number with decimal value
    Decimal(f64),
    /// True or false value
    Boolean(bool),
    /// Vector of bytes
    Blob(Vec<u8>),
}

impl Table {
    pub(crate) fn new(name: Data, attributes: Vec<Data>) -> Self {
        Self {
            name,
            attributes,
            data: Vec::new(), 
        }
    }

    pub fn push_row(&mut self, row: Vec<Option<Data>>) -> Result<&mut Table, TableError> {
        if row.len() != self.attributes.len() {
            return Err(TableError{ source: TableErrorSource::IncorrectRowSize });
        }

        self.data.push(row);

        Ok(self)
    }
}