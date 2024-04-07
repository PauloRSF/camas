mod parser;

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
                if array.is_empty() {
                    return String::from("*0\r\n");
                }

                let elements = array
                    .iter()
                    .map(|item| item.serialize())
                    .collect::<String>();

                format!("*{}\r\n{}", array.len(), elements)
            }
            DataType::BulkString(string) => {
                if string.is_empty() {
                    return String::from("$0\r\n");
                }

                format!("${}\r\n{}\r\n", string.len(), string)
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
            DataType::Null => String::from("_\r\n"),
            DataType::Boolean(boolean) => {
                format!("#{}\r\n", if *boolean { 't' } else { 'f' })
            }
            DataType::Double(double) => {
                if double.is_nan() {
                    return String::from(",nan\r\n");
                }

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
        match parser::data_type(value) {
            Ok((_, data_type)) => Ok(data_type),
            Err(err) => {
                eprintln!("{err}");
                Err("Parsing error".into())
            }
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

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn null_serializes() {
        let result = DataType::Null.serialize();

        assert_eq!(result, "_\r\n");
    }

    #[test]
    fn double_with_no_fractional_part_serializes() {
        let result = DataType::Double(3_f64).serialize();

        assert_eq!(result, ",3\r\n");
    }

    #[test]
    fn double_with_fractional_part_serializes() {
        let result = DataType::Double(3.141592).serialize();

        assert_eq!(result, ",3.141592\r\n");
    }

    #[test]
    fn double_with_infinity_serializes() {
        let result = DataType::Double(f64::INFINITY).serialize();

        assert_eq!(result, ",inf\r\n");
    }

    #[test]
    fn double_with_negative_infinity_serializes() {
        let result = DataType::Double(f64::NEG_INFINITY).serialize();

        assert_eq!(result, ",-inf\r\n");
    }

    #[test]
    fn double_with_not_a_number_serializes() {
        let result = DataType::Double(f64::NAN).serialize();

        assert_eq!(result, ",nan\r\n");
    }

    #[test]
    fn boolean_true_serializes() {
        let result = DataType::Boolean(true).serialize();

        assert_eq!(result, "#t\r\n");
    }

    #[test]
    fn boolean_false_serializes() {
        let result = DataType::Boolean(false).serialize();

        assert_eq!(result, "#f\r\n");
    }

    #[test]
    fn positive_integer_serializes() {
        let result = DataType::Integer(42).serialize();

        assert_eq!(result, ":42\r\n");
    }

    #[test]
    fn negative_integer_serializes() {
        let result = DataType::Integer(-42).serialize();

        assert_eq!(result, ":-42\r\n");
    }

    #[test]
    fn positive_big_number_serializes() {
        let value = "298416298361318972639172639182763918263981267391826379128";

        let result = DataType::BigNumber(BigInt::from_str(value).unwrap()).serialize();

        let expected = format!("({}\r\n", value);

        assert_eq!(result, expected);
    }

    #[test]
    fn negative_big_number_serializes() {
        let value = "-298416298361318972639172639182763918263981267391826379128";

        let result = DataType::BigNumber(BigInt::from_str(value).unwrap()).serialize();

        let expected = format!("({}\r\n", value);

        assert_eq!(result, expected);
    }

    #[test]
    fn bulk_error_serializes() {
        let result = DataType::BulkError("Some error".into()).serialize();

        assert_eq!(result, "!10\r\nSome error\r\n");
    }

    #[test]
    fn bulk_string_serializes() {
        let result = DataType::BulkString("Some string".into()).serialize();

        assert_eq!(result, "$11\r\nSome string\r\n");
    }

    #[test]
    fn bulk_string_with_zero_length_serializes() {
        let result = DataType::BulkString("".into()).serialize();

        assert_eq!(result, "$0\r\n");
    }

    #[test]
    fn simple_error_serializes() {
        let result = DataType::SimpleError("ERR Some error".into()).serialize();

        assert_eq!(result, "-ERR Some error\r\n");
    }

    #[test]
    fn simple_string_serializes() {
        let result = DataType::SimpleString("OK".into()).serialize();

        assert_eq!(result, "+OK\r\n");
    }

    #[test]
    fn array_serializes() {
        let result = DataType::Array(vec![
            DataType::BulkString("Foo".into()),
            DataType::Integer(42),
            DataType::Boolean(true),
        ])
        .serialize();

        assert_eq!(result, "*3\r\n$3\r\nFoo\r\n:42\r\n#t\r\n");
    }

    #[test]
    fn array_with_no_items_serializes() {
        let result = DataType::Array(vec![]).serialize();

        assert_eq!(result, "*0\r\n");
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::*;

    #[test]
    fn null_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "_\r\n".parse()?;

        assert!(matches!(result, DataType::Null));

        Ok(())
    }

    #[test]
    fn double_with_no_fractional_part_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = ",3\r\n".parse()?;

        match result {
            DataType::Double(number) if number == 3_f64 => Ok(()),
            _ => Err(format!("Expected \"Double(3)\", got {result}").into()),
        }
    }

    #[test]
    fn double_with_fractional_part_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = ",3.141592\r\n".parse()?;

        match result {
            DataType::Double(number) if number == 3.141592_f64 => Ok(()),
            _ => Err(format!("Expected \"Double(3.141592)\", got {result}").into()),
        }
    }

    #[test]
    fn double_with_infinity_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = ",inf\r\n".parse()?;

        match result {
            DataType::Double(number) if number.is_infinite() && number.is_sign_positive() => Ok(()),
            _ => Err(format!("Expected \"Double(INFINITY)\", got {result}").into()),
        }
    }

    #[test]
    fn double_with_negative_infinity_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = ",-inf\r\n".parse()?;

        match result {
            DataType::Double(number) if number.is_infinite() && number.is_sign_negative() => Ok(()),
            _ => Err(format!("Expected \"Double(NEG_INFINITY)\", got {result}").into()),
        }
    }

    #[test]
    fn double_with_not_a_number_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = ",nan\r\n".parse()?;

        match result {
            DataType::Double(number) if number.is_nan() => Ok(()),
            _ => Err(format!("Expected \"Double(NaN)\", got {result}").into()),
        }
    }

    #[test]
    fn boolean_true_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "#t\r\n".parse()?;

        assert!(matches!(result, DataType::Boolean(true)));

        Ok(())
    }

    #[test]
    fn boolean_false_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "#f\r\n".parse()?;

        assert!(matches!(result, DataType::Boolean(false)));

        Ok(())
    }

    #[test]
    fn positive_integer_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = ":42\r\n".parse()?;

        assert!(matches!(result, DataType::Integer(42)));

        Ok(())
    }

    #[test]
    fn negative_integer_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = ":-42\r\n".parse()?;

        assert!(matches!(result, DataType::Integer(-42)));

        Ok(())
    }

    #[test]
    fn positive_big_number_parses() -> Result<(), Box<dyn Error>> {
        let number_str = "298416298361318972639172639182763918263981267391826379128";
        let value = format!("({}\r\n", number_str);
        let result = value.parse()?;

        match result {
            DataType::BigNumber(number) if number == BigInt::from_str(number_str)? => Ok(()),
            _ => Err(format!("Expected \"BigNumber({value})\", got {result}").into()),
        }
    }

    #[test]
    fn negative_big_number_parses() -> Result<(), Box<dyn Error>> {
        let number_str = "-298416298361318972639172639182763918263981267391826379128";
        let value = format!("({}\r\n", number_str);
        let result = value.parse()?;

        match result {
            DataType::BigNumber(number) if number == BigInt::from_str(number_str)? => Ok(()),
            _ => Err(format!("Expected \"BigNumber({value})\", got {result}").into()),
        }
    }

    #[test]
    fn bulk_error_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "!10\r\nSome error\r\n".parse()?;

        match result {
            DataType::BulkError(error) if error == "Some error" => Ok(()),
            _ => Err(format!("Expected \"BulkError(Some error)\", got {result}").into()),
        }
    }

    #[test]
    fn bulk_string_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "$11\r\nSome string\r\n".parse()?;

        match result {
            DataType::BulkString(string) if string == "Some string" => Ok(()),
            _ => Err(format!("Expected \"BulkString(Some string)\", got {result}").into()),
        }
    }

    #[test]
    fn bulk_string_with_zero_length_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "$0\r\n".parse()?;

        match result {
            DataType::BulkString(string) if string == "" => Ok(()),
            _ => Err(format!("Expected \"BulkString(<empty string>)\", got {result}").into()),
        }
    }

    #[test]
    fn simple_error_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "-ERR Some error\r\n".parse()?;

        match result {
            DataType::SimpleError(error) if error == "ERR Some error" => Ok(()),
            _ => Err(format!("Expected \"SimpleError(ERR Some error)\", got {result}").into()),
        }
    }

    #[test]
    fn simple_string_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "+OK\r\n".parse()?;

        match result {
            DataType::SimpleString(string) if string == "OK" => Ok(()),
            _ => Err(format!("Expected \"SimpleString(OK)\", got {result}").into()),
        }
    }

    #[test]
    fn array_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "*3\r\n$3\r\nFoo\r\n:42\r\n#t\r\n".parse()?;

        if let DataType::Array(ref array) = result {
            if let [DataType::BulkString(string), DataType::Integer(42), DataType::Boolean(true)] =
                &array[..]
            {
                if string == "Foo" {
                    return Ok(());
                }
            }
        }

        Err(format!("Expected \"Array([Foo, 42, true])\", got {result}").into())
    }

    #[test]
    fn array_with_no_items_parses() -> Result<(), Box<dyn Error>> {
        let result: DataType = "*0\r\n".parse()?;

        match result {
            DataType::Array(array) if array.is_empty() => Ok(()),
            _ => Err(format!("Expected \"Array([])\", got {result}").into()),
        }
    }
}
