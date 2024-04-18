use crate::protocol::ProtocolDataType;

use super::{CommandArguments, ProtocolCommandArguments};

pub(crate) struct FlushDbArguments {
    async_flush: bool,
}

impl FlushDbArguments {
    pub fn new(async_flush: bool) -> Self {
        Self { async_flush }
    }
}

impl CommandArguments for FlushDbArguments {
    fn to_protocol_arguments(&self) -> ProtocolCommandArguments {
        if self.async_flush {
            vec![ProtocolDataType::BulkString(String::from("ASYNC"))]
        } else {
            vec![ProtocolDataType::BulkString(String::from("SYNC"))]
        }
    }
}

#[cfg(test)]
mod protocol_arguments {
    use super::*;

    #[test]
    fn builds_in_sync_mode() {
        let result = FlushDbArguments::new(false).to_protocol_arguments();

        assert_eq!(result, vec![ProtocolDataType::BulkString("SYNC".into())]);
    }

    #[test]
    fn builds_in_async_mode() {
        let result = FlushDbArguments::new(true).to_protocol_arguments();

        assert_eq!(result, vec![ProtocolDataType::BulkString("ASYNC".into())]);
    }
}
