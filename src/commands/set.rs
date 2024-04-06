use derive_builder::Builder;

use crate::data_type::DataType;

#[derive(Clone, Copy)]
pub enum ExpirationTime {
    Seconds(u32),
    Milliseconds(u32),
    TimestampSeconds(u32),
    TimestampMilliseconds(u32),
    KeepTTL,
}

#[derive(Clone, Copy)]
pub enum SetMode {
    SetIfExists,
    SetIfNotExists,
}

#[derive(Default, Builder)]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct SetOptions {
    pub expiration_time: Option<ExpirationTime>,
    pub set_mode: Option<SetMode>,
    pub get_previous_value: Option<bool>,
}

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

        if let Some(true) = &self.options.get_previous_value {
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
