#[derive(Debug)]
pub struct Table {
    /// Name of the table
    name: Data,
    /// Attributes for each column
    attributes: Vec<Data>,
    /// 2D vector of Data's
    data: Vec<Vec<Option<Data>>>,
}

#[derive(Debug)]
pub enum Data {
    /// String of characters
    Text(String),
    /// Single character
    Character(char),
    /// Number without decimal values
    Integer(i64),
    /// Number with decimal value
    Decimal(f64),
    /// True or false value
    Boolean(bool),
    /// Vector of bytes
    Blob(Vec<u8>),
}

impl Table {
    pub(crate) fn new(name: Data, attributes: Vec<Data>) -> Self {
        Self {
            name,
            attributes,
            data: Vec::new(), 
        }
    }

    pub fn push_row(&mut self, row: Vec<Option<Data>>) -> Result<&mut Table, Box<dyn std::error::Error>> {
        if row.len() != self.attributes.len() {
            return Err(
                "Row is not the correct size to be pushed to this table".into()
            );
        }

        self.data.push(row);

        Ok(self)
    }
}