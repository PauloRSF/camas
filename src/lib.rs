use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use commands::{
    del::DelArguments,
    get::GetArguments,
    set::{SetArguments, SetOptions},
    Command,
};
use data_type::DataType;
use log::log;
use termcolor::Color;

pub mod commands;
mod data_type;
mod log;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect<A: ToSocketAddrs>(address: A) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(address)?;

        Ok(Self { stream })
    }

    fn log_send(&self, message: &String) -> Result<(), Box<dyn Error>> {
        log("SENT", message, Color::Green)
    }

    fn log_receive(&self, message: &String) -> Result<(), Box<dyn Error>> {
        log("RECEIVED", message, Color::Blue)
    }

    fn execute(&mut self, command: &Command) -> Result<DataType, Box<dyn Error>> {
        let serialized_command = command.serialize();

        self.log_send(&serialized_command)?;

        self.stream.write(serialized_command.as_bytes())?;

        let mut buf = [0u8; 1024];

        let bytes_read = self.stream.read(&mut buf)?;

        let response = String::from_utf8(buf[..bytes_read].to_vec())?;

        self.log_receive(&response)?;

        response.parse::<DataType>()
    }

    pub fn set<K, V>(
        &mut self,
        key: K,
        value: V,
        options: SetOptions,
    ) -> Result<DataType, Box<dyn Error>>
    where
        K: ToString,
        V: ToString,
    {
        let command = Command::Set(SetArguments::new(key, value, options));

        self.execute(&command)
    }

    pub fn get<K: ToString>(&mut self, key: K) -> Result<DataType, Box<dyn Error>> {
        let command = Command::Get(GetArguments::new(key));

        self.execute(&command)
    }

    pub fn del<K: ToString>(&mut self, keys: Vec<K>) -> Result<DataType, Box<dyn Error>> {
        let command = Command::Del(DelArguments::new(keys));

        self.execute(&command)
    }
}
