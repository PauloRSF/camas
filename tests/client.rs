use std::error::Error;

use camas::{
    commands::set::{SetOptions, SetResponse},
    data_type::DataType,
};

use crate::common::{setup, teardown};

mod common;

#[test]
fn set_with_default_options_returns_ok() -> Result<(), Box<dyn Error>> {
    let mut client = setup()?;

    let expected = SetResponse::Ok;

    let result = client.set("foo", "bar", Default::default())?;

    assert_eq!(expected, result);

    teardown(client)
}

#[test]
fn set_with_get_option_returns_previous_value_when_key_was_set() -> Result<(), Box<dyn Error>> {
    let mut client = setup()?;

    let mut options = SetOptions::default();

    options.get_previous_value = true;

    let result = client.set("foo", "bar", options)?;

    let expected = SetResponse::PreviousValue(Some(DataType::String("bar".into())));

    assert_eq!(expected, result);

    teardown(client)
}

#[test]
fn set_with_get_option_returns_no_previous_value_when_key_was_not_set() -> Result<(), Box<dyn Error>>
{
    let mut client = setup()?;

    let mut options = SetOptions::default();

    options.get_previous_value = true;

    let result = client.set("foo", "bar", options)?;

    let expected = SetResponse::PreviousValue(None);

    assert_eq!(expected, result);

    teardown(client)
}

#[test]
fn get_with_existent_key_returns_stored_value() -> Result<(), Box<dyn Error>> {
    let mut client = setup()?;

    client.set("foo", "bar", Default::default())?;

    let expected = Some(DataType::String("bar".into()));

    let result = client.get("foo")?;

    assert_eq!(expected, result);

    teardown(client)
}

#[test]
fn get_with_non_existent_key_returns_none() -> Result<(), Box<dyn Error>> {
    let mut client = setup()?;

    client.set("foo", "bar", Default::default())?;

    let result = client.get("foo")?;

    assert_eq!(None, result);

    teardown(client)
}
