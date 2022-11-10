use std::io;

use database::{
    command::{default::DefaultCommandParser, CommandParser},
    database::{Database, DatabaseConfig, DatabaseStorageType},
};

fn main() {
    let mut db = Database::new(DatabaseConfig::new(true, DatabaseStorageType::Memory));

    println!("--- Bank Managment System ---\nEnter a command:");

    let mut buf = String::new();

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let mut command = DefaultCommandParser.parse(&buf).unwrap();

        //let result = db.command(&mut command);

        //println!("{}", result.unwrap());
        // Idea,
        // GetTable users;PushRow
    }
}
