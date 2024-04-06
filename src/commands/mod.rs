use self::{get::GetArguments, set::SetArguments};

pub mod get;
pub mod set;

pub enum Command {
    Set(SetArguments),
    Get(GetArguments),
}

impl Command {
    pub fn serialize(&self) -> String {
        match self {
            Command::Set(arguments) => arguments.serialize(),
            Command::Get(arguments) => arguments.serialize(),
        }
    }
}
