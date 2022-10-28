use std::{error::Error, fmt, collections::BTreeMap};

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
    AttributeNotFound,
}

impl fmt::Display for TableErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TableErrorSource::IncorrectRowSize =>
                    "row is not the correct size to be pushed to the table",
                TableErrorSource::AttributeNotFound => "the given attribute was not found",
            }
        )
    }
}

impl Error for TableErrorSource {}

#[derive(Debug)]
pub struct Table {
    /// Name of the table
    pub(crate) name: Data,
    /// HashMap of HashMaps with the key to those hashmaps being the attribute
    table: BTreeMap<Data, Vec<Option<Data>>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

// impl fmt::Debug for Table {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for attribute in self.table.keys() {
//             write!(f, "{:?}\t", attribute).unwrap();
//         }

//         writeln!(f).unwrap();

//         for row in self.table.values() {
//             for cell in row {
//                 write!(f, "{:?}", cell).unwrap();
//             }

//             writeln!(f).unwrap();
//         }

//         Ok(())
//     }
// }

impl Table {
    pub(crate) fn new(name: Data, attributes: Vec<Data>) -> Self {
        let mut table = BTreeMap::new();

        for attribute in attributes {
            table.insert(attribute, Vec::new());
        }

        Self {
            name,
            table,
        }
    }

    pub fn push_row(&mut self, mut row: Vec<Option<Data>>) -> usize {
        for _ in 0..(row.len() - self.table.keys().len()) {
            row.push(None);
        }

        for (table_cell, cell) in self.table.values_mut().zip(row) {
            table_cell.push(cell);
        }

        0
    }

    pub fn get_cell(
        &mut self,
        attribute: &Data,
        row_id: usize,
    ) -> Result<&mut Option<Data>, TableError> {
        for (atr, data) in &mut self.table {
            if atr == attribute {
                return Ok(&mut data[row_id]);
            }
        }

        Err(TableError {
            source: TableErrorSource::AttributeNotFound,
        })
    }
}
