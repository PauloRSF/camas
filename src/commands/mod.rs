use crate::protocol::ProtocolDataType;

use self::{del::DelArguments, flushdb::FlushDbArguments, get::GetArguments, set::SetArguments};

pub(crate) mod del;
pub mod flushdb;
pub(crate) mod get;
pub mod set;

pub type ProtocolCommandArguments = Vec<ProtocolDataType>;

pub(super) trait CommandArguments {
    fn to_protocol_arguments(&self) -> ProtocolCommandArguments;
}

pub(crate) enum Command {
    Set(SetArguments),
    Get(GetArguments),
    Del(DelArguments),
    FlushDb(FlushDbArguments),
}

impl Command {
    pub fn command_name(&self) -> &str {
        match self {
            Command::Set(_) => "SET",
            Command::Get(_) => "GET",
            Command::Del(_) => "DEL",
            Command::FlushDb(_) => "FLUSHDB",
        }
    }

    pub fn argument_list(&self) -> ProtocolCommandArguments {
        match self {
            Command::Set(arguments) => arguments.to_protocol_arguments(),
            Command::Get(arguments) => arguments.to_protocol_arguments(),
            Command::Del(arguments) => arguments.to_protocol_arguments(),
            Command::FlushDb(arguments) => arguments.to_protocol_arguments(),
        }
    }

    pub fn serialize(&self) -> String {
        let mut arguments = Vec::new();

        arguments.push(ProtocolDataType::BulkString(self.command_name().into()));

        arguments.extend(self.argument_list());

        ProtocolDataType::Array(arguments).serialize()
    }
}
