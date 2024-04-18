use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while},
    character::{
        complete::{char, crlf},
        is_digit,
    },
    combinator::map,
    error::VerboseError,
    multi::many_m_n,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use super::ProtocolDataType;

fn bulk_string_with_content(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    let (rest, count) = map(
        preceded(char('$'), take_while(|a: char| is_digit(a as u8))),
        |value| u32::from_str(value).unwrap(),
    )(input)?;

    map(delimited(crlf, take(count), crlf), |value: &str| {
        ProtocolDataType::BulkString(value.to_string())
    })(rest)
}

fn bulk_string_nil(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag("$-1"), crlf)), |_| ProtocolDataType::Null)(input)
}

fn bulk_string_empty(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag("$0"), crlf)), |_| {
        ProtocolDataType::BulkString(String::new())
    })(input)
}

fn bulk_string(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    alt((bulk_string_nil, bulk_string_empty, bulk_string_with_content))(input)
}

fn simple_string(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(
        delimited(char('+'), take_until("\r\n"), crlf),
        |text: &str| ProtocolDataType::SimpleString(text.to_string()),
    )(input)
}

fn simple_error(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(
        delimited(char('-'), take_until("\r\n"), crlf),
        |text: &str| ProtocolDataType::SimpleError(text.to_string()),
    )(input)
}

fn integer(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(
        delimited(char(':'), take_until("\r\n"), crlf),
        |integer_str: &str| ProtocolDataType::Integer(integer_str.parse().unwrap()),
    )(input)
}

fn array_empty(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag("*0"), crlf)), |_| {
        ProtocolDataType::Array(Vec::new())
    })(input)
}

fn array_with_elements(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    let (rest, count) = map(
        delimited(char('*'), take_while(|a: char| is_digit(a as u8)), crlf),
        |value| usize::from_str(value).unwrap(),
    )(input)?;

    map(many_m_n(count, count, data_type), |elements| {
        ProtocolDataType::Array(elements)
    })(rest)
}

fn array(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    alt((array_empty, array_with_elements))(input)
}

fn boolean_true(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag("#t"), crlf)), |_| {
        ProtocolDataType::Boolean(true)
    })(input)
}

fn boolean_false(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag("#f"), crlf)), |_| {
        ProtocolDataType::Boolean(false)
    })(input)
}

fn boolean(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    alt((boolean_true, boolean_false))(input)
}

fn double_infinity(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag(",inf"), crlf)), |_| {
        ProtocolDataType::Double(f64::INFINITY)
    })(input)
}

fn double_negative_infinity(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag(",-inf"), crlf)), |_| {
        ProtocolDataType::Double(f64::NEG_INFINITY)
    })(input)
}
fn double_not_a_number(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag(",nan"), crlf)), |_| {
        ProtocolDataType::Double(f64::NAN)
    })(input)
}

fn double_number(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(
        delimited(char(','), take_until("\r\n"), crlf),
        |double_str: &str| ProtocolDataType::Double(double_str.parse().unwrap()),
    )(input)
}

fn double(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    alt((
        double_infinity,
        double_negative_infinity,
        double_not_a_number,
        double_number,
    ))(input)
}

fn null(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((char('_'), crlf)), |_| ProtocolDataType::Null)(input)
}

fn big_number(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(
        delimited(char('('), take_until("\r\n"), crlf),
        |number_str: &str| ProtocolDataType::BigNumber(number_str.parse().unwrap()),
    )(input)
}

fn bulk_error_empty(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    map(tuple((tag("!0"), crlf)), |_| {
        ProtocolDataType::BulkError(String::new())
    })(input)
}

fn bulk_error_with_content(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    let (rest, count) = map(
        preceded(char('!'), take_while(|a: char| is_digit(a as u8))),
        |value| u32::from_str(value).unwrap(),
    )(input)?;

    map(delimited(crlf, take(count), crlf), |value: &str| {
        ProtocolDataType::BulkError(value.to_string())
    })(rest)
}

fn bulk_error(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    alt((bulk_error_empty, bulk_error_with_content))(input)
}

pub fn data_type(input: &str) -> IResult<&str, ProtocolDataType, VerboseError<&str>> {
    alt((
        simple_string,
        simple_error,
        bulk_string,
        bulk_error,
        big_number,
        integer,
        boolean,
        double,
        array,
        null,
    ))(input)
}
