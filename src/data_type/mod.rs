mod parsers;

use std::{error::Error, fmt::Display, str::FromStr};

use num_bigint::BigInt;

#[derive(Clone, Debug)]
pub enum DataType {
    Null,
    Double(f64),
    Boolean(bool),
    Integer(i64),
    BigNumber(BigInt),
    BulkError(String),
    BulkString(String),
    SimpleError(String),
    SimpleString(String),
    Array(Vec<DataType>),
    // Map(HashMap<DataType, DataType>),
}

impl DataType {
    pub fn serialize(&self) -> String {
        match self {
            DataType::Array(array) => {
                let elements = array
                    .iter()
                    .map(|item| item.serialize())
                    .collect::<Vec<_>>()
                    .join("\r\n");

                format!("*{}\r\n{}\r\n", array.len(), elements)
            }
            DataType::BulkString(string) => {
                format!("${}\r\n{}", string.len(), string)
            }
            DataType::Integer(integer) => {
                format!(":{}\r\n", integer)
            }
            DataType::SimpleString(string) => {
                format!("+{}\r\n", string)
            }
            DataType::SimpleError(error) => {
                format!("-{}\r\n", error)
            }
            DataType::Null => {
                format!("_\r\n")
            }
            DataType::Boolean(boolean) => {
                format!("#{}\r\n", if *boolean { 't' } else { 'f' })
            }
            DataType::Double(double) => {
                format!(",{}\r\n", double)
            }
            DataType::BigNumber(number) => {
                format!("({}\r\n", number)
            }
            // DataType::Map(map) => {
            //     let elements = map
            //         .iter()
            //         .map(|(key, value)| format!("{}{}", key.serialize(), value.serialize()))
            //         .collect::<String>();

            //     format!("%{}\r\n{}\r\n", map.len(), elements)
            // }
            DataType::BulkError(error) => {
                format!("!{}\r\n{}\r\n", error.len(), error)
            }
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Null => f.write_str("null"),
            DataType::BulkString(string) => f.write_fmt(format_args!("\"{}\"", string)),
            DataType::Integer(integer) => f.write_str(integer.to_string().as_str()),
            DataType::SimpleString(string) => f.write_str(string.to_string().as_str()),
            DataType::SimpleError(error) => f.write_str(error.to_string().as_str()),
            DataType::Boolean(boolean) => f.write_str(boolean.to_string().as_str()),
            DataType::Double(double) => f.write_str(double.to_string().as_str()),
            DataType::BigNumber(number) => f.write_str(number.to_string().as_str()),
            DataType::BulkError(error) => f.write_str(error.to_string().as_str()),
            DataType::Array(array) => {
                let items = array
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                f.write_fmt(format_args!("[{}]", items))
            } // DataType::Map(map) => {
              //     let elements = map
              //         .iter()
              //         .map(|(key, value)| format!("\t{}: {}", key.to_string(), value.to_string()))
              //         .collect::<Vec<String>>()
              //         .join("\n");

              //     f.write_fmt(format_args!("{{\n{}\n}}", elements))
              // }
        }
    }
}

impl FromStr for DataType {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.chars().nth(0) {
            Some('$') => parsers::bulk_string(value),
            Some('+') => parsers::simple_string(value),
            Some('-') => parsers::simple_error(value),
            Some(':') => parsers::integer(value),
            Some('*') => parsers::array(value),
            Some('#') => parsers::boolean(value),
            Some(',') => parsers::double(value),
            Some('_') => Ok(DataType::Null),
            _ => unimplemented!(),
        }
    }
}

impl From<&str> for DataType {
    fn from(value: &str) -> Self {
        DataType::BulkString(value.to_string())
    }
}

impl From<i64> for DataType {
    fn from(value: i64) -> Self {
        DataType::Integer(value)
    }
}
