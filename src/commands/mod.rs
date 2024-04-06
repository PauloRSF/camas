use self::{del::DelArguments, get::GetArguments, set::SetArguments};

pub mod del;
pub mod get;
pub mod set;

pub enum Command {
    Set(SetArguments),
    Get(GetArguments),
    Del(DelArguments),
}

impl Command {
    pub fn serialize(&self) -> String {
        match self {
            Command::Set(arguments) => arguments.serialize(),
            Command::Get(arguments) => arguments.serialize(),
            Command::Del(arguments) => arguments.serialize(),
        }
    }
}
