use std::{error::Error, fmt};

use crate::{
    data::Data,
    database::{Database, DatabaseError},
    table::Table,
};

#[derive(Debug)]
pub struct DatabaseCommandError {
    source: DatabaseCommandErrorSource,
}

impl fmt::Display for DatabaseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source().unwrap())
    }
}

impl Error for DatabaseCommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
enum DatabaseCommandErrorSource {
    KeywordNotProvided,
    KeywordNotFound,
    CommandIsNotDefault,
    ArgumentNotProvided(String),
}

impl fmt::Display for DatabaseCommandErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                DatabaseCommandErrorSource::KeywordNotProvided =>
                    String::from("a keyword was not provided in the command."),
                DatabaseCommandErrorSource::KeywordNotFound =>
                    String::from("the keyword provdied was not found"),
                DatabaseCommandErrorSource::CommandIsNotDefault => String::from(
                    "the provided command is not part of the default commands execute method"
                ),
                DatabaseCommandErrorSource::ArgumentNotProvided(arg_name) =>
                    format!("`{}` argument not provided", arg_name),
            }
        )
    }
}

impl Error for DatabaseCommandErrorSource {}

pub trait DatabaseCommand<T>: fmt::Debug {
    fn arg_parser(&mut self, args: Vec<&str>) -> Result<(), DatabaseCommandError>;
    fn execute<'a>(&'a mut self, db: &'a mut Database) -> Result<&mut T, DatabaseError>;
}

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

pub trait CommandParser<T> where NewTable: DatabaseCommand<T> {
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
    fn parse(&self, input: &str) -> Result<Box<dyn DatabaseCommand<T>>, DatabaseCommandError> {
        let mut input_words = input.split_whitespace();

        let mut command = match self.keyword_parser(&match input_words.next() {
            Some(keyword) => keyword,
            None => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::KeywordNotProvided,
                })
            }
        }) {
            Ok(command) => command,
            Err(error) => return Err(error),
        };

        match command.arg_parser(input_words.collect()) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }

        Ok(command)
    }
}

pub struct DefaultCommandParser;

impl<T> CommandParser<T> for DefaultCommandParser where NewTable: DatabaseCommand<T> {}
