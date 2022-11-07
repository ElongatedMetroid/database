use crate::{data::Data, table::Table, database::{Database, DatabaseError}};

use super::{DatabaseCommand, DatabaseCommandError, DatabaseCommandErrorSource, CommandParser};

#[derive(Debug)]
pub struct NewTable {
    name: Data,
    attributes: Vec<Data>,
}

impl DatabaseCommand<Table> for NewTable {
    fn arg_parser(&mut self, args: Vec<&str>) -> Result<(), DatabaseCommandError> {
        // First arg is the name of the table, the rest of the args are attributes

        // Get the name argument into our name field
        self.name = match args.get(0) {
            Some(name) => Data::from(*name),
            None => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::ArgumentNotProvided(String::from(
                        "table name",
                    )),
                })
            }
        };

        // Check if attribute field(s) were provided
        if args.get(1).is_none() {
            return Err(DatabaseCommandError {
                source: DatabaseCommandErrorSource::ArgumentNotProvided(String::from("attributes")),
            });
        }

        // Convert the attribute args into the correct Data enum varients
        for arg in &args[1..] {
            match arg.parse::<i64>() {
                Ok(attribute) => {
                    self.attributes.push(Data::from(attribute));
                    continue;
                }
                Err(_) => (),
            }
            match arg.parse::<char>() {
                Ok(attribute) => {
                    self.attributes.push(Data::from(attribute));
                    continue;
                }
                Err(_) => (),
            }
            match arg.parse::<bool>() {
                Ok(attribute) => {
                    self.attributes.push(Data::from(attribute));
                    continue;
                }
                Err(_) => (),
            }

            self.attributes.push(Data::from(*arg));
        }

        Ok(())
    }

    fn execute<'a>(&mut self, db: &'a mut Database) -> Result<&'a mut Table, DatabaseError> {
        db.add_table(self.name.clone(), self.attributes.clone())
    }
}

pub struct DefaultCommandParser;

impl<T> CommandParser<T> for DefaultCommandParser where NewTable: DatabaseCommand<T> {
    fn keyword_parser(
        &self,
        keyword: &str,
    ) -> Result<Box<dyn DatabaseCommand<T>>, DatabaseCommandError> {
        match keyword {
            "NewTable" => Ok(Box::new(NewTable {
                name: Data::from(String::new()),
                attributes: Vec::new(),
            })),
            _ => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::KeywordNotFound,
                })
            }
        }
    }
}
