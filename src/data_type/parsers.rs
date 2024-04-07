use std::error::Error;

use super::DataType;

fn strip_element(value: &str, type_char: char) -> &str {
    value
        .strip_prefix(type_char)
        .unwrap()
        .strip_suffix("\r\n")
        .unwrap()
}

pub fn bulk_string(value: &str) -> Result<DataType, Box<dyn Error>> {
    let string = strip_element(value, '$');

    if string == "-1" {
        return Ok(DataType::Null);
    }

    let (_, text) = string.split_once("\r\n").unwrap();

    Ok(DataType::BulkString(text.to_string()))
}

pub fn simple_string(value: &str) -> Result<DataType, Box<dyn Error>> {
    let text = strip_element(value, '+').to_string();

    Ok(DataType::SimpleString(text))
}

pub fn simple_error(value: &str) -> Result<DataType, Box<dyn Error>> {
    let text = strip_element(value, '-').to_string();

    Ok(DataType::SimpleError(text))
}

pub fn integer(value: &str) -> Result<DataType, Box<dyn Error>> {
    let integer = strip_element(value, ':').parse::<i64>()?;

    Ok(DataType::Integer(integer))
}

pub fn array(value: &str) -> Result<DataType, Box<dyn Error>> {
    let (_, elements_str) = strip_element(value, '*').split_once("\r\n").unwrap();

    let elements = elements_str
        .split("\r\n")
        .map(|item| item.parse::<DataType>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DataType::Array(elements))
}

pub fn boolean(value: &str) -> Result<DataType, Box<dyn Error>> {
    match strip_element(value, '#') {
        "t" => Ok(DataType::Boolean(true)),
        "f" => Ok(DataType::Boolean(false)),
        _ => Err("sei la".into()),
    }
}

pub fn double(value: &str) -> Result<DataType, Box<dyn Error>> {
    match strip_element(value, ',') {
        "inf" => Ok(DataType::Double(f64::INFINITY)),
        "-inf" => Ok(DataType::Double(f64::NEG_INFINITY)),
        "nan" => Ok(DataType::Double(-f64::NAN)),
        double_str => Ok(DataType::Double(double_str.parse::<f64>()?)),
    }
}
