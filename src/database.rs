use core::fmt;
use std::error::Error;

use crate::table::{Table, Data};

#[derive(Debug)]
pub struct DatabaseError {
    source: DatabaseErrorSource,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database")
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
enum DatabaseErrorSource {
    NameAlreadyExists,
    NameDoesNotExist,
}

impl fmt::Display for DatabaseErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                DatabaseErrorSource::NameAlreadyExists => "a table with that name already exists",
                DatabaseErrorSource::NameDoesNotExist => "a table with that name does not exist",
            }
        )
    }
}

impl Error for DatabaseErrorSource {}

pub struct Database {
    tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
        }
    }

    pub fn add_table(&mut self, name: Data, attributes: Vec<Data>) -> Result<&mut Table, DatabaseError> {
        // If a table with that name already exists return an error
        for table in &self.tables {
            if table.name == name {
                return Err(DatabaseError { source: DatabaseErrorSource::NameAlreadyExists });
            }
        }

        self.tables.push(Table::new(name, attributes));
        let element = self.tables.len() - 1;
        Ok(&mut self.tables[element])
    }

    // pub fn get_table(&mut self, name: Data) -> Result<&mut Table, DatabaseError> {

    // }
}