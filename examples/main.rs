use std::error::Error;

use camas::{
    commands::set::{ExpirationTime, SetOptionsBuilder},
    Client,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect("localhost:6379")?;

    let set_options = SetOptionsBuilder::default()
        .expiration_time(ExpirationTime::Seconds(25))
        .build()?;

    client.set("foo", "234", set_options)?;

    let value = client.get("foo")?;

    println!("Value: {value}");

    client.del(vec!["foo"])?;

    Ok(())
}
