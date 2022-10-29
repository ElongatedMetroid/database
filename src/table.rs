use std::{collections::BTreeMap, error::Error, fmt};

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
                    "row does not have enough elements to be pushed (note: attributes can be set to None)",
                TableErrorSource::AttributeNotFound => 
                    "the given attribute was not found",
            }
        )
    }
}

impl Error for TableErrorSource {}

pub struct Table {
    /// Name of the table
    pub(crate) name: Data,
    /// HashMap of HashMaps with the key to those hashmaps being the attribute
    table: BTreeMap<Data, Vec<Option<Data>>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TABLE: {:?}\nAttributes: ", self.name).unwrap();
        for attribute in self.table.keys() {
            write!(f, "{:?}\t", attribute).unwrap();
        }

        writeln!(f).unwrap();

        for row in self.table.values() {
            for cell in row {
                write!(f, "\t{:?}", cell).unwrap();
            }
        }

        Ok(())
    }
}

impl Table {
    pub(crate) fn new(name: Data, attributes: Vec<Data>) -> Self {
        let mut table = BTreeMap::new();

        for attribute in attributes {
            table.insert(attribute, Vec::new());
        }

        Self { name, table }
    }

    pub fn push_row(&mut self, row: Vec<(Data, Option<Data>)>) -> Result<usize, TableError> {
        // Make sure they are setting all attributes
        if self.table.keys().len() - row.len() != 0 {
            return Err(TableError { source: TableErrorSource::IncorrectRowSize })
        }
        // Make sure they are not trying to insert data under non-existant attributes
        for (attribute, _) in &row {
            if !self.table.contains_key(attribute) {
                return Err(TableError { source: TableErrorSource::AttributeNotFound });
            }
        }

        // Push each Option<Data> into the column they belong
        for (attribute, data) in row { 
            self.table
                .entry(attribute)
                .and_modify(|column| column.push(data));
        }

        Ok(self.table.values().next().unwrap().len() - 1)
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
