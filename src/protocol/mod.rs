use std::{cmp::Ordering, error::Error, fmt::Display, str::FromStr};

use num_bigint::BigInt;

mod parser;

/// A Redis data type
#[derive(Clone, Debug)]
pub enum ProtocolDataType {
    Null,
    Double(f64),
    Boolean(bool),
    Integer(i64),
    BigNumber(BigInt),
    BulkError(String),
    BulkString(String),
    SimpleError(String),
    SimpleString(String),
    Array(Vec<ProtocolDataType>),
    // Map(HashMap<ProtocolDataType, ProtocolDataType>),
}

impl PartialEq for ProtocolDataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ProtocolDataType::Null, ProtocolDataType::Null) => true,
            (ProtocolDataType::Double(lhs), ProtocolDataType::Double(rhs)) => {
                if lhs.is_nan() == rhs.is_nan() {
                    return true;
                }

                lhs.partial_cmp(rhs)
                    .map(|ord| ord == Ordering::Equal)
                    .unwrap_or(false)
            }
            (ProtocolDataType::Boolean(lhs), ProtocolDataType::Boolean(rhs)) => lhs == rhs,
            (ProtocolDataType::Integer(lhs), ProtocolDataType::Integer(rhs)) => lhs == rhs,
            (ProtocolDataType::BigNumber(lhs), ProtocolDataType::BigNumber(rhs)) => lhs == rhs,
            (ProtocolDataType::BulkError(lhs), ProtocolDataType::BulkError(rhs)) => lhs == rhs,
            (ProtocolDataType::BulkString(lhs), ProtocolDataType::BulkString(rhs)) => lhs == rhs,
            (ProtocolDataType::SimpleError(lhs), ProtocolDataType::SimpleError(rhs)) => lhs == rhs,
            (ProtocolDataType::SimpleString(lhs), ProtocolDataType::SimpleString(rhs)) => {
                lhs == rhs
            }
            (ProtocolDataType::Array(lhs), ProtocolDataType::Array(rhs)) => lhs.eq(rhs),
            _ => false,
        }
    }
}

impl ProtocolDataType {
    pub(crate) fn serialize(&self) -> String {
        match self {
            ProtocolDataType::Array(array) => {
                if array.is_empty() {
                    return String::from("*0\r\n");
                }

                let elements = array
                    .iter()
                    .map(|item| item.serialize())
                    .collect::<String>();

                format!("*{}\r\n{}", array.len(), elements)
            }
            ProtocolDataType::BulkString(string) => {
                if string.is_empty() {
                    return String::from("$0\r\n");
                }

                format!("${}\r\n{}\r\n", string.len(), string)
            }
            ProtocolDataType::Integer(integer) => {
                format!(":{}\r\n", integer)
            }
            ProtocolDataType::SimpleString(string) => {
                format!("+{}\r\n", string)
            }
            ProtocolDataType::SimpleError(error) => {
                format!("-{}\r\n", error)
            }
            ProtocolDataType::Null => String::from("_\r\n"),
            ProtocolDataType::Boolean(boolean) => {
                format!("#{}\r\n", if *boolean { 't' } else { 'f' })
            }
            ProtocolDataType::Double(double) => {
                if double.is_nan() {
                    return String::from(",nan\r\n");
                }

                format!(",{}\r\n", double)
            }
            ProtocolDataType::BigNumber(number) => {
                format!("({}\r\n", number)
            }
            // ProtocolDataType::Map(map) => {
            //     let elements = map
            //         .iter()
            //         .map(|(key, value)| format!("{}{}", key.serialize(), value.serialize()))
            //         .collect::<String>();

            //     format!("%{}\r\n{}\r\n", map.len(), elements)
            // }
            ProtocolDataType::BulkError(error) => {
                format!("!{}\r\n{}\r\n", error.len(), error)
            }
        }
    }
}

impl Display for ProtocolDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolDataType::Null => f.write_str("null"),
            ProtocolDataType::BulkString(string) => f.write_fmt(format_args!("\"{}\"", string)),
            ProtocolDataType::Integer(integer) => f.write_str(integer.to_string().as_str()),
            ProtocolDataType::SimpleString(string) => f.write_str(string.to_string().as_str()),
            ProtocolDataType::SimpleError(error) => f.write_str(error.to_string().as_str()),
            ProtocolDataType::Boolean(boolean) => f.write_str(boolean.to_string().as_str()),
            ProtocolDataType::Double(double) => f.write_str(double.to_string().as_str()),
            ProtocolDataType::BigNumber(number) => f.write_str(number.to_string().as_str()),
            ProtocolDataType::BulkError(error) => f.write_str(error.to_string().as_str()),
            ProtocolDataType::Array(array) => {
                let items = array
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                f.write_fmt(format_args!("[{}]", items))
            } // ProtocolDataType::Map(map) => {
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

impl FromStr for ProtocolDataType {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match parser::data_type(value) {
            Ok((_, data_type)) => Ok(data_type),
            Err(err) => {
                eprintln!("{err}");
                Err("Parsing error".into())
            }
        }
    }
}

impl From<&str> for ProtocolDataType {
    fn from(value: &str) -> Self {
        ProtocolDataType::BulkString(value.to_string())
    }
}

impl From<i64> for ProtocolDataType {
    fn from(value: i64) -> Self {
        ProtocolDataType::Integer(value)
    }
}

#[cfg(test)]
mod serialization {
    use super::*;

    #[test]
    fn serializes_null() {
        let result = ProtocolDataType::Null.serialize();

        assert_eq!(result, "_\r\n");
    }

    #[test]
    fn serializes_double_with_no_fractional_part() {
        let result = ProtocolDataType::Double(3_f64).serialize();

        assert_eq!(result, ",3\r\n");
    }

    #[test]
    fn serializes_double_with_fractional_part() {
        let result = ProtocolDataType::Double(3.141592).serialize();

        assert_eq!(result, ",3.141592\r\n");
    }

    #[test]
    fn serializes_double_with_infinity() {
        let result = ProtocolDataType::Double(f64::INFINITY).serialize();

        assert_eq!(result, ",inf\r\n");
    }

    #[test]
    fn serializes_double_with_negative_infinity() {
        let result = ProtocolDataType::Double(f64::NEG_INFINITY).serialize();

        assert_eq!(result, ",-inf\r\n");
    }

    #[test]
    fn serializes_double_with_not_a_number() {
        let result = ProtocolDataType::Double(f64::NAN).serialize();

        assert_eq!(result, ",nan\r\n");
    }

    #[test]
    fn serializes_boolean_true() {
        let result = ProtocolDataType::Boolean(true).serialize();

        assert_eq!(result, "#t\r\n");
    }

    #[test]
    fn serializes_boolean_false() {
        let result = ProtocolDataType::Boolean(false).serialize();

        assert_eq!(result, "#f\r\n");
    }

    #[test]
    fn serializes_positive_integer() {
        let result = ProtocolDataType::Integer(42).serialize();

        assert_eq!(result, ":42\r\n");
    }

    #[test]
    fn serializes_negative_integer() {
        let result = ProtocolDataType::Integer(-42).serialize();

        assert_eq!(result, ":-42\r\n");
    }

    #[test]
    fn serializes_positive_big_number() {
        let value = "298416298361318972639172639182763918263981267391826379128";

        let result = ProtocolDataType::BigNumber(BigInt::from_str(value).unwrap()).serialize();

        let expected = format!("({}\r\n", value);

        assert_eq!(result, expected);
    }

    #[test]
    fn serializes_negative_big_number() {
        let value = "-298416298361318972639172639182763918263981267391826379128";

        let result = ProtocolDataType::BigNumber(BigInt::from_str(value).unwrap()).serialize();

        let expected = format!("({}\r\n", value);

        assert_eq!(result, expected);
    }

    #[test]
    fn serializes_bulk_error() {
        let result = ProtocolDataType::BulkError("Some error".into()).serialize();

        assert_eq!(result, "!10\r\nSome error\r\n");
    }

    #[test]
    fn serializes_bulk_string() {
        let result = ProtocolDataType::BulkString("Some string".into()).serialize();

        assert_eq!(result, "$11\r\nSome string\r\n");
    }

    #[test]
    fn serializes_bulk_string_with_zero_length() {
        let result = ProtocolDataType::BulkString("".into()).serialize();

        assert_eq!(result, "$0\r\n");
    }

    #[test]
    fn serializes_simple_error() {
        let result = ProtocolDataType::SimpleError("ERR Some error".into()).serialize();

        assert_eq!(result, "-ERR Some error\r\n");
    }

    #[test]
    fn serializes_simple_string() {
        let result = ProtocolDataType::SimpleString("OK".into()).serialize();

        assert_eq!(result, "+OK\r\n");
    }

    #[test]
    fn serializes_array() {
        let result = ProtocolDataType::Array(vec![
            ProtocolDataType::BulkString("Foo".into()),
            ProtocolDataType::Integer(42),
            ProtocolDataType::Boolean(true),
        ])
        .serialize();

        assert_eq!(result, "*3\r\n$3\r\nFoo\r\n:42\r\n#t\r\n");
    }

    #[test]
    fn serializes_nested_array() {
        let result = ProtocolDataType::Array(vec![
            ProtocolDataType::BulkString("Foo".into()),
            ProtocolDataType::Array(vec![
                ProtocolDataType::Boolean(true),
                ProtocolDataType::Integer(42),
            ]),
        ])
        .serialize();

        assert_eq!(result, "*2\r\n$3\r\nFoo\r\n*2\r\n#t\r\n:42\r\n");
    }

    #[test]
    fn serializes_array_with_no_items() {
        let result = ProtocolDataType::Array(vec![]).serialize();

        assert_eq!(result, "*0\r\n");
    }
}

#[cfg(test)]
mod parsing {
    use super::*;

    #[test]
    fn parses_null() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Null;

        let result: ProtocolDataType = "_\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_unsigned_double_with_no_fractional_part() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(3.0);

        let result: ProtocolDataType = ",3\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_positive_double_with_no_fractional_part() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(3.0);

        let result: ProtocolDataType = ",+3\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_negative_double_with_no_fractional_part() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(3.0);

        let result: ProtocolDataType = ",-3\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_unsigned_double_with_fractional_part() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(3.141592);

        let result: ProtocolDataType = ",3.141592\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_positive_double_with_fractional_part() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(3.141592);

        let result: ProtocolDataType = ",+3.141592\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_negative_double_with_fractional_part() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(3.141592);

        let result: ProtocolDataType = ",-3.141592\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_double_with_infinity() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(f64::INFINITY);

        let result: ProtocolDataType = ",inf\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_double_with_negative_infinity() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(f64::NEG_INFINITY);

        let result: ProtocolDataType = ",-inf\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_double_with_not_a_number() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Double(f64::NAN);

        let result: ProtocolDataType = ",nan\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_boolean_true() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Boolean(true);

        let result: ProtocolDataType = "#t\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_boolean_false() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Boolean(false);

        let result: ProtocolDataType = "#f\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_unsigned_integer() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Integer(42);

        let result: ProtocolDataType = ":42\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_positive_integer() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Integer(42);

        let result: ProtocolDataType = ":+42\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_negative_integer() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Integer(-42);

        let result: ProtocolDataType = ":-42\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_positive_big_number() -> Result<(), Box<dyn Error>> {
        let number_str = "298416298361318972639172639182763918263981267391826379128";

        let expected = ProtocolDataType::BigNumber(BigInt::from_str(number_str)?);

        let result: ProtocolDataType = format!("({}\r\n", number_str).parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_negative_big_number() -> Result<(), Box<dyn Error>> {
        let number_str = "-298416298361318972639172639182763918263981267391826379128";

        let expected = ProtocolDataType::BigNumber(BigInt::from_str(number_str)?);

        let result: ProtocolDataType = format!("({}\r\n", number_str).parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_bulk_error() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::BulkError(String::from("Some error"));

        let result: ProtocolDataType = "!10\r\nSome error\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_bulk_string() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::BulkString(String::from("Some string"));

        let result: ProtocolDataType = "$11\r\nSome string\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_bulk_string_with_zero_length() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::BulkString(String::new());

        let result: ProtocolDataType = "$0\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_simple_error() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::SimpleError(String::from("ERR Some error"));

        let result: ProtocolDataType = "-ERR Some error\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_simple_string() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::SimpleString(String::from("OK"));

        let result: ProtocolDataType = "+OK\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_array() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Array(vec![
            ProtocolDataType::BulkString(String::from("Foo")),
            ProtocolDataType::Integer(42),
            ProtocolDataType::Boolean(true),
        ]);

        let result: ProtocolDataType = "*3\r\n$3\r\nFoo\r\n:42\r\n#t\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_array_with_no_items() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Array(Vec::new());

        let result: ProtocolDataType = "*0\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parses_nested_array() -> Result<(), Box<dyn Error>> {
        let expected = ProtocolDataType::Array(vec![
            ProtocolDataType::Array(vec![
                ProtocolDataType::Integer(1),
                ProtocolDataType::Integer(2),
                ProtocolDataType::Integer(3),
            ]),
            ProtocolDataType::Array(vec![
                ProtocolDataType::SimpleString(String::from("Hello")),
                ProtocolDataType::SimpleError(String::from("World")),
            ]),
        ]);

        let result: ProtocolDataType =
            "*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Hello\r\n-World\r\n".parse()?;

        assert_eq!(expected, result);

        Ok(())
    }
}
