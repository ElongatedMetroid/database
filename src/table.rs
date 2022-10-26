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

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for attribute in &self.attributes {
            write!(f, "{:?}\t", attribute).unwrap();
        }

        writeln!(f).unwrap();

        for row in &self.data {
            for cell in row {
                write!(f, "{:?}", cell).unwrap();
            }

            writeln!(f).unwrap();
        }

        Ok(())
    }
}

impl Table {
    pub(crate) fn new(name: Data, attributes: Vec<Data>) -> Self {
        Self {
            name,
            attributes,
            data: Vec::new(),
        }
    }

    pub fn push_row(&mut self, row: Vec<Option<Data>>) -> Result<usize, TableError> {
        if row.len() != self.attributes.len() {
            return Err(TableError {
                source: TableErrorSource::IncorrectRowSize,
            });
        }

        self.data.push(row);

        Ok(self.data.len() - 1)
    }

    pub fn get_cell(
        &mut self,
        attribute: Data,
        row_id: usize,
    ) -> Result<&mut Option<Data>, TableError> {
        for (x, a) in self.attributes.iter().enumerate() {
            if *a == attribute {
                return Ok(&mut self.data[row_id][x]);
            }
        }

        Err(TableError {
            source: TableErrorSource::AttributeNotFound,
        })
    }
}
