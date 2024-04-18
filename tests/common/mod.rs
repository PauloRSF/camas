use std::error::Error;

use camas::client::Client;

pub fn setup() -> Result<Client, Box<dyn Error>> {
    Ok(Client::connect("localhost:6379")?)
}

pub fn teardown(mut client: Client) -> Result<(), Box<dyn Error>> {
    client.flushdb(false)?;

    Ok(())
}
