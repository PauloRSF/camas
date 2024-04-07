use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use commands::{
    del::DelArguments,
    get::GetArguments,
    set::{SetArguments, SetOptions, SetResponse},
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

        self.stream.write_all(serialized_command.as_bytes())?;

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
    ) -> Result<SetResponse, Box<dyn Error>>
    where
        K: ToString,
        V: ToString,
    {
        let arguments = SetArguments::new(key, value, options);
        let command = Command::Set(arguments.clone());

        let response = self.execute(&command)?;

        Ok(SetResponse::parse(&arguments, &response))
    }

    pub fn get<K: ToString>(&mut self, key: K) -> Result<DataType, Box<dyn Error>> {
        let command = Command::Get(GetArguments::new(key));

        self.execute(&command)
    }

    pub fn del<K: ToString>(&mut self, keys: Vec<K>) -> Result<u32, Box<dyn Error>> {
        let command = Command::Del(DelArguments::new(keys));

        let response = self.execute(&command)?;

        match response {
            DataType::Integer(deleted_key_count) => Ok(deleted_key_count as u32),
            _ => Err("sei la".into()),
        }
    }
}
