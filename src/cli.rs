use std::{error::Error, fmt};

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
}

impl fmt::Display for DatabaseCommandErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                DatabaseCommandErrorSource::KeywordNotProvided =>
                    "a keyword was not provided in the command.",
                DatabaseCommandErrorSource::KeywordNotFound =>
                    "the keyword provdied was not found",
            }
        )
    }
}

impl Error for DatabaseCommandErrorSource {}

pub trait DatabaseCommand {
    fn set_args(&mut self, args: Vec<&str>) -> Result<(), DatabaseCommandError>;
}

struct NewTable {}

impl DatabaseCommand for NewTable {
    fn set_args(&mut self, args: Vec<&str>) -> Result<(), DatabaseCommandError> {
        todo!()
    }
}

pub trait CommandParser {
    fn keyword_parser(&self, keyword: &str) -> Result<Box<dyn DatabaseCommand>, DatabaseCommandError> {
        match keyword {
            "NewTable" => Ok(Box::new(NewTable {})) ,
            _ => {
                return Err(DatabaseCommandError {
                    source: DatabaseCommandErrorSource::KeywordNotFound,
                })
            }
        }
    }
}

pub struct DefaultCommandParser {}

impl CommandParser for DefaultCommandParser {}

impl DefaultCommandParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(
        &self,
        input: &str,
    ) -> Result<Box<dyn DatabaseCommand>, DatabaseCommandError> {
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

        match command.set_args(input_words.collect()) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }

        Ok(command)
    }
}
