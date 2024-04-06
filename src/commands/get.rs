use crate::data_type::DataType;

pub struct GetArguments {
    key: String,
}

impl GetArguments {
    pub fn new<K: ToString>(key: K) -> Self {
        Self {
            key: key.to_string(),
        }
    }

    pub fn serialize(&self) -> String {
        DataType::Array(vec![
            DataType::BulkString(String::from("GET")),
            DataType::BulkString(self.key.clone()),
        ])
        .serialize()
    }
}
