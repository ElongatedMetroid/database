use crate::{
    data::Data,
    database::{Database, DatabaseError},
    table::Table,
};

use super::{CommandParser, DatabaseCommand, DatabaseCommandError, DatabaseCommandErrorSource};

pub struct NewTable {
    /// This field is an Option<Data> so we can move the data instead of cloning it
    name: Option<Data>,
    attributes: Option<Vec<Data>>,
}

impl DatabaseCommand<Table> for NewTable {
    fn arg_parser(&mut self, args: Vec<&str>) -> Result<(), DatabaseCommandError> {
        // First arg is the name of the table, the rest of the args are attributes

        // Get the name argument into our name field
        self.name = Some(match args.get(0) {
            Some(name) => Data::convert_to_correct_data_from_str(*name),
            None => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::ArgumentNotProvided(String::from(
                        "table name",
                    )),
                })
            }
        });

        // Check if attribute field(s) were provided
        if args.get(1).is_none() {
            return Err(DatabaseCommandError {
                source: DatabaseCommandErrorSource::ArgumentNotProvided(String::from("attributes")),
            });
        }

        self.attributes = Some(Vec::new());
        // Convert the attribute args into the correct Data enum varients
        for arg in &args[1..] {
            if let Some(attributes) = &mut self.attributes {
                attributes.push(Data::convert_to_correct_data_from_str(*arg));
            }

            continue;
        }

        Ok(())
    }

    fn execute<'a>(&mut self, db: &'a mut Database) -> Result<&'a mut Table, DatabaseError> {
        // Since the name and attributes fields were in an option we can move them instead of cloning
        db.add_table(self.name.take().unwrap(), self.attributes.take().unwrap())
    }
}

impl NewTable {
    pub fn new() -> Self {
        Self {
            name: None,
            attributes: None,
        }
    }
}

pub struct GetTable {
    name: Data,
}

impl DatabaseCommand<Table> for GetTable {
    fn arg_parser(&mut self, args: Vec<&str>) -> Result<(), DatabaseCommandError> {
        // The first (and only) argument is the name of the table
        self.name = match args.get(0) {
            Some(name) => Data::convert_to_correct_data_from_str(*name),
            None => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::ArgumentNotProvided(String::from(
                        "table name",
                    )),
                });
            }
        };

        Ok(())
    }

    fn execute<'a>(&'a mut self, db: &'a mut Database) -> Result<&mut Table, DatabaseError> {
        db.get_table(&self.name)
    }
}

impl GetTable {
    pub fn new() -> Self {
        Self {
            name: Data::from(0),
        }
    }
}

pub struct DefaultCommandParser;

impl<T> CommandParser<T> for DefaultCommandParser
where
    NewTable: DatabaseCommand<T>,
    GetTable: DatabaseCommand<T>,
{
    fn keyword_parser(
        &self,
        keyword: &str,
    ) -> Result<Box<dyn DatabaseCommand<T>>, DatabaseCommandError> {
        match keyword {
            "NewTable" => Ok(Box::new(NewTable::new())),
            "GetTable" => Ok(Box::new(GetTable::new())),
            _ => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::KeywordNotFound,
                })
            }
        }
    }
}
