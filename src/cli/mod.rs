pub mod default;

use std::{error::Error, fmt};

use crate::database::{Database, DatabaseError};

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

pub trait CommandParser<T> {
    fn keyword_parser(
        &self,
        keyword: &str,
    ) -> Result<Box<dyn DatabaseCommand<T>>, DatabaseCommandError>;
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
