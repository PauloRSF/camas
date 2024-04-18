use crate::protocol::ProtocolDataType;

use super::{CommandArguments, ProtocolCommandArguments};

pub(crate) struct GetArguments {
    key: String,
}

impl GetArguments {
    pub fn new<K: ToString>(key: K) -> Self {
        Self {
            key: key.to_string(),
        }
    }
}

impl CommandArguments for GetArguments {
    fn to_protocol_arguments(&self) -> ProtocolCommandArguments {
        vec![ProtocolDataType::BulkString(self.key.clone())]
    }
}

#[cfg(test)]
mod protocol_arguments {
    use super::*;

    #[test]
    fn builds_correctly() {
        let result = GetArguments::new("foo").to_protocol_arguments();

        assert_eq!(result, vec![ProtocolDataType::BulkString("foo".into()),]);
    }
}
