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
                TableErrorSource::AttributeNotFound => "the given attribute was not found",
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

// TODO: Create a clear way to view a database
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

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Seperator character that seperates cells vertically
        let vertical_seperator = "|";
        // Seperator character that seperates cells horizontally
        let horizontal_seperator = "_";
        // Character that will be used for blank space
        let blank_seperator = " ";
        // Spacing on each side of the attribute
        let spacing = 5;

        let draw_horizontal_seperator = |f: &mut fmt::Formatter<'_>| {
            writeln!(f).unwrap();

            for attribute in self.table.keys() {
                write!(
                    f,
                    "{}",
                    horizontal_seperator.repeat(
                        attribute.to_string().len() + vertical_seperator.len() + (spacing * 2)
                    )
                )
                .unwrap();
            }

            writeln!(f).unwrap();
        };

        for attribute in self.table.keys() {
            write!(
                f,
                "{}{}{}{}",
                blank_seperator.repeat(spacing),
                attribute,
                blank_seperator.repeat(spacing),
                vertical_seperator
            )
            .unwrap();
        }

        draw_horizontal_seperator(f);

        if !self.table.values().next().unwrap().is_empty() {
            for i in 0..self.table.values().next().unwrap().len() {
                for (attribute, column) in &self.table {
                    let cell_space = (spacing * 2) + attribute.to_string().len();

                    write!(
                        f,
                        "{}",
                        match &column[i] {
                            Some(value) => {
                                let mut value = value.to_string();

                                if value.len() > cell_space {
                                    value.truncate(value.len() - cell_space - 3);

                                    format!("{}...{}", value, vertical_seperator)
                                } else {
                                    format!(
                                        "{}{}{}",
                                        value,
                                        blank_seperator.repeat(cell_space - value.len()),
                                        vertical_seperator
                                    )
                                }
                            }
                            None => format!(
                                "{}{}",
                                blank_seperator.repeat(cell_space),
                                vertical_seperator
                            ),
                        }
                    )
                    .unwrap();
                }

                draw_horizontal_seperator(f);
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
            return Err(TableError {
                source: TableErrorSource::IncorrectRowSize,
            });
        }
        // Make sure they are not trying to insert data under non-existant attributes
        for (attribute, _) in &row {
            if !self.table.contains_key(attribute) {
                return Err(TableError {
                    source: TableErrorSource::AttributeNotFound,
                });
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
