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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_arguments_serializes() {
        let result = GetArguments::new("foo").serialize();

        assert_eq!(result, "*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n");
    }
}
