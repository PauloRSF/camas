use derive_builder::Builder;

use crate::data_type::DataType;

#[derive(Clone, Copy)]
pub enum ExpirationTime {
    Seconds(u64),
    Milliseconds(u64),
    TimestampSeconds(u64),
    TimestampMilliseconds(u64),
    KeepTTL,
}

#[derive(Clone, Copy)]
pub enum SetMode {
    SetIfExists,
    SetIfNotExists,
}

#[derive(Default, Builder, Clone, Copy)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct SetOptions {
    pub expiration_time: Option<ExpirationTime>,
    pub set_mode: Option<SetMode>,
    pub get_previous_value: bool,
}

#[derive(Clone)]
pub struct SetArguments {
    key: String,
    value: String,
    options: SetOptions,
}

impl SetArguments {
    pub fn new<K, V>(key: K, value: V, options: SetOptions) -> Self
    where
        K: ToString,
        V: ToString,
    {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            options,
        }
    }

    pub fn serialize(&self) -> String {
        let mut arguments = vec![
            DataType::BulkString(String::from("SET")),
            DataType::BulkString(self.key.clone()),
            DataType::BulkString(self.value.clone()),
        ];

        if let Some(set_mode) = &self.options.set_mode {
            match set_mode {
                SetMode::SetIfExists => {
                    arguments.push(DataType::BulkString(String::from("XX")));
                }
                SetMode::SetIfNotExists => {
                    arguments.push(DataType::BulkString(String::from("NX")));
                }
            }
        }

        if self.options.get_previous_value {
            arguments.push(DataType::BulkString(String::from("GET")));
        }

        if let Some(expiration_time) = &self.options.expiration_time {
            match expiration_time {
                ExpirationTime::Seconds(seconds) => {
                    arguments.push(DataType::BulkString(String::from("EX")));
                    arguments.push(DataType::BulkString((*seconds).to_string()));
                }
                ExpirationTime::Milliseconds(milliseconds) => {
                    arguments.push(DataType::BulkString(String::from("PX")));
                    arguments.push(DataType::BulkString((*milliseconds).to_string()));
                }
                ExpirationTime::TimestampSeconds(seconds) => {
                    arguments.push(DataType::BulkString(String::from("EXAT")));
                    arguments.push(DataType::BulkString((*seconds).to_string()));
                }
                ExpirationTime::TimestampMilliseconds(milliseconds) => {
                    arguments.push(DataType::BulkString(String::from("PXAT")));
                    arguments.push(DataType::BulkString((*milliseconds).to_string()));
                }
                ExpirationTime::KeepTTL => {
                    arguments.push(DataType::BulkString(String::from("KEEPTTL")));
                }
            }
        }

        DataType::Array(arguments).serialize()
    }
}

#[derive(Debug)]
pub enum SetResponse {
    Ok,
    Aborted,
    PreviousValue(Option<DataType>),
}

impl SetResponse {
    pub fn parse(arguments: &SetArguments, response: &DataType) -> Self {
        if arguments.options.get_previous_value {
            return match response {
                DataType::Null => SetResponse::PreviousValue(None),
                value => SetResponse::PreviousValue(Some(value.clone())),
            };
        }

        if arguments.options.set_mode.is_some() {
            if let DataType::Null = response {
                return SetResponse::Aborted;
            }
        }

        if let DataType::SimpleString(string) = response {
            if string == "OK" {
                return SetResponse::Ok;
            }
        }

        unreachable!("Redis should never return something different here")
    }
}
    }
}
