

fn main() {
    println!("Hello, world!");
}

mod comms {
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    #[derive(Debug, EnumIter)]
    #[derive(PartialEq)]
    pub enum DataType {
        SimpleString,
        Error,
        Integer,
        BulkString,
        Array,
    }

    impl DataType {
        pub fn value(&self) -> char {
            match self {
                DataType::SimpleString => '+',
                DataType::Error => '-',
                DataType::Integer => ':',
                DataType::BulkString => '$',
                DataType::Array => '*',
            }
        } 
    }

    fn is_null(message: &String) -> bool {
        message == "$-1\r\n" || message == "*-1\r\n"
    }

    pub fn get_message_data_type(message: &String) -> Option<DataType> {
        for data_type in DataType::iter() {
            if message.starts_with(data_type.value()) {
                return Some(data_type)
            }
        }
        None
    }

    pub fn ingest_message(message: &String) {
    
    }
}

#[cfg(test)]
mod tests {
    use crate::comms::ingest_message;
    use crate::comms::get_message_data_type;
    use crate::comms::DataType;

    #[test] 
    fn test_get_message_data_type() {
        assert_eq!(get_message_data_type(&String::from("+test")), Some(DataType::SimpleString));
        assert_eq!(get_message_data_type(&String::from("sedx")), None);
    }

    #[test] 
    fn test_ingest_message() {
        ingest_message(&String::from("hi"));
    }
}