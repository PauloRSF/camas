use crate::protocol::ProtocolDataType;

use super::{CommandArguments, ProtocolCommandArguments};

pub(crate) struct DelArguments {
    keys: Vec<String>,
}

impl DelArguments {
    pub fn new<K: ToString>(keys: Vec<K>) -> Self {
        Self {
            keys: keys.iter().map(|item| item.to_string()).collect(),
        }
    }
}

impl CommandArguments for DelArguments {
    fn to_protocol_arguments(&self) -> ProtocolCommandArguments {
        self.keys
            .iter()
            .cloned()
            .map(ProtocolDataType::BulkString)
            .collect()
    }
}

#[cfg(test)]
mod protocol_arguments {
    use super::*;

    #[test]
    fn builds_correctly() {
        let result = DelArguments::new(vec!["foo", "bar", "baz"]).to_protocol_arguments();

        assert_eq!(
            result,
            vec![
                ProtocolDataType::BulkString("foo".into()),
                ProtocolDataType::BulkString("bar".into()),
                ProtocolDataType::BulkString("baz".into())
            ]
        );
    }
}
