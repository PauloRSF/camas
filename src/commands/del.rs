use crate::data_type::DataType;

pub struct DelArguments {
    keys: Vec<String>,
}

impl DelArguments {
    pub fn new<K: ToString>(keys: Vec<K>) -> Self {
        Self {
            keys: keys.iter().map(|item| item.to_string()).collect(),
        }
    }

    pub fn serialize(&self) -> String {
        let mut arguments = vec![DataType::BulkString(String::from("DEL"))];

        let keys = self
            .keys
            .iter()
            .cloned()
            .map(|item| DataType::BulkString(item));

        arguments.extend(keys);

        DataType::Array(arguments).serialize()
    }
}
