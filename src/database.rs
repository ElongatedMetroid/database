use crate::table::{Table, Data};

pub struct Database {
    tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
        }
    }

    pub fn add_table(&mut self, name: Data, attributes: Vec<Data>) -> &mut Table {
        self.tables.push(Table::new(name, attributes));
        let element = self.tables.len() - 1;
        &mut self.tables[element]
    }
}