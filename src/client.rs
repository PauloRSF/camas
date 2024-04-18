use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::{
    commands::{
        del::DelArguments,
        flushdb::FlushDbArguments,
        get::GetArguments,
        set::{SetArguments, SetOptions, SetResponse},
        Command,
    },
    data_type::DataType,
    debug::log,
    protocol::ProtocolDataType,
};

const CLIENT_RECEIVE_BUFFER_SIZE: usize = 1024;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    /// Connects to a Redis instance and returns a connected `Client` ready
    /// to send commands.
    pub fn connect<A: ToSocketAddrs>(address: A) -> std::io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Ok(Self { stream })
    }

    /// Serializes a command, sends it to Redis and parses the response
    fn execute(&mut self, command: &Command) -> Result<ProtocolDataType, Box<dyn Error>> {
        let serialized_command = command.serialize();

        log("SENT", &serialized_command)?;

        self.stream.write_all(serialized_command.as_bytes())?;

        let mut response = String::new();

        loop {
            let mut buf = [0u8; CLIENT_RECEIVE_BUFFER_SIZE];

            let bytes_read = self.stream.read(&mut buf)?;

            response.push_str(&String::from_utf8_lossy(&buf[..bytes_read]));

            log("RECEIVED", &response)?;

            if bytes_read < CLIENT_RECEIVE_BUFFER_SIZE {
                break;
            }
        }

        match response.parse::<ProtocolDataType>()? {
            ProtocolDataType::SimpleError(error) | ProtocolDataType::BulkError(error) => {
                Err(error.into())
            }
            parsed_response => Ok(parsed_response),
        }
    }

    /// Sets a value for a key.
    ///
    /// # Example
    ///
    ///
    /// # use std::error::Error;
    /// use camas::{client::Client, data_type::DataType};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let mut client = Client::connect("localhost:6379")?;
    ///
    /// let valueToStore = DataType::Integer(123);
    ///
    /// let response = client.set("foo", valueToStore, Default::default())?;
    ///
    /// assert_eq!(response, Some(SetResponse::Ok));
    ///
    /// let storedValue = client.get("foo")?;
    ///
    /// assert_eq!(storedValue, Some(value));
    /// # Ok(())
    /// # }
    /// ```
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

    /// Returns the value for a given key.
    ///
    /// The returned value can be any of the data types supported by Redis or
    /// `None`, if the key is not set.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::error::Error;
    /// use camas::{client::Client, data_type::DataType};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let mut client = Client::connect("localhost:6379")?;
    ///
    /// client.set("foo", "Hello", Default::default())?;
    ///
    /// assert_eq!(client.get("foo")?, Some(DataType::String(String::from("Hello"))));
    /// assert_eq!(client.get("non-existing-key")?, None);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get<K: ToString>(&mut self, key: K) -> Result<Option<DataType>, Box<dyn Error>> {
        let command = Command::Get(GetArguments::new(key));

        let response = self.execute(&command)?;

        if response == ProtocolDataType::Null {
            Ok(None)
        } else {
            Ok(Some(response.try_into()?))
        }
    }

    /// Removes the given keys.
    ///
    /// Returns the number of deleted keys. If some key wasn't previously set,
    /// it will be ignored.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::error::Error;
    /// use camas::{client::Client, data_type::DataType};
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let mut client = Client::connect("localhost:6379")?;
    ///
    /// client.set("foo", "Hello", Default::default())?;
    /// client.set("bar", "World", Default::default())?;
    ///
    /// let keys = ["foo", "qux", "bar"];
    ///
    /// let deleted_key_count = client.del(&keys)?;
    ///
    /// assert_eq!(deleted_key_count, 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn del<K: ToString + Clone>(&mut self, keys: &[K]) -> Result<u32, Box<dyn Error>> {
        let command = Command::Del(DelArguments::new(keys.to_vec()));

        let response = self.execute(&command)?;

        if let ProtocolDataType::Integer(deleted_key_count) = response {
            Ok(deleted_key_count as u32)
        } else {
            unreachable!("Redis should never return something different here")
        }
    }

    pub fn flushdb(&mut self, async_flush: bool) -> Result<(), Box<dyn Error>> {
        let command = Command::FlushDb(FlushDbArguments::new(async_flush));

        self.execute(&command)?;

        Ok(())
    }
}
