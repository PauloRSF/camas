use derive_builder::Builder;

use crate::{data_type::DataType, protocol::ProtocolDataType};

use super::{CommandArguments, ProtocolCommandArguments};

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
pub(crate) struct SetArguments {
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
}

impl CommandArguments for SetArguments {
    fn to_protocol_arguments(&self) -> ProtocolCommandArguments {
        let mut arguments = vec![
            ProtocolDataType::BulkString(self.key.clone()),
            ProtocolDataType::BulkString(self.value.clone()),
        ];

        if let Some(set_mode) = &self.options.set_mode {
            match set_mode {
                SetMode::SetIfExists => {
                    arguments.push(ProtocolDataType::BulkString("XX".into()));
                }
                SetMode::SetIfNotExists => {
                    arguments.push(ProtocolDataType::BulkString("NX".into()));
                }
            }
        }

        if self.options.get_previous_value {
            arguments.push(ProtocolDataType::BulkString("GET".into()));
        }

        if let Some(expiration_time) = &self.options.expiration_time {
            match expiration_time {
                ExpirationTime::Seconds(seconds) => {
                    arguments.push(ProtocolDataType::BulkString("EX".into()));
                    arguments.push(ProtocolDataType::BulkString((*seconds).to_string()));
                }
                ExpirationTime::Milliseconds(milliseconds) => {
                    arguments.push(ProtocolDataType::BulkString("PX".into()));
                    arguments.push(ProtocolDataType::BulkString((*milliseconds).to_string()));
                }
                ExpirationTime::TimestampSeconds(seconds) => {
                    arguments.push(ProtocolDataType::BulkString("EXAT".into()));
                    arguments.push(ProtocolDataType::BulkString((*seconds).to_string()));
                }
                ExpirationTime::TimestampMilliseconds(milliseconds) => {
                    arguments.push(ProtocolDataType::BulkString("PXAT".into()));
                    arguments.push(ProtocolDataType::BulkString((*milliseconds).to_string()));
                }
                ExpirationTime::KeepTTL => {
                    arguments.push(ProtocolDataType::BulkString("KEEPTTL".into()));
                }
            }
        }

        arguments
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SetResponse {
    Ok,
    Aborted,
    PreviousValue(Option<DataType>),
}

impl SetResponse {
    pub(crate) fn parse(arguments: &SetArguments, response: &ProtocolDataType) -> Self {
        if arguments.options.get_previous_value {
            return match response {
                ProtocolDataType::Null => SetResponse::PreviousValue(None),
                value => SetResponse::PreviousValue(Some(value.try_into().unwrap())),
            };
        }

        if arguments.options.set_mode.is_some() {
            if let ProtocolDataType::Null = response {
                return SetResponse::Aborted;
            }
        }

        if let ProtocolDataType::SimpleString(string) = response {
            if string == "OK" {
                return SetResponse::Ok;
            }
        }

        unreachable!("Redis should never return something different here")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn set_arguments_serializes_without_options() {
//         let result = SetArguments::new("foo", "bar", SetOptions::default()).serialize();

//         let expected = "*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n";

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_arguments_serializes_with_set_mode_xx() {
//         let mut set_options = SetOptions::default();

//         set_options.set_mode = Some(SetMode::SetIfExists);

//         let result = SetArguments::new("foo", "bar", set_options).serialize();

//         let expected = "*4\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nXX\r\n";

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_arguments_serializes_with_set_mode_nx() {
//         let mut set_options = SetOptions::default();

//         set_options.set_mode = Some(SetMode::SetIfNotExists);

//         let result = SetArguments::new("foo", "bar", set_options).serialize();

//         let expected = "*4\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nNX\r\n";

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_arguments_serializes_with_get() {
//         let mut set_options = SetOptions::default();

//         set_options.get_previous_value = true;

//         let result = SetArguments::new("foo", "bar", set_options).serialize();

//         let expected = "*4\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$3\r\nGET\r\n";

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_arguments_serializes_with_seconds_expiration_time() {
//         let mut set_options = SetOptions::default();

//         set_options.expiration_time = Some(ExpirationTime::Seconds(42));

//         let result = SetArguments::new("foo", "bar", set_options).serialize();

//         let expected = "*5\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nEX\r\n$2\r\n42\r\n";

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_arguments_serializes_with_milliseconds_expiration_time() {
//         let mut set_options = SetOptions::default();

//         set_options.expiration_time = Some(ExpirationTime::Milliseconds(42000));

//         let result = SetArguments::new("foo", "bar", set_options).serialize();

//         let expected = "*5\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nPX\r\n$5\r\n42000\r\n";

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_arguments_serializes_with_timestamp_seconds_expiration_time() {
//         let mut set_options = SetOptions::default();

//         set_options.expiration_time = Some(ExpirationTime::TimestampSeconds(1712451584));

//         let result = SetArguments::new("foo", "bar", set_options).serialize();

//         let expected =
//             "*5\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$4\r\nEXAT\r\n$10\r\n1712451584\r\n";

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_arguments_serializes_with_timestamp_milliseconds_expiration_time() {
//         let mut set_options = SetOptions::default();

//         set_options.expiration_time = Some(ExpirationTime::TimestampMilliseconds(1712451584000));

//         let result = SetArguments::new("foo", "bar", set_options).serialize();

//         let expected =
//             "*5\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$4\r\nPXAT\r\n$13\r\n1712451584000\r\n";

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_arguments_serializes_options_in_order() -> Result<(), SetOptionsBuilderError> {
//         let set_options = SetOptionsBuilder::default()
//             .get_previous_value(true)
//             .expiration_time(ExpirationTime::Seconds(42))
//             .set_mode(SetMode::SetIfExists)
//             .build()?;

//         let result = SetArguments::new("foo", "bar", set_options).serialize();

//         let expected = "*7\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nXX\r\n$3\r\nGET\r\n$2\r\nEX\r\n$2\r\n42\r\n";

//         assert_eq!(expected, result);

//         Ok(())
//     }

//     #[test]
//     fn set_response_parses_to_ok_when_get_is_not_given_and_got_ok() {
//         let arguments = SetArguments::new("foo", "bar", SetOptions::default());

//         let result = SetResponse::parse(&arguments, &ProtocolDataType::SimpleString("OK".into()));

//         let expected = SetResponse::Ok;

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_response_parses_to_empty_previous_value_when_get_is_given_and_got_null() {
//         let mut set_options = SetOptions::default();

//         set_options.get_previous_value = true;

//         let arguments = SetArguments::new("foo", "bar", set_options);

//         let result = SetResponse::parse(&arguments, &ProtocolDataType::Null);

//         let expected = SetResponse::PreviousValue(None);

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_response_parses_to_previous_value_when_get_is_given_and_got_some_value() {
//         let mut set_options = SetOptions::default();

//         set_options.get_previous_value = true;

//         let arguments = SetArguments::new("foo", "bar", set_options);

//         let result = SetResponse::parse(&arguments, &ProtocolDataType::BulkString("baz".into()));

//         let expected = SetResponse::PreviousValue(Some(DataType::String("baz".into())));

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_response_parses_to_aborted_when_get_is_not_given_and_nx_is_given_and_got_null() {
//         let mut set_options = SetOptions::default();

//         set_options.set_mode = Some(SetMode::SetIfNotExists);

//         let arguments = SetArguments::new("foo", "bar", set_options);

//         let result = SetResponse::parse(&arguments, &ProtocolDataType::Null);

//         let expected = SetResponse::Aborted;

//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn set_response_parses_to_aborted_when_get_is_not_given_and_xx_is_given_and_got_null() {
//         let mut set_options = SetOptions::default();

//         set_options.set_mode = Some(SetMode::SetIfExists);

//         let arguments = SetArguments::new("foo", "bar", set_options);

//         let result = SetResponse::parse(&arguments, &ProtocolDataType::Null);

//         let expected = SetResponse::Aborted;

//         assert_eq!(expected, result);
//     }
// }
