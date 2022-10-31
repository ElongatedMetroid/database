# database
database written in rust

## Initial Plan/Idea

Create an easy to use an scalable database

Nmb's of the newest data will be stored inside memory, while older data will be written to storage

## Example

```Rust
use database::{database::Database, table::Data};

fn main() {
    // Create a new empty database
    let db = Database::new();

    // Add a table with the specified attributes to the databse
    db
        .add_table(vec![
            Data::Text(String::from("Name")), 
            Data::Text(String::from("Birthday")), 
            Data::Text(String::from("Money"))
        ])
        .unwrap();

    // Add a row to the table
    let row_id = table
        .push_row(vec![
            (
                Data::Text(String::from("Name")),
                Some(Data::Text(String::from("Bobdfdsafasdfsadfsadffasdsby"))),
            ),
            (
                Data::Text(String::from("Birthday")),
                Some(Data::Integer(1990)),
            ),
            (Data::Text(String::from("Money")), Some(Data::Integer(1))),
        ])
        .unwrap();

    // Modify a cell in that row
}
```