use std::io;

use database::{
    cli::{CommandParser, DefaultCommandParser},
    data::Data,
    database::{Database, DatabaseConfig, DatabaseStorageType},
};

fn main() {
    let mut db = Database::new(DatabaseConfig::new(true, DatabaseStorageType::Memory));

    println!("--- Bank Managment System ---\nEnter a command:");

    let mut buf = String::new();

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let command = DefaultCommandParser.parse(&buf).unwrap();

        println!("{:#?}", command);

        db.command(command);

        println!(
            "{}",
            db.get_table(Data::Text(String::from("users"))).unwrap()
        );
    }
}
