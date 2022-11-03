use std::{error::Error, fmt, ops::Residual};

use crate::{database::{Database, DatabaseError}, table::{Data, Table}};

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
                    format!("{} argument not provided", arg_name),
            }
        )
    }
}

impl Error for DatabaseCommandErrorSource {}

pub trait DatabaseCommand: fmt::Debug {
    fn arg_parser(&mut self, args: Vec<&str>) -> Result<(), DatabaseCommandError>;
    fn execute(&mut self, db: &mut Database) -> Result<(), DatabaseError>;
}

#[derive(Debug)]
pub struct NewTable {
    // TODO: MAKE
    name: String,
    attributes: Vec<Data>,
}

impl DatabaseCommand for NewTable {
    fn arg_parser(&mut self, args: Vec<&str>) -> Result<(), DatabaseCommandError> {
        // First arg is the name of the table, the rest of the args are attributes

        // Get the name argument into our name field
        self.name = match args.get(0) {
            Some(name) => name,
            None => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::ArgumentNotProvided(String::from(
                        "table name",
                    )),
                })
            }
        }
        .to_string();

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
                    self.attributes.push(Data::Integer(attribute));
                    continue;
                }
                Err(_) => (),
            }
            match arg.parse::<char>() {
                Ok(attribute) => {
                    self.attributes.push(Data::Character(attribute));
                    continue;
                }
                Err(_) => (),
            }
            match arg.parse::<bool>() {
                Ok(attribute) => {
                    self.attributes.push(Data::Boolean(attribute));
                    continue;
                }
                Err(_) => (),
            }

            self.attributes.push(Data::Text(arg.to_string()));
        }

        Ok(())
    }

    fn execute(&mut self, db: &mut Database) -> Result<(), DatabaseError> {
        db.add_table(Data::Text(self.name.clone()), self.attributes.clone())
    }
}

pub trait CommandParser<T> {
    fn keyword_parser(
        &self,
        keyword: &str,
    ) -> Result<Box<dyn DatabaseCommand>, DatabaseCommandError> {
        match keyword {
            "NewTable" => Ok(Box::new(NewTable {
                name: String::new(),
                attributes: Vec::new(),
            })),
            _ => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::KeywordNotFound,
                })
            }
        }
    }
    fn parse(&self, input: &str) -> Result<Box<dyn DatabaseCommand>, DatabaseCommandError> {
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

pub struct DefaultCommandParser {}

impl<T> CommandParser<T> for DefaultCommandParser {}

impl DefaultCommandParser {
    pub fn new() -> Self {
        Self {}
    }
}