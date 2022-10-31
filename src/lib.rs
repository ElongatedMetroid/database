#![feature(exact_size_is_empty)]
mod database;
mod table;

#[cfg(test)]
mod tests {
    use crate::{database::Database, table::Data};

    #[test]
    fn bank_managment() {
        let mut db = Database::new();

        db
            .add_table(/*Mode::Memory, Mode::Storage, Mode::Smart (write all to storage but keep newest data in mem) */ Data::Text(String::from("Users")),
        vec![Data::Text(String::from("Name")), Data::Text(String::from("Birthday")), Data::Text(String::from("Money"))]).unwrap();

        // Add new users
        let table = db.get_table(Data::Text(String::from("Users"))).unwrap();

        println!("{}\n\n", table);

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

            table
            .push_row(vec![
                (
                    Data::Text(String::from("Name")),
                    Some(Data::Text(String::from("Timmy"))),
                ),
                (
                    Data::Text(String::from("Birthday")),
                    Some(Data::Integer(2000)),
                ),
                (Data::Text(String::from("Money")), Some(Data::Integer(10000))),
            ])
            .unwrap();

        println!("{}\n\n", table);

        // Users desposit money
        match table
            .get_cell(&Data::Text(String::from("Money")), row_id)
            .unwrap()
        {
            Some(money) => match money {
                Data::Integer(d) => *d += 1,
                _ => (),
            },
            None => (),
        }

        println!("{}\n\n", table);
    }
}
