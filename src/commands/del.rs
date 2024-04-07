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
            .map(DataType::BulkString);

        arguments.extend(keys);

        DataType::Array(arguments).serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn del_arguments_serializes() {
        let result = DelArguments::new(vec!["foo", "bar", "baz"]).serialize();

        assert_eq!(
            result,
            "*4\r\n$3\r\nDEL\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$3\r\nbaz\r\n"
        );
    }
}
