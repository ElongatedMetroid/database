mod table;
mod database;

#[cfg(test)]
mod tests {
    use crate::{database::Database, table::Data};

    #[test]
    fn it_works() {
        let mut db = Database::new();

        println!("{:#?}", 
            db
                .add_table(Data::Text(String::from("Table")), 
                    vec![Data::Text(String::from("ID")), Data::Text(String::from("Name")), Data::Text(String::from("Birthdate"))])
                .push_row(vec![Some(Data::Integer(1)), None, None])
                .unwrap()
                .push_row(vec![Some(Data::Integer(1)), None, None])
                .unwrap()
        );

        println!("{:#?}", db.add_table(Data::Text(String::from("More_Data")), 
        vec![Data::Text(String::from("ID")), Data::Text(String::from("Name")), Data::Text(String::from("Birthdate"))]));
    }
}
