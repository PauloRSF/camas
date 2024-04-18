use std::error::Error;

use camas::{
    client::Client,
    commands::set::{ExpirationTime, SetMode, SetOptionsBuilder},
};

pub fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // let mut client = Client::connect("cache-32stns.serverless.use1.cache.amazonaws.com:6379")?;
    let mut client = Client::connect("localhost:6379")?;

    let set_options = SetOptionsBuilder::default()
        .expiration_time(ExpirationTime::Seconds(25))
        .set_mode(SetMode::SetIfNotExists)
        .build()?;

    client.set("foo", "321", set_options)?;

    let value = client.get("foo")?;

    println!("Value: {value:?}");

    let asd = ["foo", "qux", "bar"];

    client.del(&asd)?;

    Ok(())
}
