use core::fmt;
use std::error::Error;

use crate::{
    table::Table,
    data::Data,
};

#[derive(Debug)]
pub struct DatabaseError {
    source: DatabaseErrorSource,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source().unwrap())
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
    config: DatabaseConfig,
    tables: Vec<Table>,
}

pub enum DatabaseStorageType {
    /// The database is completly stored in memory
    Memory,
    /// The database is completly stored on disk (to the extent possible)
    Storage(String),
    /// The database is both stored on disk and memory
    MemoryAndStorage(String),
    /// The database is completly stored on disk, with the most recently/most used items stored
    /// inside memory
    Smart(String),
}

pub struct DatabaseConfig {
    auto_save: bool,
    storage_type: DatabaseStorageType,
}

impl DatabaseConfig {
    pub fn new(auto_save: bool, storage_type: DatabaseStorageType) -> Self {
        Self {
            auto_save,
            storage_type,
        }
    }
}

impl Database {
    /// Create a new empty database
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            config,
            tables: Vec::new(),
        }
    }

    /// Add a new table to the database with the specified attributes.
    /// This will fail if you try to create a database with a name that is already in use by another database.
    pub fn add_table(
        &mut self,
        name: Data,
        attributes: Vec<Data>,
    ) -> Result<&mut Table, DatabaseError> {
        // If a table with that name already exists return an error
        for table in &self.tables {
            if table.name == name {
                return Err(DatabaseError {
                    source: DatabaseErrorSource::NameAlreadyExists,
                });
            }
        }

        self.tables.push(Table::new(name, attributes));
        let element = self.tables.len() - 1;
        Ok(&mut self.tables[element])
    }

    /// Get a mutable reference to the table with a specified name.
    /// This will fail if a table with the name does not exist.
    pub fn get_table(&mut self, name: Data) -> Result<&mut Table, DatabaseError> {
        for table in &mut self.tables {
            if table.name == name {
                return Ok(table);
            }
        }

        Err(DatabaseError {
            source: DatabaseErrorSource::NameDoesNotExist,
        })
    }
}
