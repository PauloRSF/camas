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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_arguments_serializes_without_options() {
        let result = SetArguments::new("foo", "bar", SetOptions::default()).serialize();

        assert_eq!(result, "*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
    }

    #[test]
    fn set_arguments_serializes_with_set_mode_xx() {
        let mut set_options = SetOptions::default();

        set_options.set_mode = Some(SetMode::SetIfExists);

        let result = SetArguments::new("foo", "bar", set_options).serialize();

        assert_eq!(
            result,
            "*4\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nXX\r\n"
        );
    }

    #[test]
    fn set_arguments_serializes_with_set_mode_nx() {
        let mut set_options = SetOptions::default();

        set_options.set_mode = Some(SetMode::SetIfNotExists);

        let result = SetArguments::new("foo", "bar", set_options).serialize();

        assert_eq!(
            result,
            "*4\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nNX\r\n"
        );
    }

    #[test]
    fn set_arguments_serializes_with_get() {
        let mut set_options = SetOptions::default();

        set_options.get_previous_value = true;

        let result = SetArguments::new("foo", "bar", set_options).serialize();

        assert_eq!(
            result,
            "*4\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$3\r\nGET\r\n"
        );
    }

    #[test]
    fn set_arguments_serializes_with_seconds_expiration_time() {
        let mut set_options = SetOptions::default();

        set_options.expiration_time = Some(ExpirationTime::Seconds(42));

        let result = SetArguments::new("foo", "bar", set_options).serialize();

        assert_eq!(
            result,
            "*5\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nEX\r\n$2\r\n42\r\n"
        );
    }

    #[test]
    fn set_arguments_serializes_with_milliseconds_expiration_time() {
        let mut set_options = SetOptions::default();

        set_options.expiration_time = Some(ExpirationTime::Milliseconds(42000));

        let result = SetArguments::new("foo", "bar", set_options).serialize();

        assert_eq!(
            result,
            "*5\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nPX\r\n$5\r\n42000\r\n"
        );
    }

    #[test]
    fn set_arguments_serializes_with_timestamp_seconds_expiration_time() {
        let mut set_options = SetOptions::default();

        set_options.expiration_time = Some(ExpirationTime::TimestampSeconds(1712451584));

        let result = SetArguments::new("foo", "bar", set_options).serialize();

        assert_eq!(
            result,
            "*5\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$4\r\nEXAT\r\n$10\r\n1712451584\r\n"
        );
    }

    #[test]
    fn set_arguments_serializes_with_timestamp_milliseconds_expiration_time() {
        let mut set_options = SetOptions::default();

        set_options.expiration_time = Some(ExpirationTime::TimestampMilliseconds(1712451584000));

        let result = SetArguments::new("foo", "bar", set_options).serialize();

        assert_eq!(
            result,
            "*5\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$4\r\nPXAT\r\n$13\r\n1712451584000\r\n"
        );
    }

    #[test]
    fn set_arguments_serializes_options_in_order() -> Result<(), SetOptionsBuilderError> {
        let set_options = SetOptionsBuilder::default()
            .get_previous_value(true)
            .expiration_time(ExpirationTime::Seconds(42))
            .set_mode(SetMode::SetIfExists)
            .build()?;

        let result = SetArguments::new("foo", "bar", set_options).serialize();

        assert_eq!(
            result,
            "*7\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$2\r\nXX\r\n$3\r\nGET\r\n$2\r\nEX\r\n$2\r\n42\r\n"
        );

        Ok(())
    }

    #[test]
    fn set_response_parses_to_ok_when_get_is_not_given_and_got_ok() {
        let arguments = SetArguments::new("foo", "bar", SetOptions::default());

        let result = SetResponse::parse(&arguments, &DataType::SimpleString("OK".into()));

        assert!(
            matches!(result, SetResponse::Ok),
            "Expected \"Ok\", got {:?}",
            result
        );
    }

    #[test]
    fn set_response_parses_to_empty_previous_value_when_get_is_given_and_got_null() {
        let mut set_options = SetOptions::default();

        set_options.get_previous_value = true;

        let arguments = SetArguments::new("foo", "bar", set_options);

        let result = SetResponse::parse(&arguments, &DataType::Null);

        assert!(
            matches!(result, SetResponse::PreviousValue(None)),
            "Expected \"PreviousValue(None)\", got {:?}",
            result
        );
    }

    #[test]
    fn set_response_parses_to_previous_value_when_get_is_given_and_got_some_value(
    ) -> Result<(), String> {
        let mut set_options = SetOptions::default();

        set_options.get_previous_value = true;

        let arguments = SetArguments::new("foo", "bar", set_options);

        let result = SetResponse::parse(&arguments, &DataType::BulkString("baz".into()));

        if let SetResponse::PreviousValue(Some(DataType::BulkString(value))) = result {
            assert_eq!(value, "baz");
            Ok(())
        } else {
            Err(format!(
                "Expected \"PreviousValue(Some(DataType::BulkString(\"baz\")))\", got {:?}",
                result
            ))
        }
    }

    #[test]
    fn set_response_parses_to_aborted_when_get_is_not_given_and_nx_is_given_and_got_null() {
        let mut set_options = SetOptions::default();

        set_options.set_mode = Some(SetMode::SetIfNotExists);

        let arguments = SetArguments::new("foo", "bar", set_options);

        let result = SetResponse::parse(&arguments, &DataType::Null);

        assert!(
            matches!(result, SetResponse::Aborted),
            "Expected \"Aborted\", got {:?}",
            result
        );
    }

    #[test]
    fn set_response_parses_to_aborted_when_get_is_not_given_and_xx_is_given_and_got_null() {
        let mut set_options = SetOptions::default();

        set_options.set_mode = Some(SetMode::SetIfExists);

        let arguments = SetArguments::new("foo", "bar", set_options);

        let result = SetResponse::parse(&arguments, &DataType::Null);

        assert!(
            matches!(result, SetResponse::Aborted),
            "Expected \"Aborted\", got {:?}",
            result
        );
    }
}
