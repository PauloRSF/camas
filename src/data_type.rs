use crate::protocol::ProtocolDataType;

use std::fmt::Display;

/// A user-facing Redis data type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    String(String),
    List(Vec<String>),
}

impl Into<ProtocolDataType> for DataType {
    fn into(self) -> ProtocolDataType {
        match self {
            DataType::String(string) => ProtocolDataType::BulkString(string),
            DataType::List(list) => ProtocolDataType::Array(
                list.iter()
                    .cloned()
                    .map(ProtocolDataType::BulkString)
                    .collect(),
            ),
        }
    }
}

impl TryFrom<ProtocolDataType> for DataType {
    type Error = String;

    fn try_from(value: ProtocolDataType) -> Result<Self, Self::Error> {
        match value {
            ProtocolDataType::Double(double) => Ok(Self::String(double.to_string())),
            ProtocolDataType::Boolean(boolean) => Ok(Self::String(boolean.to_string())),
            ProtocolDataType::Integer(integer) => Ok(Self::String(integer.to_string())),
            ProtocolDataType::BigNumber(number) => Ok(Self::String(number.to_string())),
            ProtocolDataType::BulkString(string) => Ok(Self::String(string.to_string())),
            ProtocolDataType::SimpleString(string) => Ok(Self::String(string.to_string())),
            ProtocolDataType::Array(items) => Ok(Self::List(
                items
                    .iter()
                    .cloned()
                    .map(|item| DataType::try_from(item).unwrap().to_string())
                    .collect(),
            )),
            _ => Err("sei la".into()),
        }
    }
}

impl TryFrom<&ProtocolDataType> for DataType {
    type Error = String;

    fn try_from(value: &ProtocolDataType) -> Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::String(string) => f.write_fmt(format_args!("\"{}\"", string)),
            DataType::List(list) => {
                let items = list
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                f.write_fmt(format_args!("[{}]", items))
            }
        }
    }
}
